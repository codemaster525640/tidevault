# Run after Smart App Control is Off (or inside Docker).
$ErrorActionPreference = "Stop"
Set-Location (Join-Path $PSScriptRoot "..")
$env:CARGO_NET_OFFLINE = "true"
Write-Host "== measure =="
powershell -File .\scripts\measure.ps1
Write-Host "== fmt =="
cargo fmt --all -- --check
Write-Host "== clippy =="
cargo clippy --workspace --all-targets -- -D warnings
Write-Host "== build =="
cargo build --workspace
Write-Host "== test =="
cargo test --workspace -- --test-threads=2
Write-Host "OK"
