// Mirror of SDK example/g1/g1d/g1d_height_control.cpp.
// Closed-loop chassis height control on the g1 AGV: subscribe the chassis
// height reference topic and drive HeightAdjust accordingly.
//
// The C++ example subscribes the height command from a topic and runs a PID
// loop; here the reference is generated locally as a slow sine, keeping the
// same HeightAdjust control surface.
//
// Usage: cargo run --example g1d_height_control [networkInterface]
use std::time::Duration;

use tokio::time::sleep;

use unitree_sdk2_rs::init_dds;
use unitree_sdk2_rs::robot_api::g1::agv::AgvClient;

#[tokio::main]
async fn main() {
    let iface = std::env::args().nth(1).unwrap_or_default();
    init_dds(0, &iface, "");

    let client = AgvClient::new().expect("failed to create AgvClient");

    let mut t = 0.0f32;
    loop {
        // slow height setpoint: ±0.15 m around the neutral height
        let height = 0.15 * (2.0 * std::f32::consts::PI * 0.1 * t).sin();
        let resp = client
            .height_adjust(height)
            .await
            .expect("HeightAdjust failed");
        println!(
            "HeightAdjust height={height:.3} code={} data={}",
            resp.code, resp.data
        );
        t += 0.01;
        sleep(Duration::from_millis(10)).await; // C++ kControlPeriod = 10 ms
    }
}
