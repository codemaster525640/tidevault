"""Strip vendored build scripts and patch OUT_DIR includes for App Control hosts."""
from pathlib import Path
import hashlib
import json
import re

ROOT = Path(r"D:\AQ\prometheus\work\gen1\tidevault")
VENDOR = ROOT / "vendor"


def refresh_checksums() -> None:
    for meta in VENDOR.glob("*/.cargo-checksum.json"):
        data = json.loads(meta.read_text(encoding="utf-8"))
        files = data.get("files", {})
        changed = False
        for rel, old in list(files.items()):
            path = meta.parent / rel
            if not path.exists():
                del files[rel]
                changed = True
                continue
            digest = hashlib.sha256(path.read_bytes()).hexdigest()
            if digest != old:
                files[rel] = digest
                changed = True
        if changed:
            data["files"] = files
            meta.write_text(json.dumps(data, indent=2) + "\n", encoding="utf-8")


def strip_build_scripts() -> None:
    for build in VENDOR.glob("*/build.rs"):
        build.unlink(missing_ok=True)
    for cargo in VENDOR.glob("*/Cargo.toml"):
        text = cargo.read_text(encoding="utf-8")
        new = re.sub(r'(?m)^\s*build\s*=\s*".*?"\s*\n', "", text)
        new = re.sub(r'(?m)^\s*links\s*=\s*".*?"\s*\n', "", new)
        new = re.sub(r"(?ms)^\[build-dependencies\]\n(?:(?!^\[).*\n)*", "", new)
        if new != text:
            cargo.write_text(new, encoding="utf-8")


PRIVATE = """#[doc(hidden)]
        pub mod __private229 {
            #[doc(hidden)]
            pub use crate::private::*;
        }"""

SERDE_PRIVATE = """#[doc(hidden)]
        pub mod __private229 {
            #[doc(hidden)]
            pub use crate::private::*;
        }
        use serde_core::__private229 as serde_core_private;"""


def patch_outdir() -> None:
    include_private = 'include!(concat!(env!("OUT_DIR"), "/private.rs"));'

    core = VENDOR / "serde_core" / "src" / "crate_root.rs"
    if core.exists():
        text = core.read_text(encoding="utf-8")
        if include_private in text:
            core.write_text(text.replace(include_private, PRIVATE), encoding="utf-8")
            print("patched serde_core")

    serde_lib = VENDOR / "serde" / "src" / "lib.rs"
    if serde_lib.exists():
        text = serde_lib.read_text(encoding="utf-8")
        if include_private in text:
            serde_lib.write_text(text.replace(include_private, SERDE_PRIVATE), encoding="utf-8")
            print("patched serde lib include")
        elif "// outdir stripped" in text:
            serde_lib.write_text(text.replace("// outdir stripped", SERDE_PRIVATE), encoding="utf-8")
            print("patched serde lib stripped")

    serde_core_root = VENDOR / "serde" / "src" / "core" / "crate_root.rs"
    if serde_core_root.exists():
        text = serde_core_root.read_text(encoding="utf-8")
        if include_private in text:
            serde_core_root.write_text(text.replace(include_private, PRIVATE), encoding="utf-8")
            print("patched serde/core/crate_root")

    for rel in ["rustversion/src/lib.rs", "rust_decimal/src/lib.rs"]:
        path = VENDOR / rel
        if not path.exists():
            continue
        text = path.read_text(encoding="utf-8")
        text2, n = re.subn(
            r'include!\(concat!\(env!\("OUT_DIR"\),\s*"/[^"]+"\)\);',
            "// outdir stripped",
            text,
        )
        text2 = text2.replace(
            '#![doc = include_str!(concat!(env!("OUT_DIR"), "/README-lib.md"))]',
            '#![doc = "rust_decimal"]',
        )
        if text2 != text:
            path.write_text(text2, encoding="utf-8")
            print("patched", rel, "subs", n)


def write_config() -> None:
    (ROOT / ".cargo" / "config.toml").write_text(
        """[source.crates-io]
replace-with = "vendored-sources"

[source.vendored-sources]
directory = "vendor"

[build]
rustflags = [
  "--cfg", "procmacro2_semver_exempt",
  "--cfg", "fast_arithmetic=\\"64\\"",
]
""",
        encoding="utf-8",
    )


def main() -> None:
    strip_build_scripts()
    patch_outdir()
    refresh_checksums()
    write_config()
    print("prepare_offline_build done")


if __name__ == "__main__":
    main()
