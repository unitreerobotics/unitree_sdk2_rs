// Mirror of SDK example/g1/g1d/g1_ankle_swing_example.cpp — safety-first skeleton (ankle swing (g1 ankle joints)).
//
// The C++ example switches to low-level control (hg LowCmd on rt/lowcmd) and
// swings the arm joints (g1 35-motor layout) to predefined poses. This mirror
// keeps the low-level channel (damping mode) and the motion_switcher handover;
// for arm poses prefer the high-level API:
//   robot_api::g1::arm::G1ArmActionClient (execute_action / execute_custom_action)
//
// Usage: cargo run --example g1d_arm_example [networkInterface]
use std::time::Duration;

use tokio::time::sleep;

use unitree_sdk2_rs::init_dds;
use unitree_sdk2_rs::robot_api::b2::motion_switcher::MotionSwitcherClient;
use unitree_sdk2_rs::unitree_hg::msg::dds_::{LowCmd, LowState};
use unitree_sdk2_rs::{subscribe, Publisher};

#[tokio::main]
async fn main() {
    let iface = std::env::args().nth(1).unwrap_or_default();
    init_dds(0, &iface, "");

    // Handover to low-level control, like the C++ example.
    let msc = MotionSwitcherClient::new().expect("failed to create MotionSwitcherClient");
    let resp = msc.release_mode().await.expect("ReleaseMode failed");
    println!("ReleaseMode code={} data={}", resp.code, resp.data);

    // Low-level channel: publish a damping LowCmd (hg 35 motors).
    let motor = |_| unitree_sdk2_rs::unitree_hg::msg::dds_::MotorCmd {
        mode: 0x01,
        q: 0.0,
        dq: 0.0,
        tau: 0.0,
        kp: 0.0,
        kd: 0.0,
        reserve: 0,
    };
    let cmd = LowCmd {
        mode_pr: 0,
        mode_machine: 0,
        motor_cmd: std::array::from_fn(motor),
        reserve: [0; 4],
        crc: 0, // CRC over the C++ layout — see examples/go2_low_level.rs note
    };
    let pub_ = Publisher::<LowCmd>::new("rt/lowcmd").expect("publisher failed");
    let (_sub, rx) = subscribe::<LowState>("rt/lowstate");

    loop {
        pub_.publish(&cmd);
        if let Some(s) = rx.borrow().as_ref() {
            println!("tick={}", s.tick);
        }
        sleep(Duration::from_millis(10)).await;
    }
}
