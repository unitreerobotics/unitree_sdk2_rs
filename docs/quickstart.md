# Subscribe & Publish

## Initialization

Call `init_dds` once before any subscribe/publish/RPC operation:

- `domain_id`: DDS domain ID
- `network_interface`: network interface to bind (e.g. `"eth0"`, empty = default)
- `config_file`: CycloneDDS config file path (empty = default config)

## Subscribe

`subscribe::<T>(topic)` subscribes to a topic and returns `(Subscriber, watch::Receiver<Option<T>>)`:

- the `Subscriber` keeps the subscription alive while held and unsubscribes on drop;
- the `Receiver` holds the latest message (`None` until the first one arrives); read it with `rx.borrow()`.

```rust,no_run
use unitree_sdk2_rs::{init_dds, subscribe};
use unitree_sdk2_rs::unitree_hg::msg::dds_::BmsState;

init_dds(0, "eth0", "");

let (_sub, rx) = subscribe::<BmsState>("rt/lf/bmsstate");

if let Some(state) = rx.borrow().as_ref() {
    println!("SOC={}% voltage={}mV", state.soc, state.bmsvoltage[0]);
}
```

## Publish

Create a publisher with `Publisher::<T>::new(topic)` and publish one message with `publish(&msg)`:

```rust,no_run
use unitree_sdk2_rs::{init_dds, Publisher};
use unitree_sdk2_rs::unitree_hg::msg::dds_::BmsState;

init_dds(0, "eth0", "");

let pub_ = Publisher::<BmsState>::new("rt/lf/bmsstate").expect("failed to create publisher");
let bms = BmsState {
    version_high: 1, version_low: 0, r#fn: 0,
    cell_vol: [4000; 40],
    bmsvoltage: [42000, 0, 0],
    current: -2500, soc: 85, soh: 100,
    temperature: [32; 12],
    cycle: 1, manufacturer_date: 0,
    bmsstate: [0; 5], reserve: [0; 3],
};
pub_.publish(&bms);
```

## Message Types

Message types are accessed as `<package>::msg::dds_::<TypeName>`, one-to-one with the `.msg`
files under `msgs/`; fields match the `.msg` definitions:

| .msg file | Rust type |
|---|---|
| `msgs/unitree_hg/msg/BmsState.msg` | `unitree_hg::msg::dds_::BmsState` |
| `msgs/unitree_go/msg/SportModeState.msg` | `unitree_go::msg::dds_::SportModeState` |
| `msgs/unitree_api/msg/Request.msg` | `unitree_api::msg::dds_::Request` |
| `msgs/std_msgs/msg/String.msg` | `std_msgs::msg::dds_::String` |

Topic names (e.g. `rt/lf/bmsstate`) and their message formats follow the unitree_sdk2 official docs.
