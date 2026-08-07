# ADR-0009: Preprocessor policy (L0)

- **Status:** Accepted
- **Date:** 2026-08-05
- **Decides:** D11

## Context

Полный preprocessor PL/I — отдельный язык. Манифест: не блокировать L0.

## Decision

- **L0: preprocessor не исполняется.**
- Встретив `%` preprocessor-statement (включая `%INCLUDE`, `%REPLACE`, `%IF`, `%DO`, …): после распознавания — **not-implemented** (ошибка уровня реализации), либо (если ещё не распознали в грамматике) syntax/diagnostic — но не молчаливый skip.
- Исключение не делается для `%INCLUDE` на L0 (перенос на **L1**, отдельный ADR/SPEC).
- `*PROCESS` / аналог опций — out-of-scope или not-implemented на L0.

## Consequences

- Must-run L0 без `%`.
- Must-parse может содержать `%` только если грамматика уже есть и тест ждёт not-implemented после parse — иначе не включать.

## References

- MANIFEST L0 table
- Iron Spring preprocessor (информативно, research/docs practice)
