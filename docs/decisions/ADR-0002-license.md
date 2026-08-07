# ADR-0002: License

- **Status:** Accepted
- **Date:** 2026-08-05
- **Decides:** D5

## Context

Продукт с нуля; `pl1gcc` в `research/` под GPLv2 и **не** входит в линковку. Нужна лицензия, удобная для долгой игры и распространения компилятора + runtime.

## Decision

- Компилятор, runtime, тесты и документация проекта — **Apache License 2.0**.
- Сырьё в `research/` сохраняет **свои** лицензии (pl1gcc GPLv2, чужие PDF/HTML — только reference, не перелицензировать).
- Новые файлы кода — с SPDX: `SPDX-License-Identifier: Apache-2.0`.

## Consequences

- Не тащить код pl1gcc в продукт.
- Patent grant Apache полезен для компиляторного инструмента.
- При желании позже dual-license — отдельный ADR.

## References

- `docs/decisions/OPEN.md` D5
- `research/pl1gcc-0.0.15/` (не продукт)
