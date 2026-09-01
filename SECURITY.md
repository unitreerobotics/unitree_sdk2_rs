# Security Policy

## Reporting a Vulnerability

Please **do not** open a public issue for security vulnerabilities.

Instead, report them privately via GitHub's security advisory feature at:

https://github.com/unitreerobotics/unitree_sdk2_rs/security/advisories/new

or by contacting the maintainers directly through the repository.

You should receive a response within a few days. If not, please follow up to
make sure your report was received.

## Scope

- The Rust bindings in `src/`
- The C++ bridge in `cpp/`
- The code generator in `build.rs` and generated `gen/` files

Vulnerabilities in the upstream Unitree SDK2 C++ SDK or CycloneDDS should be
reported to those projects instead.
