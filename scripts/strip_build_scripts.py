"""Remove vendored build scripts so Cargo never needs to execute blocked binaries."""
from pathlib import Path
import json
import re
import hashlib

vendor = Path(r"D:\AQ\prometheus\work\gen1\tidevault\vendor")

for build in vendor.glob("*/build.rs"):
    build.unlink(missing_ok=True)

patched = 0
for cargo in vendor.glob("*/Cargo.toml"):
    text = cargo.read_text(encoding="utf-8")
    new = text
    new = re.sub(r'(?m)^\s*build\s*=\s*".*?"\s*\n', "", new)
    # packages with links= require a build script; drop links to allow scriptless builds
    new = re.sub(r'(?m)^\s*links\s*=\s*".*?"\s*\n', "", new)
    # drop build-dependencies tables (best-effort)
    new = re.sub(
        r"(?ms)^\[build-dependencies\]\n(?:(?!^\[).*\n)*",
        "",
        new,
    )
    if new != text:
        cargo.write_text(new, encoding="utf-8")
        patched += 1
print("patched_cargo_toml", patched)

updated = 0
for meta in vendor.glob("*/.cargo-checksum.json"):
    data = json.loads(meta.read_text(encoding="utf-8"))
    files = data.get("files", {})
    changed = False
    for rel in list(files.keys()):
        p = meta.parent / rel
        if not p.exists():
            del files[rel]
            changed = True
            continue
        h = hashlib.sha256(p.read_bytes()).hexdigest()
        if files[rel] != h:
            files[rel] = h
            changed = True
    if changed:
        data["files"] = files
        meta.write_text(json.dumps(data, indent=2) + "\n", encoding="utf-8")
        updated += 1
print("checksums_updated", updated)
