// Mirror of SDK example/go2/go2_trajectory_follow.cpp.
//
// NOTE: the shipped unitree_sdk2 headers no longer expose TrajectoryFollow on the
// go2 sport client (no API_ID constant in go2/sport/sport_api.hpp), so this
// example documents the trajectory math and the wire payload, and performs the
// call through the b2 sport client (api 1018), which is the current SDK home of
// `TrajectoryFollow` with an identical PathPoint layout.
//
// Usage: cargo run --example go2_trajectory_follow [networkInterface]
use std::f32::consts::PI;
use std::time::Duration;

use tokio::time::sleep;

use unitree_sdk2_rs::init_dds;
use unitree_sdk2_rs::robot_api::b2::sport::{PathPoint, SportClient as B2SportClient};

fn build_path(count: f32) -> Vec<PathPoint> {
    // Mirrors the C++ generator: a sinusoidal track at vx = 0.3 m/s.
    let vx = 0.3f32;
    let delta = 0.06f32;
    (0..30)
        .map(|i| {
            let i = i as f32;
            let var = count + i * delta;
            PathPoint {
                time_from_start: i * delta,
                x: vx * var,
                y: 0.6 * (PI * vx * var).sin(),
                yaw: 2.0 * 0.6 * vx * PI * (PI * vx * var).cos(),
                vx,
                vy: PI * vx * 0.6 * (PI * vx * var).cos(),
                vyaw: -PI * vx * 2.0 * 0.6 * vx * PI * (PI * vx * var).sin(),
            }
        })
        .collect()
}

#[tokio::main]
async fn main() {
    let iface = std::env::args().nth(1).unwrap_or_default();
    init_dds(0, &iface, "");

    let client = B2SportClient::new().expect("failed to create SportClient");

    let mut count = 0.0f32;
    let dt = 0.002f32;
    loop {
        let path = build_path(count);
        let resp = client
            .trajectory_follow(&path)
            .await
            .expect("TrajectoryFollow failed");
        println!("TrajectoryFollow code={} data={}", resp.code, resp.data);
        count += dt;
        sleep(Duration::from_millis(2)).await;
    }
}
