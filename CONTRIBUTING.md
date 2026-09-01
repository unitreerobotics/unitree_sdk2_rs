# Contributing

Thanks for your interest in contributing to `unitree_sdk2_rs`!

## Development setup

This crate binds the C++ Unitree SDK2 via `cxx`. On Linux the full C++ bridge is
compiled and linked; on macOS the build is type-check only (C++ is skipped).

### Linux (recommended)

```bash
# The SDK is fetched automatically (default: clone from GitHub during build).
cargo build --all-targets
cargo test --all-targets
```

To use a local SDK checkout instead of cloning:

```bash
export UNITREE_SDK2_PATH=/path/to/unitree_sdk2
```

### macOS

`cargo build` works for type-checking the library, but linking executables/tests
requires the C++ bridge, so run the full checks in Docker/Linux:

```bash
./build-in-docker.sh   # builds and packages install/ via build.sh
```

## Code style

- Run `cargo fmt --check -- --config skip_children=true` and
  `cargo clippy --all-targets -- -D warnings` before pushing; both must pass.
  (`skip_children` keeps rustfmt from recursing into the generated `gen/` files.)
- Public API changes must keep the existing API surface stable (semver).
- If you change `msgs/*.msg` or `build.rs`, generated files under `gen/` are
  regenerated automatically by the build script — commit them too.
- Update documentation when changing behavior:
  - `README.md` (English) and `README_cn.md` (中文)
  - `docs/*.md` (English) and `docs/*_cn.md` (中文)

## Tests

- `tests/bms_test.rs` — BMS pub/sub loopback on the robot (or DDS network)
- `tests/rpc_test_server.rs` / `tests/rpc_test_client.rs` — RPC round trip

## Submitting changes

1. Fork the repository and create a feature branch.
2. Make your changes, with tests/docs as appropriate.
3. Ensure `fmt`, `clippy` and the CI workflow pass.
4. Open a pull request describing the change.

## License

By contributing, you agree that your contributions will be licensed under the
[MIT License](LICENSE).
