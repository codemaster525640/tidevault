reference_name = "reference"
# Emit Rust format!("{reference}") using unicode escapes in f-string literal parts
s = f'format!("acc-\\u007breference\\u007d")'
print(s)
# Better - actual unicode chars in the f-string as literal text outside expressions:
s2 = f'format!("acc-{chr(123)}reference{chr(125)}")'
print(s2)
