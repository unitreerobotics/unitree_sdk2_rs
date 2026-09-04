// Mirror of SDK example/go2w/go2w_stand_example.cpp — safety-first skeleton.
// Same shape as go2_stand_example: release the motion service then drive
// low level. See examples/go2_stand_example.rs and go2_low_level.rs.
//
// Usage: cargo run --example go2w_stand_example [networkInterface]
use std::time::Duration;

use tokio::time::sleep;

use unitree_sdk2_rs::init_dds;
use unitree_sdk2_rs::robot_api::b2::motion_switcher::MotionSwitcherClient;

#[tokio::main]
async fn main() {
    let iface = std::env::args().nth(1).unwrap_or_default();
    init_dds(0, &iface, "");

    let msc = MotionSwitcherClient::new().expect("failed to create MotionSwitcherClient");
    let resp = msc.release_mode().await.expect("ReleaseMode failed");
    println!("ReleaseMode code={} data={}", resp.code, resp.data);

    println!("motion control released — run go2_low_level for the LowCmd demo");
    loop {
        sleep(Duration::from_secs(10)).await;
    }
}
