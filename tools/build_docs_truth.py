#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Convert the MUST literature into machine-readable docs/ (source of truth)."""

from __future__ import annotations

import re
import shutil
import sys
from pathlib import Path

from bs4 import BeautifulSoup
from pypdf import PdfReader

ROOT = Path(__file__).resolve().parents[1]
DOCS = ROOT / "docs"
RESEARCH = ROOT / "research"
LIT = RESEARCH / "refs" / "literature"


def find_books_dir() -> Path:
    """Find the user books folder without hardcoding non-ASCII in source."""
    search_roots = [RESEARCH, ROOT]
    for base in search_roots:
        if not base.exists():
            continue
        for p in base.iterdir():
            if not p.is_dir():
                continue
            if (p / "ANSI-X3_53-1976_R1998.PDF").exists():
                return p
            for f in p.glob("*.htm*"):
                if "Subset G" in f.name or "X3.74" in f.name:
                    return p
    raise FileNotFoundError("Books folder with ANSI PDFs/HTML not found under research/ or root")


def find_subset_g_html(books: Path) -> Path:
    for f in books.glob("*.htm*"):
        if "Subset" in f.name or "X3.74" in f.name or "X3.74" in f.name:
            return f
    matches = list(books.glob("*Subset*")) + list(books.glob("*X3.74*"))
    if matches:
        return matches[0]
    raise FileNotFoundError("Subset G HTML not found")


def clean_text(s: str) -> str:
    s = s.replace("\x00", "")
    s = s.replace("\r\n", "\n").replace("\r", "\n")
    s = re.sub(r"[ \t]+\n", "\n", s)
    s = re.sub(r"\n{4,}", "\n\n\n", s)
    return s.strip() + "\n"


def pdf_to_md(pdf: Path, out: Path, title: str, role: str, source: str) -> None:
    print(f"PDF -> {out.relative_to(ROOT)} ...", flush=True)
    reader = PdfReader(str(pdf))
    parts: list[str] = [
        f"# {title}\n",
        f"\n> **Role:** {role}\n",
        f"> **Source file:** `{source}`\n",
        f"> **Pages:** {len(reader.pages)}\n",
        f"> **Extraction:** pypdf text layer (not re-OCR)\n",
        "\n---\n",
    ]
    n = len(reader.pages)
    for i, page in enumerate(reader.pages):
        text = (page.extract_text() or "").strip()
        parts.append(f"\n\n## Page {i + 1}\n\n")
        parts.append(text if text else "*(no extractable text on this page)*")
        if (i + 1) % 50 == 0 or (i + 1) == n:
            print(f"  ... page {i + 1}/{n}", flush=True)
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(clean_text("".join(parts)), encoding="utf-8")
    print(f"  wrote {out.stat().st_size:,} bytes", flush=True)


def html_to_md(html_path: Path, out: Path, title: str, role: str, source: str) -> None:
    print(f"HTML -> {out.relative_to(ROOT)} ...", flush=True)
    raw = html_path.read_bytes()
    html = None
    for enc in ("utf-8", "cp1252", "latin-1"):
        try:
            html = raw.decode(enc)
            break
        except UnicodeDecodeError:
            continue
    if html is None:
        html = raw.decode("utf-8", errors="replace")

    soup = BeautifulSoup(html, "lxml")
    for tag in soup(["script", "style"]):
        tag.decompose()
    text = clean_text(soup.get_text("\n"))
    body = (
        f"# {title}\n\n"
        f"> **Role:** {role}\n"
        f"> **Source file:** `{source}`\n"
        f"> **Extraction:** HTML to plain text (BeautifulSoup)\n\n"
        f"---\n\n"
        f"{text}"
    )
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(body, encoding="utf-8")
    print(f"  wrote {out.stat().st_size:,} bytes", flush=True)


def copy_tree(src: Path, dst: Path) -> None:
    if dst.exists():
        shutil.rmtree(dst)
    shutil.copytree(src, dst)
    print(f"copied -> {dst.relative_to(ROOT)}", flush=True)


def resolve_pdf(*candidates: Path) -> Path | None:
    for p in candidates:
        if p.exists():
            return p
    return None


