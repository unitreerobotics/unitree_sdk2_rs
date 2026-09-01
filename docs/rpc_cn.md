# RPC（服务端 / 客户端）

## 服务端

`RpcServer::new(service)` 创建服务端，`register_handler` 注册 API，`start` 启动监听：

```rust,no_run
use std::sync::Arc;
use unitree_sdk2_rs::rpc::RpcServer;

#[tokio::main]
async fn main() {
    unitree_sdk2_rs::init_dds(0, "eth0", "");

    let server = Arc::new(RpcServer::new("rpc_test").expect("创建 RpcServer 失败"));
    // handler 接收请求字符串，返回的字符串作为响应数据
    server.register_handler(1001, |req| format!(r#"{{"echo":"{}"}}"#, req));
    server.register_handler(1002, |_| r#"{"result":"ok"}"#.to_string());

    eprintln!("RPC 服务端启动中 (rpc_test, api=1001,1002)");
    server.clone().start().await;
}
```

## 客户端

`RpcClient::new(service)` 连接服务（服务名须与服务端一致），`register_api` 注册要调用的 API，`call` 发起调用：

```rust,no_run
use unitree_sdk2_rs::rpc::RpcClient;

#[tokio::main]
async fn main() {
    unitree_sdk2_rs::init_dds(0, "eth0", "");

    let client = RpcClient::new("rpc_test").expect("创建 RpcClient 失败");
    client.register_api(1001);
    client.register_api(1002);

    // 返回 (code, data)：code 为返回码（0 表示成功），data 为响应内容
    let resp = client.call(1001, "hello".to_string()).await.expect("call 1001");
    println!("api=1001 code={} data={}", resp.code, resp.data);

    let resp = client.call(1002, String::new()).await.expect("call 1002");
    println!("api=1002 code={} data={}", resp.code, resp.data);
}
```
