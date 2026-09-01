# 订阅与发布

## 初始化

所有订阅/发布/RPC 之前，先调用 `init_dds` 初始化 DDS：

- `domain_id`：DDS 域 ID
- `network_interface`：绑定网卡名（如 `"eth0"`，留空用默认）
- `config_file`：CycloneDDS 配置文件路径（留空用默认配置）

## 订阅

`subscribe::<T>(topic)` 订阅话题，返回 `(Subscriber, watch::Receiver<Option<T>>)`：

- `Subscriber` 持有期间持续订阅，释放后自动退订；
- `Receiver` 保存最新一帧，未收到数据时为 `None`，用 `rx.borrow()` 读取。

```rust,no_run
use unitree_sdk2_rs::{init_dds, subscribe};
use unitree_sdk2_rs::unitree_hg::msg::dds_::BmsState;

init_dds(0, "eth0", "");

let (_sub, rx) = subscribe::<BmsState>("rt/lf/bmsstate");

if let Some(state) = rx.borrow().as_ref() {
    println!("SOC={}% 电压={}mV", state.soc, state.bmsvoltage[0]);
}
```

## 发布

`Publisher::<T>::new(topic)` 创建发布器，`publish(&msg)` 发布一帧消息：

```rust,no_run
use unitree_sdk2_rs::{init_dds, Publisher};
use unitree_sdk2_rs::unitree_hg::msg::dds_::BmsState;

init_dds(0, "eth0", "");

let pub_ = Publisher::<BmsState>::new("rt/lf/bmsstate").expect("创建发布器失败");
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

## 消息类型

消息类型按 `包名::msg::dds_::类型名` 访问，与 `msgs/` 下的 `.msg` 文件一一对应，字段与 `.msg` 定义一致：

| .msg 文件 | Rust 类型 |
|---|---|
| `msgs/unitree_hg/msg/BmsState.msg` | `unitree_hg::msg::dds_::BmsState` |
| `msgs/unitree_go/msg/SportModeState.msg` | `unitree_go::msg::dds_::SportModeState` |
| `msgs/std_msgs/msg/String.msg` | `std_msgs::msg::dds_::String` |

话题名（如 `rt/lf/bmsstate`）及各话题的消息格式见 unitree_sdk2 官方文档。
