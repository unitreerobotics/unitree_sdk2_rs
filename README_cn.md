# unitree_sdk2_rs

[English](README.md) | [简体中文](README_cn.md)

[![Crates.io](https://img.shields.io/crates/v/unitree_sdk2_rs)](https://crates.io/crates/unitree_sdk2_rs)
[![docs.rs](https://img.shields.io/docsrs/unitree_sdk2_rs)](https://docs.rs/unitree_sdk2_rs)
[![License: BSD-3-Clause](https://img.shields.io/badge/license-BSD--3--Clause-blue.svg)](LICENSE)
[![CI](https://github.com/unitreerobotics/unitree_sdk2_rs/actions/workflows/ci.yml/badge.svg)](https://github.com/unitreerobotics/unitree_sdk2_rs/actions/workflows/ci.yml)
[![MSRV: 1.88](https://img.shields.io/badge/rustc-1.88+-lightgray.svg)](Cargo.toml)

Unitree SDK2 的 Rust 绑定，提供 DDS 消息类型、话题订阅/发布与 RPC 接口。

本项目通过 `cxx` 桥接封装官方 [unitree_sdk2](https://github.com/unitreerobotics/unitree_sdk2)
C++ SDK，复用相同的 DDS 消息类型与 CDR 线格式，可与机器人上其他基于 unitree_sdk2 的程序互通。

- **消息类型**：由 `msgs/*.msg` 自动生成，与 C++ SDK 的 DDS 类型一一对应
- **订阅/发布**：`subscribe::<T>` / `Publisher::<T>` 泛型接口
- **RPC**：`rpc::RpcClient` / `rpc::RpcServer`

## 安装

在 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
unitree_sdk2_rs = { version = "0.2.0" }
```

该 crate 无 feature 开关，默认即启用全部功能。

## 快速上手

```rust,no_run
use unitree_sdk2_rs::{init_dds, subscribe};
use unitree_sdk2_rs::unitree_hg::msg::dds_::BmsState;

// 初始化 DDS（域 ID、网卡、配置文件，后两项可留空）
init_dds(0, "eth0", "");

// 订阅电池状态话题，rx 保存最新一帧（未收到时为 None）
let (_sub, rx) = subscribe::<BmsState>("rt/lf/bmsstate");
if let Some(s) = rx.borrow().as_ref() {
    println!("SOC = {}%", s.soc);
}
```

更多用法（发布、RPC 服务端/客户端）见下方章节（docs.rs 本页已内嵌，对应文件在仓库 `docs/` 目录）：

- [订阅与发布](#订阅与发布)
- [RPC（服务端/客户端）](#rpc服务端--客户端)

在线文档可在 docs.rs 查看。

## 环境变量

构建时获取 unitree_sdk2 C++ SDK（仓库自带预编译库，不做 cmake 编译）。来源按优先级：

| 环境变量 | 说明 |
|---|---|
| `UNITREE_SDK2_PATH=<目录>` | 直接用本地已有 SDK（不 clone、不编译） |
| `UNITREE_SDK2_URL=<git 地址>` | 浅克隆到 `UNITREE_SDK2_DIR` |
| 默认 | 从 GitHub 克隆 unitree_sdk2 |

您可直接 `export` 设置环境变量，也可在 `.cargo/config.toml` 下添加，示例如下：

```toml
# relative = true：值相对于本配置文件所在目录（项目根），任何目录下运行 cargo 都解析正确
[env]
UNITREE_SDK2_PATH = { value = "unitree_sdk2", relative = true }
```

优先级：`UNITREE_SDK2_PATH` > `UNITREE_SDK2_URL` > 默认 GitHub。克隆目录统一在构建临时目录（OUT_DIR）下，源目录零写入。
