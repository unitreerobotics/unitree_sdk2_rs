// Verifies that the RPC layer is implemented on top of the DDS `request`/`response`
// channels: while a call runs, a `unitree_api::msg::dds_::Request` is published on
// `rt/api/<service>/request` and a `unitree_api::msg::dds_::Response` is published
// back on `rt/api/<service>/response`.
//
// Runs the RPC server and client in a single process and asserts that both
// channels carry the expected messages.
use std::sync::Arc;
use std::time::Duration;

use unitree_sdk2_rs::rpc::{RpcClient, RpcServer};
use unitree_sdk2_rs::unitree_api::msg::dds_::{Request, Response};
use unitree_sdk2_rs::{init_dds, subscribe};

const SERVICE: &str = "rpc_req_res_test";
const API_ID: i64 = 1001;
const EXPECTED_RESPONSE: &str = "echo: hello\u{000C}\u{001F}";

fn request_topic() -> String {
    format!("rt/api/{SERVICE}/request")
}
fn response_topic() -> String {
    format!("rt/api/{SERVICE}/response")
}

#[tokio::main]
async fn main() {
    init_dds(0, "", "");

    // Tap the underlying request/response channels.
    let (_req_sub, req_rx) = subscribe::<Request>(&request_topic());
    let (_res_sub, res_rx) = subscribe::<Response>(&response_topic());

    // Start the RPC server in this process.
    let server = Arc::new(RpcServer::new(SERVICE).expect("failed to create RpcServer"));
    server.register_handler(API_ID as i32, |req| format!("echo: {req}\u{000C}\u{001F}"));
    let server_task = tokio::spawn({
        let srv = server.clone();
        async move { srv.start().await }
    });
    tokio::time::sleep(Duration::from_millis(500)).await;

    // Make an RPC call.
    let client = RpcClient::new(SERVICE).expect("failed to create RpcClient");
    client.register_api(API_ID as i32);
    let resp = client
        .call(API_ID as i32, "hello".to_string())
        .await
        .expect("RPC call failed");
    assert_eq!(resp.code, 0, "unexpected RPC return code: {}", resp.code);
    assert_eq!(resp.data, EXPECTED_RESPONSE);
    println!("RPC call OK: code={} data={}", resp.code, resp.data);

    // The request published by the client must be visible on the request channel.
    tokio::time::sleep(Duration::from_millis(200)).await;
    let req = req_rx
        .borrow()
        .clone()
        .expect("no message on the request channel");
    assert_eq!(
        req.header.identity.api_id, API_ID,
        "request api_id mismatch: {} != {}",
        req.header.identity.api_id, API_ID
    );
    println!(
        "request channel OK: api_id={} parameter={}",
        req.header.identity.api_id, req.parameter
    );

    // The response sent back by the server must be visible on the response channel.
    let res = res_rx
        .borrow()
        .clone()
        .expect("no message on the response channel");
    assert_eq!(
        res.header.status.code, 0,
        "response code mismatch: {}",
        res.header.status.code
    );
    assert_eq!(res.data, EXPECTED_RESPONSE);
    println!(
        "response channel OK: code={} data={}",
        res.header.status.code, res.data
    );

    server_task.abort();
    println!("RPC over DDS request/response verified ✓");
}
