# Misc SDK examples — sync notes

This file records what happened to the "misc" directories of the C++ SDK
(`tmp_sdk2/example/`), evaluated for a Rust port into `examples/`.
The rule for this crate: every Rust example mirrors a C++ example one-to-one
where the public API allows it, and is documented here when it cannot.

| SDK dir | C++ content | Rust status | Output |
|---|---|---|---|
| `helloworld/` | pub/sub of a *custom* DDS type (`HelloWorldData::Msg`) | Equivalent demo on the same topic with `std_msgs` String | `helloworld_pub.rs`, `helloworld_sub.rs` |
| `jsonize/` | internal JSON serialization round-trip test | Not portable (C++ internal utility, no DDS/Robot API) | — |
| `state_machine/` | whole-robot low-level state-machine controller framework | Not portable (hardware/model-coupled internals) | — |
| `wireless_controller/` | gamepad receiver demo on `rt/wirelesscontroller` | Ported | `wireless_controller.rs` |

## helloworld/ — ported as an equivalent pub/sub demo

`publisher.cpp` / `subscriber.cpp` demonstrate plain pub/sub on topic
`TopicHelloWorld` with a hand-written CycloneDDS IDL message:

```cpp
module HelloWorldData { struct Msg {
    int64  userID;   // publisher fills it with GetCurrentTimeMillisecond()
    string message;  // "HelloWorld."
}; };
```

Rust message types in this crate are generated *only* from `msgs/*.msg` by
`build.rs`, so a bespoke `HelloWorldData::Msg` has no representation and a
`Publisher::<T>`/`subscribe::<T>` pair for it cannot be instantiated.

`helloworld_pub.rs` / `helloworld_sub.rs` are the closest equivalent: the same
periodic pub (1 Hz) and the same topic name, using the generated
`std_msgs::msg::dds_::String`, with the two original fields packed into the
single `data` string as `"<userID>|<message>"`. The subscriber also mirrors the
C++ channel lifecycle (open → close → reopen …) via the subscription handle's
drop semantics.

> Interop caveat (noted in both files): DDS matches samples by registered type
> name, so the pair speaks `std_msgs::msg::dds_::String_`, *not*
> `HelloWorldData::Msg`. It interoperates with peers using the String type on
> that topic, not with the original C++ HelloWorldData endpoints.

## jsonize/ — not portable

`test_jsonize.cpp` is not a DDS/Robot example at all: it round-trips a `Test`
struct (`x/y/z` ints) and a `std::map<std::string,std::string>` through the
C++-internal `unitree/common/json/jsonize.hpp`
(`ToJsonString`/`FromJsonString`).

There is no Rust counterpart for that internal serializer; JSON in this crate
is handled by `serde`/`serde_json` (already a dependency, used for the RPC
request/response payloads). A Rust equivalent of this *test* would be a unit
test deriving `Serialize`/`Deserialize`, which has nothing to do with the SDK
API — so nothing was ported.

## state_machine/ — not portable (reasons)

`state_machine/` is a full low-level whole-robot controller framework rather
than a feature demo:

- `RobotController<UserCtrl>` template drives `rt/lowcmd` / `rt/lowstate`
  directly, with a custom PD position controller supplied by a user template
  class (`ExampleUserController`) plus `params/params.json` (dt / kp / kd /
  12-joint `init_pos`) — i.e. model- and hardware-config-specific.
- Gamepad input is parsed from the *legacy 40-byte `wireless_remote` block
  inside `LowState`* (`comm.h` REMOTE_DATA_RX), not from the
  `rt/wirelesscontroller` DDS topic used by `wireless_controller/`.
- It embeds its own state machine (DAMPING → STAND → CTRL with PD-ratio
  ramps), an upside-down termination check from projected gravity, logging,
  and a raw memory-layout CRC32 over `LowCmd` (`robot_interface.hpp` uses
  `crc32_core((uint32_t*)&low_cmd, ...)`, which the Rust side cannot reproduce
  byte-identically — see the CRC note in `go2_low_level.rs`).

Porting it faithfully would mean porting a robot-specific stance controller
with no public high-level API involved — the Rust crate deliberately keeps
examples to the public API surface. Suggested alternatives:

- High-level behaviors: use the generated robot API clients instead of a raw
  low-level loop — e.g. `robot_api::go2::sport::SportClient`
  (quadrupeds go2/a2/as2), or `robot_api::{g1,h1,h2,r1}::loco::LocoClient`
  (humanoids) — while subscribing to the matching high state
  (`rt/sportmodestate`, `rt/lf/lowstate`, …).
- Low-level channel mechanics: already demonstrated in
  `examples/go2_low_level.rs` (rt/lowcmd + rt/lowstate).
- Button/stick input in a Rust state machine can be built on
  `WirelessController` samples (see `wireless_controller.rs` for the decode +
  edge/state helpers).

## wireless_controller/ — ported

`main.cpp` subscribes to `rt/wirelesscontroller` (type
`unitree_go::msg::dds_::WirelessController_`, which *does* exist in
`msgs/unitree_go/msg/WirelessController.msg` → generated as
`unitree_sdk2_rs::unitree_go::msg::dds_::WirelessController` with
`lx/ly/rx/ry: f32` + `keys: u16`) and `advanced_gamepad.hpp` adds filtering:

- dead-zone + exponential smoothing per stick,
- each `keys` bit → a `Button` with `pressed` / `on_press` / `on_release`
  edge flags,
- a 40 ms work loop printing smoothed lx and the A-button state (counting
  presses).

`wireless_controller.rs` ports all of it: same topic, same smoothing/dead-zone
defaults used by `main.cpp` (0.2 / 0.5), the 16-bit `keys` layout following the
C++ `xKeySwitchUnion` bitfield order (bit 0 = R1 … bit 15 = left, LSB-first on
little-endian), and a 25 Hz loop. Run it on a machine that can see the robot's
wireless receiver samples (e.g. the operator PC or the robot itself).

## g1 additional notes

The following SDK g1 examples have no Rust port yet (SDK-private DDS/hardware
interfaces, no robot_api client backing):

- `g1/high_level/g1_arm5_sdk_dds_example.cpp` / `g1_arm7_sdk_dds_example.cpp` /
  `g1_hand_sdk_example.cpp` — external arm/hand SDK via dedicated DDS channels
  (see the arm-sdk notes under h1/h2/r1)
- `g1/high_level/g1_userctrl_dds_example.cpp` — user-control mode via `rt/userctrl`
  LowCmd channel (low-level pattern in examples/go2_low_level.rs)
- `g1/dex3/g1_dex3_example.cpp` — dexterous-hand hardware demo
- `g1/low_level/terminations.cpp` — safety-termination helper library, not a demo

Ported g1 demos: `g1/g1d/*` (agv, height control, arm), `g1/high_level/*`
(loco / arm action / audio), `g1/low_level/*` (ankle swing, dual arm).
