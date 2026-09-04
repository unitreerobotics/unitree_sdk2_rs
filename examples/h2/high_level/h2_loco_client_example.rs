// Mirror of SDK example/h2/high_level/h2_loco_client_example.cpp.
// Command-driven loco (sport) client: pass the operation and args, e.g.
//
//   cargo run --example h2_loco_client eth0 get_fsm_id
//   cargo run --example h2_loco_client eth0 set_velocity 0.2 0.0 0.0 1.0
//   cargo run --example h2_loco_client eth0 set_fsm_id 1
//   cargo run --example h2_loco_client eth0 set_speed_mode 2
//   cargo run --example h2_loco_client eth0 set_swing_height 0.12
//   cargo run --example h2_loco_client eth0 set_balance_mode 1
//   cargo run --example h2_loco_client eth0 set_task_id 3
//   cargo run --example h2_loco_client eth0 set_punch_api 0.5 0.2
use unitree_sdk2_rs::init_dds;
use unitree_sdk2_rs::robot_api::h2::loco::LocoClient;

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
        "get_balance_mode" => client.get_balance_mode().await,
        "get_swing_height" => client.get_swing_height().await,
        "get_stand_height" => client.get_stand_height().await,
        "set_fsm_id" => {
            let v = f32s(rest);
            client.set_fsm_id(v[0] as i32).await
        }
        "set_velocity" => {
            let v = f32s(rest);
            // vx, vy, omega, duration
            client
                .set_velocity(v[0], v[1], v[2], v.get(3).copied().unwrap_or(1.0))
                .await
        }
        "set_swing_height" => client.set_swing_height(f32s(rest)[0]).await,
        "set_stand_height" => client.set_stand_height(f32s(rest)[0]).await,
        "set_speed_mode" => client.set_speed_mode(f32s(rest)[0] as i32).await,
        "set_balance_mode" => client.set_balance_mode(f32s(rest)[0] as i32).await,
        "set_task_id" => client.set_task_id(f32s(rest)[0] as i32).await,
        "set_punch_api" => client.set_punch_api(&f32s(rest)).await,
        "set_arm_sdk_status" => {
            let v = f32s(rest);
            client.set_arm_sdk_status(v[0] != 0.0).await
        }
        other => {
            eprintln!("unknown op: {other}");
            return;
        }
    }
    .expect("RPC call failed");
    println!("{op}: code={} data={}", resp.code, resp.data);
}
