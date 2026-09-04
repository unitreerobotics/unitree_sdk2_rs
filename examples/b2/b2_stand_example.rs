// Mirror of SDK example/b2/b2_stand_example.cpp (and b2w/b2w_stand_example.cpp,
// identical shape) — safety-first skeleton.
//
// Handover to low-level control like the C++ example:
//   1. MotionSwitcherClient::ReleaseMode() stops the built-in motion service,
//   2. then the robot is driven with LowCmd on rt/lowcmd (LowState on
//      rt/lowstate). The C++ stance controller (joint ramps + sin waves) is
//      replaced here by a safe damping command — see go2_low_level.rs for the
//      channel pattern and the CRC caveat.
//
// Usage: cargo run --example b2_stand_example [networkInterface]
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
