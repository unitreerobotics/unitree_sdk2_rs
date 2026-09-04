// Mirror of SDK example/r1/high_level/r1_loco_client_example.cpp.
// Command-driven r1 loco (sport) client:
//
//   cargo run --example r1_loco_client eth0 get_fsm_id
//   cargo run --example r1_loco_client eth0 set_fsm_id 1
//   cargo run --example r1_loco_client eth0 move 0.2 0.0 0.0
//   cargo run --example r1_loco_client eth0 stop_move
//   cargo run --example r1_loco_client eth0 set_speed_mode 2
//   cargo run --example r1_loco_client eth0 switch_move_mode 1
use unitree_sdk2_rs::init_dds;
use unitree_sdk2_rs::robot_api::r1::loco::LocoClient;

fn f32s(args: &[String]) -> Vec<f32> {
    args.iter().filter_map(|a| a.parse::<f32>().ok()).collect()
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let iface = args.get(1).cloned().unwrap_or_default();
    let op = args.get(2).map(|s| s.as_str()).unwrap_or("get_fsm_id");
    let rest = &args[3..];
    init_dds(0, &iface, "");

    let client = LocoClient::new().expect("failed to create LocoClient");

    let resp = match op {
        "get_fsm_id" => client.get_fsm_id().await,
        "get_fsm_mode" => client.get_fsm_mode().await,
        "set_fsm_id" => client.set_fsm_id(f32s(rest)[0] as i32).await,
        // C++ `Move` (client-side velocity with its own duration); the Rust
        // mirror takes the continous_move flag explicitly.
        "move" => {
            let v = f32s(rest);
            client.move_to(v[0], v[1], v[2], false).await
        }
        "stop_move" => client.stop_move().await,
        "set_velocity" => {
            let v = f32s(rest);
            client
                .set_velocity(v[0], v[1], v[2], v.get(3).copied().unwrap_or(1.0))
                .await
        }
        "set_speed_mode" => client.set_speed_mode(f32s(rest)[0] as i32).await,
        "damp" => client.damp().await,
        "start" => client.start().await,
        "stand_up" => client.stand_up().await,
        "zero_torque" => client.zero_torque().await,
        other => {
            eprintln!("unknown op: {other}");
            return;
        }
    }
    .expect("RPC call failed");
    println!("{op}: code={} data={}", resp.code, resp.data);
}
