from pathlib import Path
import json
import hashlib

vendor = Path(r"D:\AQ\prometheus\work\gen1\tidevault\vendor")
updated = 0
for meta in vendor.glob("*/.cargo-checksum.json"):
    data = json.loads(meta.read_text(encoding="utf-8"))
    files = data.get("files", {})
    changed = False
    for rel, old in list(files.items()):
        p = meta.parent / rel
        if not p.exists():
            continue
        h = hashlib.sha256(p.read_bytes()).hexdigest()
        if h != old:
            files[rel] = h
            changed = True
    if changed:
        data["files"] = files
        meta.write_text(json.dumps(data, indent=2) + "\n", encoding="utf-8")
        updated += 1
print("updated", updated)
