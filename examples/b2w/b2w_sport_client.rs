// Mirror of SDK example/b2w/b2w_sport_client.cpp.
// Drives b2w (wheeled b2) with directional velocity commands; note the robot
// stops automatically ~0.5 s after the last Move request.
//
// Usage: cargo run --example b2w_sport_client [networkInterface]
use std::time::Duration;

use tokio::time::sleep;

use unitree_sdk2_rs::init_dds;
use unitree_sdk2_rs::robot_api::b2::sport::SportClient;

#[tokio::main]
async fn main() {
    let iface = std::env::args().nth(1).unwrap_or_default();
    init_dds(0, &iface, "");

    let client = SportClient::new().expect("failed to create SportClient");

    // Sequence: forward, side, turn — each Move lasts ~0.5 s unless repeated.
    for (vx, vy, vyaw, ms) in [
        (0.3f32, 0.0f32, 0.0f32, 3000u64),
        (0.0, 0.3, 0.0, 3000),
        (0.0, 0.0, 0.5, 3000),
    ] {
        let resp = client.move_to(vx, vy, vyaw).await.expect("Move failed");
        println!(
            "Move(vx={vx},vy={vy},vyaw={vyaw}) code={} data={}",
            resp.code, resp.data
        );
        sleep(Duration::from_millis(ms)).await;
    }

    let resp = client.stop_move().await.expect("StopMove failed");
    println!("StopMove code={} data={}", resp.code, resp.data);
}
