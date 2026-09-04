// Mirror of SDK example/wireless_controller/{main.cpp,advanced_gamepad.hpp}.
//
// Subscribes to the robot's wireless receiver DDS topic `rt/wirelesscontroller`
// (message `unitree_go::msg::dds_::WirelessController_`, defined by
// msgs/unitree_go/msg/WirelessController.msg) and turns the raw samples into a
// filtered gamepad state:
//
//   - stick axes lx/ly/rx/ry: dead-zone + exponential smoothing
//     lx = lx*(1-smooth) + (|raw| < dead_zone ? 0 : raw)*smooth   (per axis)
//   - `keys` (u16 bitmask): each bit drives one button with rising/falling
//     edge flags (pressed / on_press / on_release), exactly like the C++
//     `Button::update`.
//
// `keys` bit layout follows the `xKeySwitchUnion` bitfield in
// advanced_gamepad.hpp (declared R1→left, packed LSB-first on little-endian):
//
//   bit 0..7 : R1 L1 start select R2 L2 F1 F2
//   bit 8..15: A  B  X     Y     up right down left
//
// Loop cadence mirrors the C++ 40 ms recurrent work thread (~25 Hz); the C++
// demo counts A button presses and prints lx + A state every tick — kept here
// with all four axes for clarity.
//
// Usage: cargo run --example wireless_controller [networkInterface]
use std::time::Duration;

use tokio::time::sleep;

use unitree_sdk2_rs::unitree_go::msg::dds_::WirelessController;
use unitree_sdk2_rs::{init_dds, subscribe};

const TOPIC_JOYSTICK: &str = "rt/wirelesscontroller";

/// Button bit positions in `WirelessController.keys` (advanced_gamepad.hpp).
#[allow(dead_code)] // only A is observed below, the rest document the layout
mod key {
    pub const R1: u16 = 1 << 0;
    pub const L1: u16 = 1 << 1;
    pub const START: u16 = 1 << 2;
    pub const SELECT: u16 = 1 << 3;
    pub const R2: u16 = 1 << 4;
    pub const L2: u16 = 1 << 5;
    pub const F1: u16 = 1 << 6;
    pub const F2: u16 = 1 << 7;
    pub const A: u16 = 1 << 8;
    pub const B: u16 = 1 << 9;
    pub const X: u16 = 1 << 10;
    pub const Y: u16 = 1 << 11;
    pub const UP: u16 = 1 << 12;
    pub const RIGHT: u16 = 1 << 13;
    pub const DOWN: u16 = 1 << 14;
    pub const LEFT: u16 = 1 << 15;
}

/// One button: level plus rising/falling edge flags (`Button` in the C++).
#[derive(Default, Debug)]
struct Button {
    pressed: bool,
    on_press: bool,
    on_release: bool,
}

impl Button {
    /// C++ `Button::update`: on_press is the rising edge, on_release the
    /// falling edge, both computed against the *previous* level.
    fn update(&mut self, state: bool) {
        self.on_press = state && !self.pressed;
        self.on_release = !state && self.pressed;
        self.pressed = state;
    }
}

/// Filtered gamepad state (`Gamepad` in the C++).
#[derive(Default)]
struct Gamepad {
    smooth: f32,
    dead_zone: f32,
    lx: f32,
    ly: f32,
    rx: f32,
    ry: f32,
    /// Buttons indexed by bit position 0..16 (R1..left).
    buttons: [Button; 16],
}

impl Gamepad {
    /// C++ `Gamepad::update(joystick_msg)`: smooth each stick (dead-zone first)
    /// and refresh every button edge.
    fn update(&mut self, msg: &WirelessController) {
        self.lx = Self::smooth_stick(self.lx, msg.lx, self.smooth, self.dead_zone);
        self.ly = Self::smooth_stick(self.ly, msg.ly, self.smooth, self.dead_zone);
        self.rx = Self::smooth_stick(self.rx, msg.rx, self.smooth, self.dead_zone);
        self.ry = Self::smooth_stick(self.ry, msg.ry, self.smooth, self.dead_zone);
        for (bit, button) in self.buttons.iter_mut().enumerate() {
            button.update(msg.keys & (1 << bit) != 0);
        }
    }

    fn smooth_stick(prev: f32, raw: f32, smooth: f32, dead_zone: f32) -> f32 {
        prev * (1.0 - smooth) + if raw.abs() < dead_zone { 0.0 } else { raw } * smooth
    }

    fn button(&self, mask: u16) -> &Button {
        &self.buttons[mask.trailing_zeros() as usize]
    }
}

#[tokio::main]
async fn main() {
    let iface = std::env::args().nth(1).unwrap_or_default();
    init_dds(0, &iface, "");

    let (_sub, rx) = subscribe::<WirelessController>(TOPIC_JOYSTICK);

    // main.cpp overrides the struct defaults: smooth 0.2, dead zone 0.5.
    let mut gamepad = Gamepad {
        smooth: 0.2,
        dead_zone: 0.5,
        ..Default::default()
    };
    let mut press_count = 0u32;

    println!(
        "subscribing {TOPIC_JOYSTICK} — waiting for the wireless receiver (\"rt/wirelesscontroller\")"
    );

    // 40 ms work-thread period of the C++ `GamepadExample::Start`.
    loop {
        if let Some(msg) = rx.borrow().as_ref() {
            gamepad.update(msg);

            let a = gamepad.button(key::A);
            if a.on_press {
                press_count += 1;
            }

            // Same printout as the C++ Step(): smoothed lx, A state, press count.
            println!(
                "lx: {:.2} | A: pressed: {}; on_press: {}; on_release: {} | press count: {}",
                gamepad.lx, a.pressed, a.on_press, a.on_release, press_count
            );
            println!("===========================");
        }
        sleep(Duration::from_micros(40_000)).await;
    }
}
