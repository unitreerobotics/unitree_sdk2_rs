// Mirror of SDK example/r1/audio/r1_audio_client_example.cpp.
// Voice service demo: query/set volume and synthesize speech (TTS). The C++
// example also subscribes rt/audio_msg for ASR results — shown here with a
// std_msgs String subscription (the wire type matches the SDK's ros2 String).
//
// Usage: cargo run --example r1_audio_client [networkInterface]
use std::time::Duration;

use tokio::time::sleep;

use unitree_sdk2_rs::init_dds;
use unitree_sdk2_rs::robot_api::r1::audio::AudioClient;
use unitree_sdk2_rs::std_msgs::msg::dds_::String;
use unitree_sdk2_rs::subscribe;

#[tokio::main]
async fn main() {
    let iface = std::env::args().nth(1).unwrap_or_default();
    init_dds(0, &iface, "");

    let client = AudioClient::new().expect("failed to create AudioClient");
    let (_asr_sub, asr_rx) = subscribe::<String>("rt/audio_msg");

    let resp = client.get_volume().await.expect("GetVolume failed");
    println!("GetVolume code={} data={}", resp.code, resp.data);

    let resp = client.set_volume(100).await.expect("SetVolume failed");
    println!("SetVolume(100) code={} data={}", resp.code, resp.data);

    // C++ example speaks Chinese; use any text the voice service accepts.
    let resp = client
        .tts_maker("Hello, Unitree robot speaking.", 0)
        .await
        .expect("TtsMaker failed");
    println!("TtsMaker code={} data={}", resp.code, resp.data);

    loop {
        if let Some(s) = asr_rx.borrow().as_ref() {
            println!("ASR(rt/audio_msg): {}", s.data);
        }
        sleep(Duration::from_millis(500)).await;
    }
}
