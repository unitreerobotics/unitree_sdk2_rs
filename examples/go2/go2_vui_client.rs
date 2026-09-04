// Mirror of SDK example/go2/go2_vui_client.cpp.
// Cycles the VUI screen brightness (0..10) and reads it back.
//
// Usage: cargo run --example go2_vui_client [networkInterface]
use std::time::Duration;

use tokio::time::sleep;

use unitree_sdk2_rs::init_dds;
use unitree_sdk2_rs::robot_api::go2::vui::VuiClient;

#[tokio::main]
async fn main() {
    let iface = std::env::args().nth(1).unwrap_or_default();
    init_dds(0, &iface, "");

    let client = VuiClient::new().expect("failed to create VuiClient");

    let mut level = 0i32;
    loop {
        let resp = client
            .set_brightness(level)
            .await
            .expect("SetBrightness failed");
        println!(
            "SetBrightness level={level} code={} data={}",
            resp.code, resp.data
        );
        level = (level + 1) % 11;
        sleep(Duration::from_secs(1)).await;

        let resp = client.get_brightness().await.expect("GetBrightness failed");
        // The value is carried in the response data (JSON).
        println!("GetBrightness code={} data={}", resp.code, resp.data);
        sleep(Duration::from_secs(1)).await;
    }
}
