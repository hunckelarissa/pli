# ADR-0003: Implementation language

- **Status:** Accepted
- **Date:** 2026-08-05
- **Decides:** D1

## Context

Нужны жёсткие границы модулей, долгий рост AST/sema/HIR, эмит в C, runtime на C. ИИ хорошо генерирует и Rust, и C++.

## Decision

- **Frontend компилятора (lex…codegen_c, driver): Rust** (edition 2021+).
- **Runtime (`libpl1rt`): C11** (или C17, если toolchain позволяет; минимум C11).
- Сборка: `cargo` для фронта; runtime — CMake или `cc` из build.rs / отдельный CMake; линковка драйвером `pl1c` с системным `cc`.
- Не писать весь продукт на C «как pl1gcc».

## Consequences

- Чёткая граница FFI только на стыке с runtime / libclang-less emit of `.c` text.
- Нужен Rust toolchain на dev-машинах (Windows + Linux).
- Modula-2/ULM остаётся своим миром; общий только опыт, не код.

## References

- `STACK.md`
- ADR-0001
