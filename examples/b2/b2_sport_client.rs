// Mirror of SDK example/b2/b2_sport_client.cpp.
// Runs a fixed motion sequence through the b2 sport service.
//
// Usage: cargo run --example b2_sport_client [networkInterface]
use std::time::Duration;

use tokio::time::sleep;

use unitree_sdk2_rs::init_dds;
use unitree_sdk2_rs::robot_api::b2::sport::SportClient;

#[tokio::main]
async fn main() {
    let iface = std::env::args().nth(1).unwrap_or_default();
    init_dds(0, &iface, "");

    let client = SportClient::new().expect("failed to create SportClient");

    // C++ example switches between these actions on keyboard input; here the
    // sequence is played in order with pauses.
    let actions: Vec<(&str, u64)> = vec![
        ("damp", 2000),
        ("balance_stand", 3000),
        ("move", 3000),
        ("stand_down", 2000),
        ("recovery_stand", 3000),
        ("hand_stand", 3000),
        ("free_walk", 3000),
        ("classic_walk", 3000),
        ("euler", 3000),
        ("stop_move", 500),
    ];
    for (name, ms) in actions {
        let resp = match name {
            "damp" => client.damp().await,
            "balance_stand" => client.balance_stand().await,
            "move" => client.move_to(0.0, 0.0, 0.5).await,
            "stand_down" => client.stand_down().await,
            "recovery_stand" => client.recovery_stand().await,
            "hand_stand" => client.hand_stand(true).await,
            "free_walk" => client.free_walk().await,
            "classic_walk" => client.classic_walk(true).await,
            "euler" => client.euler(0.0, 0.2, 0.0).await,
            _ => client.stop_move().await,
        }
        .expect("RPC call failed");
        println!("{name}: code={} data={}", resp.code, resp.data);
        sleep(Duration::from_millis(ms)).await;
    }
}
