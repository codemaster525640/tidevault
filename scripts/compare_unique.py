from pathlib import Path
import hashlib

def files(root):
    ex = {".git", "target", "vendor", "node_modules"}
    for p in Path(root).rglob("*"):
        if not p.is_file():
            continue
        if any(part in ex for part in p.parts):
            continue
        yield p

def digest(p):
    h = hashlib.sha256()
    with p.open("rb") as f:
        for chunk in iter(lambda: f.read(1 << 20), b""):
            h.update(chunk)
    return h.hexdigest()

a = Path(r"D:\AQ\prometheus\work\gen1\meilisearch")
b = Path(r"D:\AQ\prometheus\work\gen1\tidevault")
map_a = {}
for p in files(a):
    map_a.setdefault(digest(p), []).append(str(p.relative_to(a)))
identical = []
for p in files(b):
    d = digest(p)
    if d in map_a:
        # ignore empty files
        if p.stat().st_size == 0:
            continue
        identical.append((str(p.relative_to(b)), map_a[d]))
print("identical_nonempty", len(identical))
for item in identical[:20]:
    print(item)
