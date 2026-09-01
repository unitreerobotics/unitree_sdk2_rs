use std::time::Duration;

use unitree_sdk2_rs::rpc::RpcClient;

#[tokio::main]
async fn main() {
    unitree_sdk2_rs::init_dds(0, "", "");

    let client = RpcClient::new("rpc_test").expect("failed to create RpcClient");
    client.register_api(1001);
    client.register_api(1002);

    tokio::time::sleep(Duration::from_secs(2)).await;

    let resp = client
        .call(1001, "hello".to_string())
        .await
        .expect("call 1001");
    println!("api=1001 code={} data={}", resp.code, resp.data);

    let resp = client.call(1002, String::new()).await.expect("call 1002");
    println!("api=1002 code={} data={}", resp.code, resp.data);
}
