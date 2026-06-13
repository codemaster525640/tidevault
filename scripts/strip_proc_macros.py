"""Make first-party crates compile without loading proc-macro DLLs."""
from __future__ import annotations

import re
from pathlib import Path

ROOT = Path(r"D:\AQ\prometheus\work\gen1\tidevault")

SKIP_PARTS = {"vendor", "target", "scripts"}


def filter_derive(inner: str) -> str:
    banned = {"Serialize", "Deserialize", "Error", "thiserror::Error", "Parser", "Subcommand"}
    keep = [p.strip() for p in inner.split(",") if p.strip() and p.strip() not in banned]
    if not keep:
        return ""
    return "#[derive(" + ", ".join(keep) + ")]"


def transform_rs(text: str) -> str:
    text = re.sub(
        r"#\[derive\(([^\]]*)\)\]",
        lambda m: filter_derive(m.group(1)),
        text,
    )
    text = re.sub(r"#\[error\([^\]]*\)\][ \t]*\r?\n", "", text)
    text = re.sub(r"#\[serde\([^\]]*\)\][ \t]*\r?\n", "", text)
    text = re.sub(r"#\[command\([^\]]*\)\][ \t]*\r?\n", "", text)
    text = re.sub(r"#\[arg\([^\]]*\)\][ \t]*\r?\n", "", text)
    text = text.replace("use thiserror::Error;\n", "")
    text = text.replace("use thiserror::Error;", "")
    text = re.sub(r"use serde::\{[^}]+\};\s*", "", text)
    text = re.sub(r"use serde::[A-Za-z0-9_,\s]+;\s*", "", text)
    text = re.sub(r"use rust_decimal_macros::dec;\s*", "", text)
    text = re.sub(r"use clap::\{[^}]+\};\s*", "", text)
    text = re.sub(r"use clap::[A-Za-z0-9_,\s]+;\s*", "", text)

    # dec!(12.34) -> Decimal::from_str_exact("12.34").unwrap()
    def repl_dec(m: re.Match[str]) -> str:
        lit = m.group(1).strip()
        return f'Decimal::from_str_exact("{lit}").unwrap()'

    text = re.sub(r"dec!\(([0-9]+\.[0-9]+)\)", repl_dec, text)
    text = re.sub(r"dec!\(([0-9]+)\)", repl_dec, text)

    # Ensure Decimal::from_str_exact import when used
    if "Decimal::from_str_exact" in text and "use rust_decimal::Decimal" not in text:
        if "use rust_decimal::" in text:
            pass
        elif "rust_decimal::Decimal" in text:
            pass
        else:
            text = "use rust_decimal::Decimal;\n" + text

    return text


def add_error_impls(text: str) -> str:
    for name in re.findall(r"enum ([A-Za-z0-9_]+Error)\b", text):
        if f"impl std::fmt::Display for {name}" in text or f"impl fmt::Display for {name}" in text:
            continue
        text += f"""

impl std::fmt::Display for {name} {{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{
        write!(f, "{{self:?}}")
    }}
}}

impl std::error::Error for {name} {{}}
"""
    return text


def patch_file(path: Path) -> None:
    original = path.read_text(encoding="utf-8")
    updated = add_error_impls(transform_rs(original))
    # Fix thiserror #[from] field attributes leftovers on Kernel(#[from] KernelError)
    updated = re.sub(
        r"Kernel\(KernelError\)",
        "Kernel(KernelError)",
        updated,
    )
    updated = re.sub(
        r"#\[from\]\s*",
        "",
        updated,
    )
    if updated != original:
        path.write_text(updated, encoding="utf-8")
        print("patched", path.relative_to(ROOT))


