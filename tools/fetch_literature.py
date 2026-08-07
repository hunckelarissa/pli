#!/usr/bin/env python3
"""
Download OPEN literature for the PL/I compiler concept shelf.

Downloads ONLY from public archival / vendor documentation URLs:
  - Bitsavers (historical IBM manuals)
  - Iron Spring (vendor docs they publish)
  - GitHub (subsetg-book)

Does NOT download:
  - ANSI / ISO standards (paid; obtain legally yourself)
  - ACM paywalled papers
  - Scans from pirate / shadow libraries

Usage (from repo root f:\\proj\\pli):
  python tools/fetch_literature.py
  python tools/fetch_literature.py --dry-run
  python tools/fetch_literature.py --only core
"""

from __future__ import annotations

import argparse
import hashlib
import ssl
import sys
import urllib.error
import urllib.request
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / "research" / "refs" / "literature"
MANIFEST_OUT = OUT / "DOWNLOAD_MANIFEST.txt"

# BitSavers mirrors (try in order)
BITSAVERS = [
    "https://bitsavers.org/pdf",
    "https://bitsavers.trailing-edge.com/pdf",
]

# ---------------------------------------------------------------------------
# Catalog: id -> (category, relative_or_absolute_url, local_name, notes)
# category: core | classic | runtime | practice | optional
# URLs starting with http are absolute; others are appended to Bitsavers base.
# ---------------------------------------------------------------------------

ITEMS: list[dict] = [
    # --- CORE classical LRM ---
    {
        "id": "gc33-0009-4",
        "cat": "core",
        "path": "ibm/360/pli/GC33-0009-4_PLI_Checkout_And_Opt_Compiler_Lang_Ref_Oct76.pdf",
        "file": "GC33-0009-4_PLI_Checkout_And_Opt_Compiler_Lang_Ref_Oct76.pdf",
        "note": "Primary classic LRM (Checkout+Optimizer), Oct 1976",
    },
    {
        "id": "gc28-8201-4",
        "cat": "core",
        "path": "ibm/360/pli/GC28-8201-4_PLI_F_Language_Reference_Dec72.pdf",
        "file": "GC28-8201-4_PLI_F_Language_Reference_Dec72.pdf",
        "note": "PL/I(F) Language Reference",
    },
    {
        "id": "sc26-3114-01",
        "cat": "core",
        "path": "ibm/370/pli/SC26-3114-01_PLI_for_MVS_and_VM_Release_1.1_Language_Reference_199506.pdf",
        "file": "SC26-3114-01_PLI_for_MVS_and_VM_1.1_Language_Reference.pdf",
        "note": "PL/I for MVS&VM 1.1 LRM (Iron Spring reference language)",
    },
    # --- Execution / runtime model ---
    {
        "id": "sc33-0025-3",
        "cat": "runtime",
        "path": "ibm/370/pli/SC33-0025-3_PLI_Optimizing_Compiler_Execution_Logic_Sep85.pdf",
        "file": "SC33-0025-3_PLI_Optimizing_Compiler_Execution_Logic_Sep85.pdf",
        "note": "Optimizing compiler execution logic (runtime model)",
    },
    {
        "id": "sc33-0025-2",
        "cat": "runtime",
        "path": "ibm/370/pli/SC33-0025-2_PLIexLgic_Apr73.pdf",
        "file": "SC33-0025-2_PLI_Execution_Logic_Apr73.pdf",
        "note": "Earlier execution logic edition",
    },
    # --- Context / guides ---
    {
        "id": "gc33-0001-2",
        "cat": "classic",
        "path": "ibm/370/pli/GC33-0001-2_PLIgenInf_Jun73.pdf",
        "file": "GC33-0001-2_PLI_Optimizing_General_Info_Jun73.pdf",
        "note": "Optimizing compiler general information",
    },
    {
        "id": "gc33-0003-0",
        "cat": "classic",
        "path": "ibm/370/pli/GC33-0003-0_PLI_Checkout_Compiler_General_Info_Jul70.pdf",
        "file": "GC33-0003-0_PLI_Checkout_General_Info_Jul70.pdf",
        "note": "Checkout compiler general information",
    },
    {
        "id": "c28-8202-0",
        "cat": "classic",
        "path": "ibm/360/pli/C28-8202-0_PLIsubsetRM_1967.pdf",
        "file": "C28-8202-0_PLI_Subset_Reference_Manual_1967.pdf",
        "note": "Early IBM subset RM (pre-ANSI-G; historical)",
    },
    {
        "id": "sc26-3113-01",
        "cat": "classic",
        "path": "ibm/370/pli/SC26-3113-01_PLI_for_MVS_and_VM_Release_1.1_Programming_Guide_199506.pdf",
        "file": "SC26-3113-01_PLI_for_MVS_and_VM_1.1_Programming_Guide.pdf",
        "note": "MVS&VM 1.1 programming guide",
    },
    # --- Absolute URLs (non-Bitsavers) ---
    {
        "id": "iron-spring-prog-guide",
        "cat": "practice",
        "url": "http://www.iron-spring.com/prog_guide.html",
        "file": "IronSpring_Programming_Guide.html",
        "note": "Iron Spring Programming Guide (HTML; OCR/convert as needed)",
    },
    {
        "id": "iron-spring-preprocessor",
        "cat": "practice",
        "url": "http://www.iron-spring.com/Iron-SpringPLIPreprocessor.pdf",
        "file": "IronSpring_PLI_Preprocessor.pdf",
        "note": "Iron Spring preprocessor reference PDF",
    },
    {
        "id": "iron-spring-faq",
        "cat": "practice",
        "url": "http://www.iron-spring.com/faq.html",
        "file": "IronSpring_FAQ.html",
        "note": "Iron Spring FAQ (Subset G goals, licensing notes)",
    },
    {
        "id": "iron-spring-readme-linux",
        "cat": "practice",
        "url": "http://www.iron-spring.com/readme_linux.html",
        "file": "IronSpring_Readme_Linux.html",
        "note": "Iron Spring Linux release notes",
    },
]

