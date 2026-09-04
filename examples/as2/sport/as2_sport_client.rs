// Mirror of SDK example/a2/sport/as2_sport_client.cpp.
// Runs a motion sequence on the as2 sport service.
//
// Usage: cargo run --example as2_sport_client [networkInterface]
use std::time::Duration;

use tokio::time::sleep;

use unitree_sdk2_rs::init_dds;
use unitree_sdk2_rs::robot_api::as2::sport::SportClient;

#[tokio::main]
async fn main() {
    let iface = std::env::args().nth(1).unwrap_or_default();
    init_dds(0, &iface, "");

    let client = SportClient::new().expect("failed to create SportClient");

    // Compact motion sequence mirroring the C++ keyboard-driven demo.
    let seq: [(&str, u64); 6] = [
        ("damp", 2000),
        ("balance_stand", 3000),
        ("move", 3000),
        ("stand_down", 2000),
        ("stand_up", 3000),
        ("stop", 500),
    ];
    for (name, ms) in seq {
        let resp = match name {
            "damp" => client.damp().await,
            "balance_stand" => client.balance_stand().await,
            "move" => client.move_to(0.0, 0.0, 0.5).await,
            "stand_down" => client.stand_down().await,
            "stand_up" => client.stand_up().await,
            _ => client.stop_move().await,
        }
        .expect("RPC call failed");
        println!("{name}: code={} data={}", resp.code, resp.data);
        sleep(Duration::from_millis(ms)).await;
    }
}
