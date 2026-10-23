from pathlib import Path
import re

p = Path("runtime/src/put.c")
t = p.read_text(encoding="utf-8")
if "pl1rt_of(" in t:
    print("already patched")
else:
    old = """/* SPDX-License-Identifier: Apache-2.0 */
#include \"pl1rt.h\"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
"""
    new = Path("tools/put_stream_header.c").read_text(encoding="utf-8")
    if old not in t:
        raise SystemExit("header not found")
    t = t.replace(old, new, 1)

t = re.sub(r"\bputchar\(([^)]+)\)", r"fputc(\1, pl1rt_of())", t)
t = t.replace("fputs(s, stdout)", "fputs(s, pl1rt_of())")
t = t.replace(
    "fwrite(s, 1, (size_t)width, stdout)", "fwrite(s, 1, (size_t)width, pl1rt_of())"
)
t = re.sub(r"\bprintf\(", "fprintf(pl1rt_of(), ", t)
t = t.replace("getchar()", "fgetc(pl1rt_if())")
t = re.sub(r"\bscanf\(", "fscanf(pl1rt_if(), ", t)
# avoid double-wrapping if re-run
t = t.replace("fputc(fputc(", "fputc(")  # noop safety
p.write_text(t, encoding="utf-8", newline="\n")
print("ok", p)
