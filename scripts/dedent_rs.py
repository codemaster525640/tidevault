from pathlib import Path
import textwrap

root = Path(r"D:\AQ\prometheus\work\gen1\tidevault")
n = 0
for p in root.rglob("*.rs"):
    if "target" in p.parts or "vendor" in p.parts:
        continue
    t = p.read_text(encoding="utf-8")
    d = textwrap.dedent(t)
    if d != t:
        p.write_text(d, encoding="utf-8")
        n += 1
print("dedented", n)
