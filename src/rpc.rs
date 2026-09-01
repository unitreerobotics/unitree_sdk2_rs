use crate::{rpc_ffi, RpcRequestHandler};
use anyhow::Context;
use cxx::UniquePtr;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

unsafe impl Send for rpc_ffi::RpcClient {}
unsafe impl Sync for rpc_ffi::RpcClient {}
unsafe impl Send for rpc_ffi::RpcServer {}
unsafe impl Sync for rpc_ffi::RpcServer {}

/// RPC client: connects to a service, registers APIs and makes calls.
///
/// `service` is the service name; it must match the one used by the server
/// ([`RpcServer::new`]).
pub struct RpcClient {
    inner: Arc<UniquePtr<rpc_ffi::RpcClient>>,
}

impl RpcClient {
    /// Creates an RPC client (`service`: service name, must match the server).
    pub fn new(service: &str) -> anyhow::Result<Self> {
        let inner = rpc_ffi::new_rpc_client(service);
        if inner.is_null() {
            log::error!("new_rpc_client({service}) returned a null pointer");
            anyhow::bail!("failed to create RpcClient({service})");
        }
        Ok(Self {
            inner: Arc::new(inner),
        })
    }

    /// Registers an API ID to call (must be done before calling it).
    pub fn register_api(&self, api_id: i32) {
        self.inner.register_api(api_id);
    }

    /// Calls a server API.
    ///
    /// Executed on a blocking thread pool, so it does not block the current task.
    /// Returns the server response.
    pub async fn call(&self, api_id: i32, request: String) -> anyhow::Result<RpcResponse> {
        let inner = self.inner.clone();
        tokio::task::spawn_blocking(move || {
            let raw = inner.call(api_id, &request);
            if raw.is_empty() {
                log::warn!("RPC call({api_id}) returned an empty response");
            }
            serde_json::from_str(&raw).context("failed to parse RPC response")
        })
        .await
        .context("RPC spawn_blocking failed")?
    }
}

/// RPC call result: `code` is the server return code (0 = success),
/// `data` is the response payload.
#[derive(Debug, Deserialize, Serialize)]
pub struct RpcResponse {
    pub code: i32,
    #[serde(default)]
    pub data: String,
}

/// RPC server: registers a service name with API handlers, then starts listening.
pub struct RpcServer {
    inner: Arc<UniquePtr<rpc_ffi::RpcServer>>,
}

impl RpcServer {
    /// Creates an RPC server (`service`: service name, clients connect with the same name).
    pub fn new(service: &str) -> anyhow::Result<Self> {
        let inner = rpc_ffi::new_rpc_server(service);
        if inner.is_null() {
            log::error!("new_rpc_server({service}) returned a null pointer");
            anyhow::bail!("failed to create RpcServer({service})");
        }
        Ok(Self {
            inner: Arc::new(inner),
        })
    }

    /// Registers an API handler: when a client calls `api_id`, `handler` runs
    /// with the request string, and the string it returns is sent back as the response.
    pub fn register_handler(&self, api_id: i32, handler: impl Fn(&str) -> String + Send + 'static) {
        self.inner
            .register_handler(api_id, Box::new(RpcRequestHandler::new(handler)));
    }

    /// Starts the server (runs on a blocking thread pool, does not block the current task).
    pub async fn start(self: Arc<Self>) {
        if let Err(e) = tokio::task::spawn_blocking(move || self.inner.start()).await {
            log::error!("RPC server thread exited with an error: {e}");
        }
    }
}
