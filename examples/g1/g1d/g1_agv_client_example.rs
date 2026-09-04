// Mirror of SDK example/g1/g1d/g1_agv_client_example.cpp.
// Drives the g1 AGV base: forward/rotate velocity loop + height adjust cycle.
//
// Usage: cargo run --example g1_agv_client [networkInterface]
use std::time::Duration;

use tokio::time::sleep;

use unitree_sdk2_rs::init_dds;
use unitree_sdk2_rs::robot_api::g1::agv::AgvClient;

#[tokio::main]
async fn main() {
    let iface = std::env::args().nth(1).unwrap_or_default();
    init_dds(0, &iface, "");

    let client = AgvClient::new().expect("failed to create AgvClient");

    // Mirrors the C++ ramp: vx sweeps 0.1..0.5 and back with a fixed vyaw.
    let mut vx = 0.1f32;
    let mut dir = 1.0f32;
    let mut height = 0.0f32;
    loop {
        let resp = client.move_to(vx, 0.0, 0.2).await.expect("Move failed");
        println!("Move vx={vx:.2} code={} data={}", resp.code, resp.data);

        // every ~2 s adjust the chassis height up/down
        height += 0.02 * dir;
        if height.abs() > 0.2 {
            dir = -dir;
        }
        let resp = client
            .height_adjust(height)
            .await
            .expect("HeightAdjust failed");
        println!(
            "HeightAdjust height={height:.2} code={} data={}",
            resp.code, resp.data
        );

        vx += 0.02 * dir;
        if !(0.1..=0.5).contains(&vx) {
            dir = -dir;
        }
        sleep(Duration::from_millis(50)).await; // C++ usleep(50ms)
    }
}
