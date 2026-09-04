// Mirror of SDK example/h1/high_level/h1_2_arm_sdk_dds_example.cpp — not yet
// portable.
//
// The h1 arm-sdk demo enables an external arm-sdk mode via the loco service and
// then publishes joint commands on rt/arm_sdk (hg LowCmd), same pattern as the
// h2 one. robot_api::h1::loco currently has no enable-arm-sdk switch, so the
// arm-sdk flow cannot be driven yet; the low-level channel pattern is shown in
// examples/h2_arm_sdk_example.rs and examples/go2_low_level.rs.
//
// Usage: (not runnable yet)
fn main() {
    eprintln!("see h2_arm_sdk_example.rs for the arm-sdk pattern (h1 switch not exposed yet)");
}
