// Mirror of SDK example/go2w/go2w_sport_client.cpp.
// go2w is driven with the go2 sport service; this mirrors the C++ velocity
// walk demo (each Move lasts ~0.5 s unless repeated in a loop).
//
// Usage: cargo run --example go2w_sport_client [networkInterface]
use std::time::Duration;

use tokio::time::sleep;

use unitree_sdk2_rs::init_dds;
use unitree_sdk2_rs::robot_api::go2::sport::SportClient;

#[tokio::main]
async fn main() {
    let iface = std::env::args().nth(1).unwrap_or_default();
    init_dds(0, &iface, "");

    let client = SportClient::new().expect("failed to create SportClient");

    for (vx, vy, vyaw, ms) in [
        (0.5f32, 0.0f32, 0.0f32, 4000u64),
        (0.0, 0.3, 0.0, 4000),
        (0.0, 0.0, 0.5, 3000),
    ] {
        let resp = client.move_to(vx, vy, vyaw).await.expect("Move failed");
        println!(
            "Move({vx},{vy},{vyaw}) code={} data={}",
            resp.code, resp.data
        );
        sleep(Duration::from_millis(ms)).await;
    }
    let resp = client.stop_move().await.expect("StopMove failed");
    println!("StopMove code={} data={}", resp.code, resp.data);
}
