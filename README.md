# PL/I-G (`pl1c`)

Компилятор диалекта классического PL/I с траекторией к Subset G.  
Транспиляция в C + runtime `libpl1rt`. Лицензия: Apache-2.0.

| | |
|--|--|
| **Version** | see [`VERSION`](VERSION) / `pl1c --version` |
| **Docs (source of truth)** | [`docs/`](docs/README.md) |
| **Concept** | [`MANIFEST.md`](MANIFEST.md) |
| **Git / GitHub** | [`GIT.md`](GIT.md) — что коммитить; `research/` локально |

## Build

Requirements: **Rust** (cargo), **gcc** (or set `PL1C_CC`).

```powershell
cargo build -p pl1c
```

## First test (MR001 hello)

```powershell
powershell -File tools/run_mr001.ps1
```

Or manually:

```powershell
cargo run -p pl1c -- tests/must-run/L0_MR_001_hello.pli -o hello.exe
.\hello.exe
```

Emit C only:

```powershell
cargo run -p pl1c -- tests/must-run/L0_MR_001_hello.pli --emit-c hello.c
```

## Layout

```text
compiler/     Rust crate pl1c
runtime/      C libpl1rt
tests/        must-run / must-parse / must-fail
docs/         SPEC, ADR, literature extracts  ← в git
research/     raw PDF/zip (локально, ~1GB)    ← не в git
```

## Status

Skeleton: lex → parse (narrow) → HIR → C → link `libpl1rt`.  
L0 language surface grows against `docs/spec/`.
