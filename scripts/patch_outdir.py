from pathlib import Path
import json
import hashlib
import re

root = Path(r"D:\AQ\prometheus\work\gen1\tidevault\vendor")

for rel in ["serde/src/core/crate_root.rs", "serde_core/src/crate_root.rs"]:
    p = root / rel
    if not p.exists():
        print("missing", rel)
        continue
    t = p.read_text(encoding="utf-8")
    old = 'include!(concat!(env!("OUT_DIR"), "/private.rs"));'
    new = """#[doc(hidden)]
        pub mod __private229 {
            #[doc(hidden)]
            pub use crate::private::*;
        }"""
    if old in t:
        p.write_text(t.replace(old, new), encoding="utf-8")
        print("patched", rel)
    else:
        print("no private pattern", rel)

for rel in ["rustversion/src/lib.rs", "rust_decimal/src/lib.rs", "serde/src/lib.rs"]:
    p = root / rel
    if not p.exists():
        print("missing", rel)
        continue
    t = p.read_text(encoding="utf-8")
    t2, n = re.subn(
        r'include!\(concat!\(env!\("OUT_DIR"\),\s*"/[^"]+"\)\);',
        "// outdir stripped",
        t,
    )
    if n:
        p.write_text(t2, encoding="utf-8")
        print("stripped", rel, n)
    else:
        print("no include", rel)

# checksum refresh
for meta in root.glob("*/.cargo-checksum.json"):
    data = json.loads(meta.read_text(encoding="utf-8"))
    files = data.get("files", {})
    ch = False
    for rel, oldh in list(files.items()):
        fp = meta.parent / rel
        if not fp.exists():
            del files[rel]
            ch = True
            continue
        h = hashlib.sha256(fp.read_bytes()).hexdigest()
        if h != oldh:
            files[rel] = h
            ch = True
    if ch:
        data["files"] = files
        meta.write_text(json.dumps(data, indent=2) + "\n", encoding="utf-8")
print("checksums refreshed")
