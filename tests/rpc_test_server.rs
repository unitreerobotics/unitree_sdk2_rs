use std::sync::Arc;

use unitree_sdk2_rs::rpc::RpcServer;

#[tokio::main]
async fn main() {
    unitree_sdk2_rs::init_dds(0, "", "");

    let server = Arc::new(RpcServer::new("rpc_test").expect("failed to create RpcServer"));
    server.register_handler(1001, |req| format!(r#"{{"echo":"{}"}}"#, req));
    server.register_handler(1002, |_| r#"{"result":"ok"}"#.to_string());

    eprintln!("RPC server starting (rpc_test, api=1001,1002)");
    server.clone().start().await;
}
