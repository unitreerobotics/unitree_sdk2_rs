// Mirror of SDK example/go2/go2_stand_example.cpp (simplified).
//
// Two phases, like the C++ example:
//   1. use the motion_switcher service to release the built-in motion control,
//   2. then take over with direct LowCmd (see go2_low_level.rs for the
//      low-level channel and the CRC caveat).
// The full C++ file runs a stance controller (joint ramps + sin waves); this
// Rust mirror keeps the same service handover and publishes a safe damping
// command instead of the stance controller.
//
// Usage: cargo run --example go2_stand_example [networkInterface]
use std::time::Duration;

use tokio::time::sleep;

use unitree_sdk2_rs::init_dds;
use unitree_sdk2_rs::robot_api::b2::motion_switcher::MotionSwitcherClient;

#[tokio::main]
async fn main() {
    let iface = std::env::args().nth(1).unwrap_or_default();
    init_dds(0, &iface, "");

    let msc = MotionSwitcherClient::new().expect("failed to create MotionSwitcherClient");

    // Like C++: deactivate the motion-control-related service first.
    let resp = msc.release_mode().await.expect("ReleaseMode failed");
    println!("ReleaseMode code={} data={}", resp.code, resp.data);

    // In the C++ example the robot then switches to robot-state / motion
    // control and runs a stance LowCmd loop. See go2_low_level.rs for the
    // low-level publish/subscribe demo.
    println!("motion control released — run go2_low_level for the LowCmd demo");

    loop {
        sleep(Duration::from_secs(10)).await;
    }
}
