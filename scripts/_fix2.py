from pathlib import Path
import py_compile
import re

p = Path(r"D:\AQ\prometheus\work\gen1\tidevault\scripts\generate_tidevault.py")
lines = p.read_text(encoding="utf-8").splitlines()
print("1974", repr(lines[1973]))
print("codes", [ord(c) for c in lines[1973][lines[1973].find('demo'):]])

# Manual fix of this specific line: write characters that are valid in f-string
# Desired source text: format!("{short}-demo-{chr(123)}i{chr(125)}", ) NO - use chr expr
# Desired: let rec = ... format!("{short}-demo-{chr(123)}i{chr(125)}")?;
lines[1973] = (
    "                    let rec = self.create(merchant, amount, "
    'format!("{short}-demo-{chr(123)}i{chr(125)}"))?;'
)
print("new", repr(lines[1973]))
p.write_text("\n".join(lines) + "\n", encoding="utf-8")

# Replace ANY remaining {{ident}} using character-level scan
text = p.read_text(encoding="utf-8")
pattern = re.compile(r"\{\{([A-Za-z_][A-Za-z0-9_]*)\}\}")
print("remaining matches", pattern.findall(text)[:20], "count", len(pattern.findall(text)))

def repl(mo):
    return "{chr(123)}" + mo.group(1) + "{chr(125)}"

text2, n = pattern.subn(repl, text)
print("subn", n)
p.write_text(text2, encoding="utf-8")

try:
    py_compile.compile(str(p), doraise=True)
    print("COMPILE_OK")
except py_compile.PyCompileError as e:
    print("FAIL", e)
