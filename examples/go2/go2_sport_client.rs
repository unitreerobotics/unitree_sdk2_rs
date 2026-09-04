// Mirror of SDK example/go2/go2_sport_client.cpp.
// Connects the sport service and drives a motion sequence; the high state
// (rt/sportmodestate) is subscribed to read position/IMU feedback.
//
// Usage: cargo run --example go2_sport_client [networkInterface]
use std::time::Duration;

use tokio::time::sleep;

use unitree_sdk2_rs::robot_api::go2::sport::SportClient;
use unitree_sdk2_rs::unitree_go::msg::dds_::SportModeState;
use unitree_sdk2_rs::{init_dds, subscribe};

#[tokio::main]
async fn main() {
    let iface = std::env::args().nth(1).unwrap_or_default();
    init_dds(0, &iface, "");

    // Mirrors the C++ `Custom` setup: sport client + high-state subscription.
    let client = SportClient::new().expect("failed to create SportClient");
    let (_sub, rx) = subscribe::<SportModeState>("rt/sportmodestate");

    sleep(Duration::from_secs(1)).await; // wait for a stable state

    if let Some(s) = rx.borrow().as_ref() {
        println!(
            "initial position: x0={} y0={} yaw0={}",
            s.position[0], s.position[1], s.imu_state.rpy[2]
        );
    }

    // Pick an action like the C++ TEST_MODE switch:
    // stand_up | stand_down | damp | balance_stand | recovery_stand |
    // velocity_move | sit | rise_sit | stop_move
    let mode = std::env::args().nth(2).unwrap_or_else(|| "stand_up".into());
    let resp = match mode.as_str() {
        "stand_up" => client.stand_up().await,
        "stand_down" => client.stand_down().await,
        "damp" => client.damp().await,
        "balance_stand" => client.balance_stand().await,
        "recovery_stand" => client.recovery_stand().await,
        "sit" => client.sit().await,
        "rise_sit" => client.rise_sit().await,
        "velocity_move" => client.move_to(0.3, 0.0, 0.3).await, // vx, vy, vyaw
        _ => client.stop_move().await,
    }
    .expect("RPC call failed");
    println!("{mode}: code={} data={}", resp.code, resp.data);

    // Keep the process alive to observe the motion / incoming state.
    loop {
        if let Some(s) = rx.borrow().as_ref() {
            println!(
                "pos=({:.2},{:.2}) rpy=({:.2},{:.2},{:.2}) vel=({:.2},{:.2})",
                s.position[0],
                s.position[1],
                s.imu_state.rpy[0],
                s.imu_state.rpy[1],
                s.imu_state.rpy[2],
                s.velocity[0],
                s.velocity[1],
            );
        }
        sleep(Duration::from_millis(500)).await;
    }
}
