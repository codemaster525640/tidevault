from pathlib import Path
import re

root = Path(r"D:\AQ\prometheus\work\gen1\tidevault\domains")
for path in root.rglob("rules.rs"):
    text = path.read_text(encoding="utf-8")

    def repl(m: re.Match[str]) -> str:
        idx = m.group(1)
        body = m.group(2)
        body = body.replace("{ i as u8 }", idx).replace("{i as u8}", idx)
        return f"fn rule_{idx}_{m.group(0).split('_', 2)[-1].split('(', 1)[0].split(idx + '_', 1)[-1] if False else ''}"

    # Simpler: per-function replace
    def fix_fn(m: re.Match[str]) -> str:
        idx = m.group(1)
        rest = m.group(2)
        rest = rest.replace("{ i as u8 }", idx).replace("{i as u8}", idx)
        return f"fn rule_{idx}_{rest}"

    new, n = re.subn(
        r"fn rule_(\d+)_([A-Za-z]+)\(rec: &([A-Za-z]+)Record\) -> Option<&'static str> \{.*?\n\}",
        lambda m: (
            f"fn rule_{m.group(1)}_{m.group(2)}(rec: &{m.group(3)}Record) -> Option<&'static str> {{"
            + m.group(0).split('{', 1)[1]
            .replace("{ i as u8 }", m.group(1))
            .replace("{i as u8}", m.group(1))
        ),
        text,
        flags=re.S,
    )
    # Even simpler global within file by scanning functions
    parts = []
    idx = None
    for line in text.splitlines(keepends=True):
        m = re.match(r"fn rule_(\d+)_", line)
        if m:
            idx = m.group(1)
        if idx is not None and "{ i as u8 }" in line:
            line = line.replace("{ i as u8 }", idx)
        if idx is not None and "{i as u8}" in line:
            line = line.replace("{i as u8}", idx)
        parts.append(line)
    new_text = "".join(parts)
    if new_text != text:
        path.write_text(new_text, encoding="utf-8")
        print("fixed", path)
