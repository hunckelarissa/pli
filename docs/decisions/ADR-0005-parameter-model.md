# ADR-0005: Parameter passing model (L0)

- **Status:** Accepted
- **Date:** 2026-08-05
- **Decides:** D3

## Context

Классический PL/I близок к dummy arguments / by-reference. Для эмита в C нужен простой, честный контракт на L0.

## Decision

**L0: передача параметров — по ссылке (by reference).**

- Фактический аргумент-переменная: передаётся адрес (в C — указатель на storage).
- Фактический аргумент-выражение или литерал: создаётся **временная** в вызывающем, передаётся её адрес; изменения кали через параметр **не** определены как must-run сверх «как получилось с temporary» — в тестах L0 избегать side-effects через литералы.
- `RETURNS` / функции: L0 может ограничиться `PROC` без значения; если ENTRY/RETURNS появятся раньше L1 — отдельное уточнение в SPEC ch.3.6.
- `BYVALUE` / иные атрибуты — **not-implemented** на L0 (parse-only при появлении в грамматике).

## Consequences

- Эмит C: `void foo(pl1_fixedbin31 *a, ...)`.
- Ближе к классике, чем by-value.
- SPEC глава 3.6 обязана повторить эти правила примерами.

## References

- Classic LRM / G-ref (информативно)
- `docs/spec/OUTLINE.md` §3.6
