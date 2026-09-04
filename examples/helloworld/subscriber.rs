// Equivalent mirror of SDK example/helloworld/subscriber.cpp.
//
// The C++ original subscribes to topic "TopicHelloWorld" with the custom
// `HelloWorldData::Msg` type and exercises the channel lifecycle: subscribe →
// wait 5 s → close → wait 3 s → re-subscribe → wait 5 s → close → wait 3 s →
// subscribe again and run forever. Each received sample is printed as
// `userID:<int64>, message:<string>`.
//
// As in helloworld_pub.rs, the custom type is replaced by std_msgs String
// (Rust types come from msgs/*.msg only), with the original `userID`/`message`
// fields packed into the `data` string as "<userID>|<message>". The lifecycle
// is mirrored through the subscription handle: dropping the returned
// `Subscriber` unsubscribes (like C++ CloseChannel), creating a new one
// re-subscribes (like InitChannel).
//
// Interop note: speaks type `std_msgs::msg::dds_::String_` on "TopicHelloWorld"
// — pair it with the matching Rust helloworld_pub, not with the original
// HelloWorldData C++ endpoints.
//
// Usage: cargo run --example helloworld_sub [networkInterface]
use std::time::Duration;

use tokio::time::sleep;

use unitree_sdk2_rs::std_msgs::msg::dds_::String as StringMsg;
use unitree_sdk2_rs::{init_dds, subscribe};

const TOPIC: &str = "TopicHelloWorld";

/// Subscribes for `seconds` seconds and prints every newly received sample.
///
/// `hello` is prepended so the caller can tell the sessions apart, mirroring
/// the C++ `reseted. sleep 3` markers between re-subscribes.
async fn listen(hello: &str, seconds: u64) {
    println!("{hello} subscribing on {TOPIC}");
    let (_sub, rx) = subscribe::<StringMsg>(TOPIC);

    // Poll every 200 ms. `seconds == u64::MAX` means "run forever" (used for
    // the final session, like the C++ infinite tail loop); saturating_mul
    // keeps `ticks_max` at u64::MAX so the loop never ends.
    let ticks_max = seconds.saturating_mul(5);
    let mut ticks = 0u64;
    // Track the payload; generated msg types have no PartialEq, so compare
    // (and own) the data string — the watch channel may drop the old value
    // on the next sample, so keep an owned copy rather than a borrow.
    let mut last_data: Option<std::string::String> = None;
    while ticks < ticks_max {
        if let Some(m) = rx.borrow().as_ref() {
            // Print each sample only once (C++ prints on every message).
            if last_data.as_deref() != Some(m.data.as_str()) {
                // Decode the "<userID>|<message>" payload sent by the publisher.
                let (user_id, message) = m.data.split_once('|').unwrap_or(("?", &m.data[..]));
                println!("userID:{user_id}, message:{message}");
                last_data = Some(m.data.clone());
            }
        }
        sleep(Duration::from_millis(200)).await;
        ticks += 1;
    }
    // `_sub` is dropped here → unsubscribe (C++ CloseChannel).
}

#[tokio::main]
async fn main() {
    let iface = std::env::args().nth(1).unwrap_or_default();
    init_dds(0, &iface, "");

    // Mirrors the C++ open/close/open sequence:
    // InitChannel → 5 s → CloseChannel → 3 s → InitChannel → 5 s →
    // CloseChannel → 3 s → InitChannel → keep running.
    listen("session 1:", 5).await;
    println!("reseted. sleep 3");
    sleep(Duration::from_secs(3)).await;

    listen("session 2:", 5).await;
    println!("reseted. sleep 3");
    sleep(Duration::from_secs(3)).await;

    // Final session: keep the process alive, like the C++ infinite tail loop.
    listen("session 3:", u64::MAX).await;
}
