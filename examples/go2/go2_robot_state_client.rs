// Mirror of SDK example/go2/go2_robot_state_client.cpp.
// Queries the robot_state service: set report frequency, switch a service
// on/off, and list the running services.
//
// Usage: cargo run --example go2_robot_state_client [networkInterface] [serviceName]
use std::time::Duration;

use tokio::time::sleep;

use unitree_sdk2_rs::init_dds;
use unitree_sdk2_rs::robot_api::go2::robot_state::RobotStateClient;

#[tokio::main]
async fn main() {
    let iface = std::env::args().nth(1).unwrap_or_default();
    let service_name = std::env::args().nth(2).unwrap_or_else(|| "sport".into());
    init_dds(0, &iface, "");

    let client = RobotStateClient::new().expect("failed to create RobotStateClient");

    // Same sequence as the C++ example.
    let resp = client
        .set_report_freq(3, 30)
        .await
        .expect("SetReportFreq failed");
    println!("SetReportFreq[3,30] code={} data={}", resp.code, resp.data);

    sleep(Duration::from_secs(5)).await;

    let resp = client
        .service_switch(&service_name, 0)
        .await
        .expect("ServiceSwitch(off) failed");
    println!(
        "ServiceSwitch[{service_name},0] code={} data={}",
        resp.code, resp.data
    );

    sleep(Duration::from_secs(5)).await;

    let resp = client
        .service_switch(&service_name, 1)
        .await
        .expect("ServiceSwitch(on) failed");
    println!(
        "ServiceSwitch[{service_name},1] code={} data={}",
        resp.code, resp.data
    );

    sleep(Duration::from_secs(5)).await;

    let resp = client.service_list().await.expect("ServiceList failed");
    println!("ServiceList code={} data={}", resp.code, resp.data);
}
