// Equivalent mirror of SDK example/helloworld/publisher.cpp.
//
// The C++ original publishes a *custom* DDS message `HelloWorldData::Msg`
// (generated from HelloWorldData.idl by Cyclone DDS idlc) on topic
// "TopicHelloWorld":
//
//   module HelloWorldData { struct Msg { int64 userID; string message; }; };
//   userID  = GetCurrentTimeMillisecond()   (monotone-ish wall-clock ms)
//   message = "HelloWorld."
//
// Rust message types are generated exclusively from msgs/*.msg (see build.rs),
// so a bespoke `HelloWorldData::Msg` cannot be expressed. This file is the
// closest equivalent: it demonstrates the same periodic pub/sub demo on the
// same topic name using the ready-made std_msgs/msg/String type, packing the
// two original fields (`userID`, `message`) into the single `data` string.
//
// Interop note: DDS matches samples by registered type name, so this pair
// speaks type `std_msgs::msg::dds_::String_`, NOT `HelloWorldData::Msg` — it
// interoperates with peers using the String type only, not with the original
// C++ HelloWorldData endpoints.
//
// Run publisher and subscriber in two terminals:
//   cargo run --example helloworld_pub [networkInterface]
//   cargo run --example helloworld_sub [networkInterface]
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use tokio::time::sleep;

use unitree_sdk2_rs::std_msgs::msg::dds_::String as StringMsg; // aliased to avoid shadowing std String
use unitree_sdk2_rs::{init_dds, Publisher};

const TOPIC: &str = "TopicHelloWorld";
const MESSAGE: &str = "HelloWorld.";

/// Milliseconds since the Unix epoch — C++ `GetCurrentTimeMillisecond()`
/// equivalent, used to fill the original `userID` int64 field.
fn now_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64
}

#[tokio::main]
async fn main() {
    let iface = std::env::args().nth(1).unwrap_or_default();
    init_dds(0, &iface, "");

    let publisher =
        Publisher::<StringMsg>::new(TOPIC).expect("failed to create publisher on TopicHelloWorld");
    println!("publishing on {TOPIC} (std_msgs String stand-in for HelloWorldData::Msg)");

    // The C++ example sleeps 1 s between writes, forever.
    loop {
        // Encode the original (userID, message) pair as "<userID>|<message>".
        let msg = StringMsg {
            data: format!("{}|{}", now_ms(), MESSAGE),
        };
        publisher.publish(&msg);
        sleep(Duration::from_secs(1)).await;
    }
}