def main() -> int:
    DOCS.mkdir(parents=True, exist_ok=True)
    books = find_books_dir()
    print(f"Books dir: {books}", flush=True)

    subset_html = find_subset_g_html(books)
    html_to_md(
        subset_html,
        DOCS / "standards" / "ANSI-X3.74-1987-Subset-G-Reference.md",
        "PL/I Subset G (ANSI X3.74-1987) Reference Manual",
        "NORTH STAR. Compatibility horizon for the project (Subset G).",
        str(subset_html.relative_to(ROOT)),
    )

    ansi_full = books / "ANSI-X3_53-1976_R1998.PDF"
    jobs = [
        (
            ansi_full,
            DOCS / "standards" / "ANSI-X3.53-1976-R1998-full-PLI.md",
            "ANSI X3.53-1976 (R1998) - Full Programming Language PL/I",
            "Boundary of full standard PL/I (NOT the implementation target). "
            "Use to mark out-of-scope vs Subset G.",
            str(ansi_full.relative_to(ROOT)),
        ),
        (
            resolve_pdf(
                LIT / "GC33-0009-4_PLI_Checkout_And_Opt_Compiler_Lang_Ref_Oct76.pdf",
                LIT / "bitsavers/360/pli/GC33-0009-4_PLI_Checkout_And_Opt_Compiler_Lang_Ref_Oct76.pdf",
            ),
            DOCS / "classic" / "GC33-0009-4-Checkout-Optimizer-LRM-1976.md",
            "GC33-0009-4 - OS PL/I Checkout and Optimizing Compilers: Language Reference (Oct 1976)",
            "PRIMARY classic LRM. Idioms, semantics, keywords-as-names. "
            "Source of truth for classical feel.",
            "research/refs/literature/GC33-0009-4_PLI_Checkout_And_Opt_Compiler_Lang_Ref_Oct76.pdf",
        ),
        (
            resolve_pdf(
                LIT / "SC33-0025-3_PLI_Optimizing_Compiler_Execution_Logic_Sep85.pdf",
                LIT / "bitsavers/370/pli/SC33-0025-3_PLI_Optimizing_Compiler_Execution_Logic_Sep85.pdf",
            ),
            DOCS / "runtime" / "SC33-0025-3-Execution-Logic-1985.md",
            "SC33-0025-3 - OS PL/I Optimizing Compiler: Execution Logic (Sep 1985)",
            "Runtime / storage / library model for our runtime concept and C transpiler lowering.",
            "research/refs/literature/SC33-0025-3_PLI_Optimizing_Compiler_Execution_Logic_Sep85.pdf",
        ),
        (
            resolve_pdf(
                LIT / "SC26-3114-01_PLI_for_MVS_and_VM_1.1_Language_Reference.pdf",
                LIT / "bitsavers/370/pli/SC26-3114-01_PLI_for_MVS_and_VM_Release_1.1_Language_Reference_199506.pdf",
            ),
            DOCS / "classic" / "SC26-3114-01-MVS-VM-1.1-LRM.md",
            "SC26-3114-01 - IBM PL/I for MVS & VM Language Reference Release 1.1",
            "Compact later classic LRM; reference language used by Iron Spring.",
            "research/refs/literature/SC26-3114-01_PLI_for_MVS_and_VM_1.1_Language_Reference.pdf",
        ),
        (
            resolve_pdf(
                LIT / "GC28-8201-4_PLI_F_Language_Reference_Dec72.pdf",
                LIT / "bitsavers/360/pli/GC28-8201-4_PLI_F_Language_Reference_Dec72.pdf",
            ),
            DOCS / "classic" / "GC28-8201-4-PLI-F-LRM-1972.md",
            "GC28-8201-4 - IBM System/360 OS PL/I (F) Language Reference (Dec 1972)",
            "F-level heritage; compare with Optimizer LRM. Secondary classic anchor.",
            "research/refs/literature/GC28-8201-4_PLI_F_Language_Reference_Dec72.pdf",
        ),
    ]

    for pdf, out, title, role, source in jobs:
        if pdf is None or not Path(pdf).exists():
            print(f"MISSING: {title}", file=sys.stderr)
            continue
        pdf_to_md(Path(pdf), out, title, role, source)

    is_guide = LIT / "IronSpring_Programming_Guide.html"
    if is_guide.exists():
        html_to_md(
            is_guide,
            DOCS / "practice" / "IronSpring-Programming-Guide.md",
            "Iron Spring PL/I Compiler Programming Guide",
            "Pragmatic modern subset implementation notes (not normative for our dialect).",
            "research/refs/literature/IronSpring_Programming_Guide.html",
        )

    sg = LIT / "subsetg-book"
    if sg.exists():
        copy_tree(sg, DOCS / "practice" / "subsetg-book")

    index = DOCS / "README.md"
    index.write_text(
        """# docs/ - Source of Truth (machine-readable)

This folder holds the **selected** literature converted to Markdown for
reading, search, and future TZ/SPEC work.

Original PDFs/HTML remain under `research/refs/literature/` and `research/` (books).
If text conflicts with a scanned PDF artifact, prefer the PDF - but for
day-to-day work, **use these `.md` files**.

## Priority (what we selected and why)

| Pri | Document | Path | Why |
|-----|----------|------|-----|
| 1 | **Subset G reference (X3.74-1987)** | `standards/ANSI-X3.74-1987-Subset-G-Reference.md` | North star. Kednos-based readable draft OK as working G bible |
| 2 | **GC33-0009-4 LRM (1976)** | `classic/GC33-0009-4-Checkout-Optimizer-LRM-1976.md` | Classic idioms and keywords-as-names |
| 3 | **SC33-0025-3 Execution Logic** | `runtime/SC33-0025-3-Execution-Logic-1985.md` | Runtime / storage model |
| 4 | **SC26-3114 MVS&VM 1.1** | `classic/SC26-3114-01-MVS-VM-1.1-LRM.md` | Compact later classic LRM |
| 5 | **ANSI X3.53 full PL/I** | `standards/ANSI-X3.53-1976-R1998-full-PLI.md` | Boundary: full vs G / out-of-scope |
| 6 | **GC28-8201-4 PL/I(F)** | `classic/GC28-8201-4-PLI-F-LRM-1972.md` | F-level heritage |
| 7 | **subsetg-book** | `practice/subsetg-book/` | Readable G companion |
| 8 | **Iron Spring guide** | `practice/IronSpring-Programming-Guide.md` | Pragmatic implementer notes |

## Explicitly NOT converted here

- GC33-0009-3 (superseded by -4 for our shelf)
- SC33-0025-2 (PDF kept; -3 is enough in text)
- Primers, textbooks, message books, DOS/PLM deep dives - stay in `research/refs/literature/bitsavers/`
- Enterprise PL/I - out-of-scope map later from IBM Docs

## Project canon

- `../MANIFEST.md` - product decisions
- `../LITERATURE.md` - full shelf policy
- `../STACK.md` - how we build
- `../research/` - raw archives (do not delete)

## Regeneration

```text
python tools/build_docs_truth.py
```
""",
        encoding="utf-8",
    )
    print(f"wrote {index.relative_to(ROOT)}", flush=True)
    print("DONE", flush=True)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
