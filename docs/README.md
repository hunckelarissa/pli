# docs/ - Source of Truth (machine-readable)

This folder holds the **selected** literature converted to Markdown for
reading, search, and future TZ/SPEC work.

Original PDFs/HTML remain under `research/refs/literature/` and `research/` (books).
If text conflicts with a scanned PDF artifact, prefer the PDF - but for
day-to-day work, **use these `.md` files**.

## Concept layer (in progress)

| Document | Path |
|----------|------|
| **What next** | [`NEXT.md`](NEXT.md) |
| Technical assignment | [`tz/TZ.md`](tz/TZ.md) |
| Decisions registry + ADR-0001…0010 | [`decisions/`](decisions/) |
| SPEC outline | [`spec/OUTLINE.md`](spec/OUTLINE.md) |
| SPEC chapters | [`spec/chapters/`](spec/chapters/) |

## Priority literature (what we selected and why)

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

## Regeneration of literature extracts

```text
python tools/build_docs_truth.py
```
