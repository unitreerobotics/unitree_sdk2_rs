// Mirror of SDK example/h2/high_level/g1_arm_action_example.cpp.
// Executes preset / custom H2 arm actions:
//
//   cargo run --example g1_arm_action_example eth0 --id 1      (0 lists actions, 99 releases)
//   cargo run --example g1_arm_action_example eth0 --list
//   cargo run --example g1_arm_action_example eth0 --name "<action>"
//   cargo run --example g1_arm_action_example eth0 --stop
use unitree_sdk2_rs::init_dds;
use unitree_sdk2_rs::robot_api::g1::arm::G1ArmActionClient;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let iface = args.get(1).cloned().unwrap_or_default();
    init_dds(0, &iface, "");

    let client = G1ArmActionClient::new().expect("failed to create G1ArmActionClient");

    let mut i = 2;
    let mut done = false;
    while i < args.len() {
        let resp = match args[i].as_str() {
            "--id" => {
                let id: i32 = args.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or(0);
                i += 1;
                client.execute_action(id).await
            }
            "--list" => client.get_action_list().await,
            "--name" => {
                let name = args.get(i + 1).cloned().unwrap_or_default();
                i += 1;
                client.execute_custom_action(&name).await
            }
            "--stop" => client.stop_custom_action().await,
            other => {
                eprintln!("unknown option: {other}");
                return;
            }
        }
        .expect("RPC call failed");
        println!("code={} data={}", resp.code, resp.data);
        i += 1;
        done = true;
    }
    if !done {
        eprintln!(
            "usage: g1_arm_action_example <iface> --id <n> | --list | --name <action> | --stop"
        );
    }
}
