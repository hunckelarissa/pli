# ADR-0008: Debug and #line policy

- **Status:** Accepted
- **Date:** 2026-08-05
- **Decides:** D7

## Context

Транспиляция в C выигрывает, если можно отлаживать и читать эмит.

## Decision

- По умолчанию эмит C **включает `#line`**, указывающие на исходный `.pli` (файл + строка).
- Флаг **`--no-line`** отключает `#line`.
- Флаг **`--emit-c <path>`** (или эквивалент) записывает C и может остановиться без линковки (`--compile-only` / `-c` семантика — уточнить в CLI).
- Флаг **`--dump-hir`** — текстовый/JSON dump HIR (debug).
- DWARF «как у родного PL/I», native debug info — **не L0**; отдельный ADR.
- Отладка L0: gdb/lldb по C с `#line` → .pli, либо чтение `.c`.

## Consequences

- Проще сопровождать must-run и охотиться за багами эмита.
- Некоторые компиляторы C чувствительны к `#line` — тестировать на gcc и clang.

## References

- ADR-0006 naming
- TZ §F4
