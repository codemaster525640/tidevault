# Tidevault

Tidevault is a multi-rail payment clearing and double-entry ledger platform for merchants and PSPs.

## Architecture

Hexagonal / domain-oriented Cargo workspace:

- `domains/*` — bounded contexts (ledger, payments, FX, fraud, ...)
- `platform/*` — store and event bus adapters
- `apps/*` — engine orchestration, HTTP API (`tidevault`), CLI (`tvctl`)

## Offline builds

Dependencies are vendored under `vendor/`. `.cargo/config.toml` forces crates.io replacement.

```bash
CARGO_NET_OFFLINE=true cargo build --workspace
CARGO_NET_OFFLINE=true cargo test --workspace
```

## Docker

```bash
docker build -t tidevault:local .
```