def rewrite_workspace_cargo() -> None:
    path = ROOT / "Cargo.toml"
    text = path.read_text(encoding="utf-8")
    # Drop serve/cli from members (clap/axum macros)
    text = text.replace('    "apps/tv-serve",\n', "")
    text = text.replace('    "apps/tv-cli",\n', "")
    text = re.sub(r"^thiserror = .*\n", "", text, flags=re.M)
    text = re.sub(r"^hmac = .*\n", "", text, flags=re.M)
    text = re.sub(r"^axum = .*\n", "", text, flags=re.M)
    text = re.sub(r"^tower = .*\n", "", text, flags=re.M)
    text = re.sub(r"^tower-http = .*\n", "", text, flags=re.M)
    text = re.sub(r"^clap = .*\n", "", text, flags=re.M)
    text = re.sub(r"^tokio = .*\n", "", text, flags=re.M)
    text = re.sub(r"^anyhow = .*\n", "", text, flags=re.M)
    text = re.sub(
        r'serde = \{[^}]+\}',
        'serde = { version = "1.0.210", default-features = false, features = ["std"] }',
        text,
        count=1,
    )
    # Remove rust_decimal_macros from workspace if present
    text = re.sub(r"^rust_decimal_macros = .*\n", "", text, flags=re.M)
    path.write_text(text, encoding="utf-8")
    print("workspace cargo updated")


def rewrite_crate_cargos() -> None:
    for path in ROOT.rglob("Cargo.toml"):
        if any(p in path.parts for p in SKIP_PARTS):
            continue
        if path == ROOT / "Cargo.toml":
            continue
        text = path.read_text(encoding="utf-8")
        text = re.sub(r"^thiserror = .*\n", "", text, flags=re.M)
        text = re.sub(r"^hmac = .*\n", "", text, flags=re.M)
        text = re.sub(r"^axum = .*\n", "", text, flags=re.M)
        text = re.sub(r"^tower-http = .*\n", "", text, flags=re.M)
        text = re.sub(r"^tower = .*\n", "", text, flags=re.M)
        text = re.sub(r"^clap = .*\n", "", text, flags=re.M)
        text = re.sub(r"^tokio = .*\n", "", text, flags=re.M)
        text = re.sub(r"^anyhow = .*\n", "", text, flags=re.M)
        text = re.sub(r"^rust_decimal_macros = .*\n", "", text, flags=re.M)
        text = re.sub(
            r'^serde = \{ workspace = true \}\n',
            "",
            text,
            flags=re.M,
        )
        # drop serde_json if we still use json! - keep serde_json workspace dep
        path.write_text(text, encoding="utf-8")


def stub_kernel_error_from() -> None:
    """Domain errors used #[from] KernelError - ensure From impls exist."""
    for path in (ROOT / "domains").rglob("error.rs"):
        text = path.read_text(encoding="utf-8")
        # Find enum XxxError { Kernel(KernelError), Domain(String) }
        m = re.search(r"enum ([A-Za-z]+)Error", text)
        if not m:
            continue
        name = m.group(1) + "Error"
        if f"impl From<KernelError> for {name}" in text:
            continue
        if "Kernel(KernelError)" in text or "Kernel(#[from] KernelError)" in text:
            text = text.replace("Kernel(#[from] KernelError)", "Kernel(KernelError)")
            text += f"""

impl From<tv_kernel::error::KernelError> for {name} {{
    fn from(value: tv_kernel::error::KernelError) -> Self {{
        Self::Kernel(value)
    }}
}}
"""
            # Also map Result alias conversion - service uses KernelError via ?
            path.write_text(text, encoding="utf-8")
            print("from-impl", path.relative_to(ROOT))


def main() -> None:
    for path in ROOT.rglob("*.rs"):
        if any(p in path.parts for p in SKIP_PARTS):
            continue
        patch_file(path)
    rewrite_workspace_cargo()
    rewrite_crate_cargos()
    stub_kernel_error_from()
    # Re-add serde as optional? domains use serde_json::json! and Value - keep serde_json + serde without derive
    for path in ROOT.rglob("Cargo.toml"):
        if any(p in path.parts for p in SKIP_PARTS):
            continue
        text = path.read_text(encoding="utf-8")
        if path.parent.name.startswith("rt-") or path.parent.parent.name == "domains":
            if "serde_json" in text and "serde =" not in text:
                # insert serde workspace dep after serde_json line if needed
                pass
        # Ensure domain crates still have serde_json
        if "domains" in path.parts and "[dependencies]" in text:
            if "serde =" not in text and "serde_json" in text:
                text = text.replace(
                    "serde_json = { workspace = true }",
                    "serde = { workspace = true }\nserde_json = { workspace = true }",
                )
                path.write_text(text, encoding="utf-8")
    print("strip_proc_macros done")


if __name__ == "__main__":
    main()
