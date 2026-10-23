# ADR-0004: Parser stack

- **Status:** Accepted
- **Date:** 2026-08-05
- **Decides:** D2

## Context

Классический PL/I с keywords-as-names (`IF THEN THEN THEN = ELSE`) плохо ложится на «чистый» сгенерированный LALR без трюков. Tree-sitter отвергнут как ядро. ANTLR возможен, но семантика контекста keywords всё равно ручная.

## Decision

- **Свой scanner (lexer) + handwritten recursive-descent parser на Rust.**
- Выражения: Pratt parsing или descent с приоритетами — на усмотрение реализации, внутри того же модуля.
- Идеи lookahead / keyword-disambiguation — из `research/pl1gcc`, реализация своя.
- **Не** использовать ANTLR/Tree-sitter/bison как обязательное ядро L0.
- Позже допустим отдельный Tree-sitter grammar **только для редактора** (другой ADR).

## Consequences

- Больше ручного кода парсера — сознательно (как ULM-дух), зато полный контроль.
- ИИ пишет descent-правила по SPEC-главам.
- Тесты must-parse — главный страж регрессий keywords-as-names.

## References

- MANIFEST §keywords-as-names
- `research/pl1gcc-0.0.15/pl1/pl1-scanner.l`, `pl1-parser.y`
