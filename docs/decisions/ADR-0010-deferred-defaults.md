# ADR-0010: Deferred defaults (L0 host environment)

- **Status:** Accepted (temporary defaults)
- **Date:** 2026-08-05
- **Decides:** D8, D9, D10, D12, D13, D14

## Context

Нужны явные допущения, чтобы не блокировать SPEC, без преждевременной детализации.

## Decision

| ID | Default до отдельного ADR |
|----|---------------------------|
| D8 | Host L0: **Windows и Linux**, little-endian, 64-bit primary |
| D9 | Исходники: **UTF-8**; ключевые слова сравниваются case-insensitive в ASCII-диапазоне; не-ASCII в идентификаторах — not-implemented/ошибка на L0 |
| D10 | В сообщениях: **`PL/I-G`** (см. ADR-0006) |
| D12 | **BIT** и **POINTER**: must-parse / not-implemented в codegen до L1 |
| D13 | Каталог `codegen_llvm/` допустим пустым; реализация после L1 |
| D14 | Концепция/SPEC на **русском**; идентификаторы кода, CLI help, SPAN комментарии кода — **английский** |

## Consequences

- Можно писать главы SPEC и скелет репо без новых споров.
- Смена default = новый ADR (не «тихо поправить»).

## References

- OPEN.md DEFER section
