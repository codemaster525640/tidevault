# Remaining host blocker (Docker not required)

Docker verification is **out of scope**.

## What already works offline

- `CARGO_NET_OFFLINE=true cargo build --workspace`
- `CARGO_NET_OFFLINE=true cargo test --workspace --no-run`
- Vendored crates under `vendor/`
- Measured size: ~50k first-party LOC, 382 source files, 145 test files, 60 bug scenarios

## Still blocked by Smart App Control (os error 4551)

Newly compiled executables cannot run, including:

- `cargo test` harness binaries
- `cargo-clippy` from the 1.85 toolchain

**Disable Smart App Control**, reboot, then:

```powershell
cd D:\AQ\prometheus\work\gen1\tidevault
$env:CARGO_NET_OFFLINE='true'
cargo test --workspace -- --test-threads=2
cargo clippy --workspace --all-targets -- -D warnings
```
