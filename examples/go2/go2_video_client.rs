// Mirror of SDK example/go2/go2_video_client.cpp.
// Requests an image sample from the videohub service.
//
// NOTE: the C++ client receives the image through the *binary* RPC channel
// (`Response.binary`); crate::rpc::RpcClient only carries the string payload
// today, so the sample cannot be decoded here yet. The call itself is issued
// and the string part of the response is printed.
//
// Usage: cargo run --example go2_video_client [networkInterface]
use std::time::Duration;

use tokio::time::sleep;

use unitree_sdk2_rs::init_dds;
use unitree_sdk2_rs::robot_api::go2::video::VideoClient;

#[tokio::main]
async fn main() {
    let iface = std::env::args().nth(1).unwrap_or_default();
    init_dds(0, &iface, "");

    let client = VideoClient::new().expect("failed to create VideoClient");

    loop {
        let resp = client
            .get_image_sample()
            .await
            .expect("GetImageSample failed");
        println!(
            "GetImageSample code={} data_len={} (binary image data needs the Response.binary channel)",
            resp.code,
            resp.data.len()
        );
        sleep(Duration::from_secs(3)).await;
    }
}
