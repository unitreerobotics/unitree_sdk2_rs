# unitree_sdk2_rs

[English](README.md) | [简体中文](README_cn.md)

[![Crates.io](https://img.shields.io/crates/v/unitree_sdk2_rs)](https://crates.io/crates/unitree_sdk2_rs)
[![docs.rs](https://img.shields.io/docsrs/unitree_sdk2_rs)](https://docs.rs/unitree_sdk2_rs)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![CI](https://github.com/Roy-dk/unitree_sdk2_rs/actions/workflows/ci.yml/badge.svg)](https://github.com/Roy-dk/unitree_sdk2_rs/actions/workflows/ci.yml)
[![MSRV: 1.88](https://img.shields.io/badge/rustc-1.88+-lightgray.svg)](Cargo.toml)

Rust bindings for the Unitree SDK2, providing DDS message types, topic publish/subscribe and RPC.

This crate wraps the official [unitree_sdk2](https://github.com/unitreerobotics/unitree_sdk2)
C++ SDK via a `cxx` bridge. It uses the same DDS message types and CDR wire format,
so it interoperates with other unitree_sdk2-based programs on the robot.

- **Message types**: generated from `msgs/*.msg`, one-to-one with the C++ SDK DDS types
- **Subscribe / Publish**: `subscribe::<T>` / `Publisher::<T>`
- **RPC**: `rpc::RpcClient` / `rpc::RpcServer`

## Installation

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
unitree_sdk2_rs = { version = "0.2.0" }
```

The crate ships no feature flags — all functionality is enabled by default.

## Quick Start

```rust,no_run
use unitree_sdk2_rs::{init_dds, subscribe};
use unitree_sdk2_rs::unitree_hg::msg::dds_::BmsState;

// Initialize DDS (domain ID, network interface, config file; the last two can be empty)
init_dds(0, "eth0", "");

// Subscribe to the battery state topic; rx holds the latest frame (None until received)
let (_sub, rx) = subscribe::<BmsState>("rt/lf/bmsstate");
if let Some(s) = rx.borrow().as_ref() {
    println!("SOC = {}%", s.soc);
}
```

More usage (publish, RPC server/client) is in the sections below — embedded in this page on
docs.rs; the same files live in `docs/` in the repo.

- [Subscribe & Publish](#subscribe--publish)
- [RPC (Server / Client)](#rpc-server--client)

Online docs are available on [docs.rs](https://docs.rs/unitree_sdk2_rs).

## Environment Variables

The build fetches the unitree_sdk2 C++ SDK (the repo ships prebuilt libraries; no cmake build).
The source is chosen by priority:

| Variable | Description |
|---|---|
| `UNITREE_SDK2_PATH=<dir>` | Use a local SDK directly (no clone, no build) |
| `UNITREE_SDK2_URL=<git url>` | Shallow-clone into `UNITREE_SDK2_DIR` |
| default | Clone unitree_sdk2 from GitHub |

You can `export` these variables, or set them in `.cargo/config.toml`:

```toml
# relative = true: the value is relative to this config file's directory (project root),
# so cargo resolves it correctly from any directory
[env]
UNITREE_SDK2_PATH = { value = "unitree_sdk2", relative = true }
```

Priority: `UNITREE_SDK2_PATH` > `UNITREE_SDK2_URL` > default GitHub. Clones land in the build
temporary directory (OUT_DIR); the source directory stays untouched.
