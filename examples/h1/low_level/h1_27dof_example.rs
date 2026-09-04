// Mirror of SDK example/h1/low_level/h1_27dof_example.cpp — safety-first
// skeleton. The C++ example drives the ankle joints (hg LowCmd) with a swing
// controller; this mirror keeps the low-level channel in damping mode (see
// examples/go2_low_level.rs for the CRC caveat).
//
// Usage: cargo run --example h1_27dof_example eth0
use std::time::Duration;

use tokio::time::sleep;

use unitree_sdk2_rs::unitree_hg::msg::dds_::{LowCmd, LowState, MotorCmd};
use unitree_sdk2_rs::{init_dds, subscribe, Publisher};

#[tokio::main]
async fn main() {
    let iface = std::env::args().nth(1).unwrap_or_default();
    init_dds(0, &iface, "");

    let motor = |_| MotorCmd {
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
        crc: 0,
    };
    let pub_ = Publisher::<LowCmd>::new("rt/lowcmd").expect("publisher failed");
    let (_sub, rx) = subscribe::<LowState>("rt/lowstate");

    loop {
        pub_.publish(&cmd);
        if let Some(s) = rx.borrow().as_ref() {
            println!("tick={} mode_machine={}", s.tick, s.mode_machine);
        }
        sleep(Duration::from_millis(2)).await;
    }
}