# Git repos to clone/update (optional tool: git)
GIT_REPOS = [
    {
        "id": "subsetg-book",
        "cat": "practice",
        "url": "https://github.com/johnwcowan/subsetg-book.git",
        "dir": "subsetg-book",
        "note": "Open description of Subset G (MIT)",
    },
]

NOT_FETCHED = """
NOT FETCHED BY THIS SCRIPT (obtain legally yourself)
====================================================
- ANSI X3.74-1981 / X3.74-1987  PL/I General-Purpose Subset (Subset G)
- ISO/IEC 6522:1992
- ANSI X3.53-1976 / ISO 6160:1979  (full PL/I)
- ECMA-50
- ACM: Abrahams "Subset/G PL/I and the PL/I Standard" (usually paywalled)
- Commercial books (Dragon Book, Engineering a Compiler, ANTLR book)
- Modern Enterprise PL/I LRM: download from official IBM Docs when needed
  https://www.ibm.com/docs/en/pli-for-zos

Open substitutes for Subset G while standards are pending:
  - research/refs/literature/subsetg-book/  (after git fetch)
  - Iron Spring docs + SC26-3114 (MVS&VM 1.1)
"""


def sha256_file(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as f:
        for chunk in iter(lambda: f.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()


def download(url: str, dest: Path, timeout: int = 120) -> None:
    dest.parent.mkdir(parents=True, exist_ok=True)
    tmp = dest.with_suffix(dest.suffix + ".part")
    ctx = ssl.create_default_context()
    req = urllib.request.Request(
        url,
        headers={"User-Agent": "pli-literature-fetch/1.0 (local research shelf)"},
    )
    with urllib.request.urlopen(req, context=ctx, timeout=timeout) as resp:
        data = resp.read()
    tmp.write_bytes(data)
    tmp.replace(dest)


def try_bitsavers(rel_path: str, dest: Path) -> str:
    errors: list[str] = []
    for base in BITSAVERS:
        url = f"{base.rstrip('/')}/{rel_path.lstrip('/')}"
        try:
            download(url, dest)
            return url
        except Exception as e:  # noqa: BLE001 — collect and try next mirror
            errors.append(f"{url}: {e}")
    raise RuntimeError("All Bitsavers mirrors failed:\n  " + "\n  ".join(errors))


def fetch_git(repo: dict, dry_run: bool) -> None:
    target = OUT / repo["dir"]
    print(f"[{repo['id']}] git {repo['url']} -> {target}")
    if dry_run:
        return
    import subprocess

    if target.exists():
        subprocess.run(["git", "-C", str(target), "pull", "--ff-only"], check=False)
    else:
        OUT.mkdir(parents=True, exist_ok=True)
        subprocess.run(["git", "clone", "--depth", "1", repo["url"], str(target)], check=True)


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--dry-run", action="store_true", help="List actions only")
    ap.add_argument(
        "--only",
        choices=["core", "classic", "runtime", "practice", "optional", "all"],
        default="all",
        help="Download subset by category (default: all)",
    )
    ap.add_argument("--skip-git", action="store_true", help="Do not clone subsetg-book")
    args = ap.parse_args()

    cats = None if args.only == "all" else {args.only}
    OUT.mkdir(parents=True, exist_ok=True)

    lines: list[str] = []
    ok = fail = skip = 0

    print(f"Output directory: {OUT}")
    print(NOT_FETCHED)

    for item in ITEMS:
        if cats and item["cat"] not in cats:
            skip += 1
            continue
        dest = OUT / item["file"]
        print(f"[{item['id']}] {item['note']}")
        if dest.exists() and dest.stat().st_size > 0:
            digest = sha256_file(dest) if not args.dry_run else "(exists)"
            print(f"  SKIP exists ({dest.stat().st_size} bytes) sha256={digest}")
            lines.append(f"EXISTS\t{item['id']}\t{dest.name}\t{digest}")
            skip += 1
            continue
        if args.dry_run:
            src = item.get("url") or f"bitsavers:{item['path']}"
            print(f"  DRY-RUN would fetch {src}")
            lines.append(f"DRY\t{item['id']}\t{src}")
            continue
        try:
            if "url" in item:
                url = item["url"]
                download(url, dest)
            else:
                url = try_bitsavers(item["path"], dest)
            digest = sha256_file(dest)
            print(f"  OK {url} -> {dest.name} ({dest.stat().st_size} bytes)")
            print(f"     sha256={digest}")
            lines.append(f"OK\t{item['id']}\t{url}\t{digest}\t{dest.stat().st_size}")
            ok += 1
        except Exception as e:  # noqa: BLE001
            print(f"  FAIL: {e}", file=sys.stderr)
            lines.append(f"FAIL\t{item['id']}\t{e}")
            fail += 1

    if not args.skip_git and (cats is None or "practice" in cats):
        for repo in GIT_REPOS:
            try:
                fetch_git(repo, args.dry_run)
                lines.append(f"GIT\t{repo['id']}\t{repo['url']}")
                ok += 1
            except Exception as e:  # noqa: BLE001
                print(f"  FAIL git {repo['id']}: {e}", file=sys.stderr)
                lines.append(f"FAIL\t{repo['id']}\t{e}")
                fail += 1

    if not args.dry_run:
        MANIFEST_OUT.write_text(
            "\n".join(lines) + "\n\n" + NOT_FETCHED,
            encoding="utf-8",
        )
        readme = OUT / "README.md"
        readme.write_text(
            "# Literature shelf (downloaded)\n\n"
            "Filled by `tools/fetch_literature.py`.\n\n"
            "After OCR, put machine-readable text next to each PDF, e.g.\n"
            "`GC33-0009-4_....pdf.txt` or a `ocr/` subdirectory.\n\n"
            "See also `../LITERATURE.md` and `DOWNLOAD_MANIFEST.txt`.\n"
            + NOT_FETCHED,
            encoding="utf-8",
        )

    print(f"\nDone: ok={ok} fail={fail} skip={skip}")
    return 1 if fail else 0


if __name__ == "__main__":
    sys.exit(main())
