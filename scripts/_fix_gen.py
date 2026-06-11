from pathlib import Path

p = Path(r"D:\AQ\prometheus\work\gen1\tidevault\scripts\generate_tidevault.py")
t = p.read_text(encoding="utf-8")
replacements = [
    ('format!("{short}-demo-{{i}}")', 'format!("{short}-demo-{{}}", i)'),
    ('format!("{short}-u-{t}")', 'format!("{short}-u-{t}")'),  # ok: t is python
]
for a, b in replacements:
    if a in t:
        t = t.replace(a, b)
        print("replaced", a[:40])
    else:
        print("missing", a[:40])

# Also convert remaining risky patterns: format!("...{{i}}...") inside f-strings
# Search for {{i}} that aren't part of {{}}
p.write_text(t, encoding="utf-8")
print("done")
