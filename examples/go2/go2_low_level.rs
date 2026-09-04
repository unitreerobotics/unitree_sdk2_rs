// Mirror of SDK example/go2/go2_low_level.cpp (simplified, safety-first).
//
// Demonstrates the low-level channel: subscribe `rt/lowstate` and publish
// `rt/lowcmd`. The full C++ example drives a sine-swing on legs 2/3 with a
// joint interpolation ramp; here the motors are put in damping mode so the
// robot stays compliant — extend `damping_cmd()` with the C++ ramp logic.
//
// CRC note: the firmware validates `LowCmd.crc` over the C++ struct memory
// layout (CRC-32, poly 0x04C11DB7). Reproducing that exactly in Rust needs a
// repr(C) mirror of the DDS struct; crc32() below is the standard CRC-32 so it
// can be plugged in once that layout mirror exists.
//
// Usage: cargo run --example go2_low_level [networkInterface]
use std::time::Duration;

use tokio::time::sleep;

use unitree_sdk2_rs::unitree_go::msg::dds_::{BmsCmd, LowCmd, LowState, MotorCmd};
use unitree_sdk2_rs::{init_dds, subscribe, Publisher};

const TOPIC_LOWCMD: &str = "rt/lowcmd";
const TOPIC_LOWSTATE: &str = "rt/lowstate";

/// Standard CRC-32 (same polynomial and init as the SDK crc32_core).
fn crc32(data: &[u8]) -> u32 {
    let mut crc: u32 = 0xFFFF_FFFF;
    for &b in data {
        crc ^= b as u32;
        for _ in 0..8 {
            if crc & 1 != 0 {
                crc = (crc >> 1) ^ 0xEDB8_8320;
            } else {
                crc >>= 1;
            }
        }
    }
    !crc
}

/// Damping-mode LowCmd: all motors hold position with zero torque.
fn damping_cmd() -> LowCmd {
    let motor = |_| MotorCmd {
        mode: 0x01,  // servo (PMSM) mode
        q: 2.146e9,  // PosStopF
        dq: 16000.0, // VelStopF
        tau: 0.0,
        kp: 0.0,
        kd: 0.0,
        reserve: [0; 3],
    };
    let mut cmd = LowCmd {
        head: [0xFE, 0xEF],
        level_flag: 0xFF,
        frame_reserve: 0,
        sn: [0; 2],
        version: [0; 2],
        bandwidth: 0,
        motor_cmd: std::array::from_fn(motor),
        bms_cmd: BmsCmd {
            off: 0,
            reserve: [0; 3],
        },
        wireless_remote: [0; 40],
        led: [0; 12],
        fan: [0; 2],
        gpio: 0,
        reserve: 0,
        crc: 0,
    };
    // crc over the CDR layout is not byte-identical to the C++ memory layout;
    // see the note above — left 0 for the damping demo.
    let _ = crc32(&[]);
    cmd.crc = 0;
    cmd
}

#[tokio::main]
async fn main() {
    let iface = std::env::args().nth(1).unwrap_or_default();
    init_dds(0, &iface, "");

    let pub_ = Publisher::<LowCmd>::new(TOPIC_LOWCMD).expect("failed to create LowCmd publisher");
    let (_sub, rx) = subscribe::<LowState>(TOPIC_LOWSTATE);

    let cmd = damping_cmd();
    println!("publishing damping LowCmd on {TOPIC_LOWCMD}, subscribing {TOPIC_LOWSTATE}");

    loop {
        pub_.publish(&cmd);
        if let Some(s) = rx.borrow().as_ref() {
            println!(
                "tick={} q0={:.3} q1={:.3} q2={:.3}",
                s.tick, s.motor_state[0].q, s.motor_state[1].q, s.motor_state[2].q
            );
        }
        sleep(Duration::from_millis(2)).await; // ~500 Hz like the C++ example
    }
}
