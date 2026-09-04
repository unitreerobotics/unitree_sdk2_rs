// Mirror of SDK example/h2/high_level/h2_arm_sdk_dds_example.cpp — skeleton.
// Enables the external Arm SDK (fsm must be 4 or 703) and publishes joint
// commands on rt/arm_sdk. The C++ example sets per-joint kp/kd ramps; this
// mirror publishes a safe damping command. CRC over the C++ layout is required
// by the robot (see examples/go2_low_level.rs note) — left 0 here.
//
// Usage: cargo run --example h2_arm_sdk_example [networkInterface]
use std::time::Duration;

use tokio::time::sleep;

use unitree_sdk2_rs::init_dds;
use unitree_sdk2_rs::robot_api::h2::loco::LocoClient;
use unitree_sdk2_rs::unitree_hg::msg::dds_::{LowCmd, MotorCmd};
use unitree_sdk2_rs::Publisher;

#[tokio::main]
async fn main() {
    let iface = std::env::args().nth(1).unwrap_or_default();
    init_dds(0, &iface, "");

    let loco = LocoClient::new().expect("failed to create LocoClient");

    // The arm-sdk channel only accepts commands in fsms 4 / 703.
    let fsm = loco.get_fsm_id().await.expect("GetFsmId failed");
    println!("fsm_id: {} data={}", fsm.code, fsm.data);
    if !fsm.data.contains("4") && !fsm.data.contains("703") {
        eprintln!("current FSM does not accept rt/arm_sdk commands (needs 4 or 703)");
        return;
    }

    let resp = loco
        .set_arm_sdk_status(true)
        .await
        .expect("EnableArmSDK failed");
    println!("EnableArmSDK code={} data={}", resp.code, resp.data);

    // Publish joint commands on rt/arm_sdk (damping here; extend with the C++
    // SetJointCommand kp/kd ramps).
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
    let pub_ = Publisher::<LowCmd>::new("rt/arm_sdk").expect("publisher failed");

    loop {
        pub_.publish(&cmd);
        sleep(Duration::from_millis(10)).await;
    }
}
