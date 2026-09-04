# RPC (Server / Client)

## Server

Create a server with `RpcServer::new(service)`, register API handlers with `register_handler`,
then start listening with `start`:

```rust,no_run
use std::sync::Arc;
use unitree_sdk2_rs::rpc::RpcServer;

#[tokio::main]
async fn main() {
    unitree_sdk2_rs::init_dds(0, "eth0", "");

    let server = Arc::new(RpcServer::new("rpc_test").expect("failed to create RpcServer"));
    // The handler receives the request string; the string it returns is sent back as the response.
    server.register_handler(1001, |req| format!(r#"{{"echo":"{}"}}"#, req));
    server.register_handler(1002, |_| r#"{"result":"ok"}"#.to_string());

    eprintln!("RPC server starting (rpc_test, api=1001,1002)");
    server.clone().start().await;
}
```

## Client

Connect with `RpcClient::new(service)` (the service name must match the server), register the
APIs to call with `register_api`, then make calls with `call`:

```rust,no_run
use unitree_sdk2_rs::rpc::RpcClient;

#[tokio::main]
async fn main() {
    unitree_sdk2_rs::init_dds(0, "eth0", "");

    let client = RpcClient::new("rpc_test").expect("failed to create RpcClient");
    client.register_api(1001);
    client.register_api(1002);

    // Returns (code, data): code is the return code (0 = success), data is the response payload.
    let resp = client.call(1001, "hello".to_string()).await.expect("call 1001");
    println!("api=1001 code={} data={}", resp.code, resp.data);

    let resp = client.call(1002, String::new()).await.expect("call 1002");
    println!("api=1002 code={} data={}", resp.code, resp.data);
}
```

## Robot Service APIs

The robot's RPC services (`sport` / `audio` / `config` / `video` / `arm` / `loco` / ...)
are exposed as `robot_api::<model>::<service>` modules with `API_ID_*` constants and
serde request payloads, generated from the unitree_sdk2 headers:

```rust,no_run
use unitree_sdk2_rs::robot_api::go2::sport::SportClient;

#[tokio::main]
async fn main() {
    unitree_sdk2_rs::init_dds(0, "eth0", "");

    // mirrors the C++ client: registers all sport api ids on connect
    let client = SportClient::new().expect("failed to create SportClient");

    // {"vx":0.3,"vy":0.0,"vyaw":0.0}
    let resp = client.move_to(0.3, 0.0, 0.0).await.expect("RPC call failed");
    println!("code={} data={}", resp.code, resp.data);
}
```

All 8 robot models (`go2` / `g1` / `h1` / `h2` / `b2` / `a2` / `as2` / `r1`) are
covered; each model has one file under `gen/robot_api/` (`robot_api::<model>::<service>`),
hand-verified against the unitree_sdk2 headers.
