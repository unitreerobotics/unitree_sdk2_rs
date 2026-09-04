// Mirror of SDK example/a2/sport/a2_sport_state.cpp (as2_sport_state.cpp is the
// same shape with the same topic) — prints the high state feedback.
//
// Usage: cargo run --example a2_sport_state [networkInterface]
use std::time::Duration;

use tokio::time::sleep;

use unitree_sdk2_rs::unitree_go::msg::dds_::SportModeState;
use unitree_sdk2_rs::{init_dds, subscribe};

const TOPIC_HIGHSTATE: &str = "rt/lf/sportmodestate";

#[tokio::main]
async fn main() {
    let iface = std::env::args().nth(1).unwrap_or_default();
    init_dds(0, &iface, "");

    let (_sub, rx) = subscribe::<SportModeState>(TOPIC_HIGHSTATE);
    println!("subscribing {TOPIC_HIGHSTATE}");

    loop {
        if let Some(s) = rx.borrow().as_ref() {
            println!(
                "pos=({:.2},{:.2}) body_height={:.2} vel=({:.2},{:.2}) yaw_speed={:.2} rpy=({:.2},{:.2},{:.2})",
                s.position[0],
                s.position[1],
                s.body_height,
                s.velocity[0],
                s.velocity[1],
                s.yaw_speed,
                s.imu_state.rpy[0],
                s.imu_state.rpy[1],
                s.imu_state.rpy[2],
            );
        }
        sleep(Duration::from_millis(100)).await;
    }
}
