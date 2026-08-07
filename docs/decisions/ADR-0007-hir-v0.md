# ADR-0007: HIR contract (L0 sketch)

- **Status:** Accepted (v0 sketch)
- **Date:** 2026-08-05
- **Decides:** D6

## Context

Нужен стабильный вход для `codegen_c`, независимый от деталей CST/парсера.

## Decision

- Промежуточное представление: **HIR**, версия схемы **`hir_v0`**.
- Sema опускает AST → HIR только для конструкций со статусом must-run; иначе diagnostics not-implemented **до** эмита.
- HIR сериализуем опционально (debug dump JSON/текстом) — флаг CLI позже.
- Ломающие изменения схемы → bump `hir_v1` + ADR.

### Узлы hir_v0 (минимум L0)

| Узел | Назначение |
|------|------------|
| `Module` | единица компиляции |
| `Proc` | процедура (`OPTIONS(MAIN)` флаг) |
| `Block` | BEGIN/тело |
| `Decl` | имя, тип L0, storage class |
| `Assign` | lvalue = expr |
| `Binary` / `Unary` | арифметика/сравнения L0 |
| `If` | then/else |
| `Do` | простой цикл L0 (форма уточняется SPEC) |
| `Call` | вызов; args = lvalues/temps |
| `PutList` | элементы списка вывода |
| `Return` / `Stop` | завершение |
| `Name` / `Literal` | ссылка / константа |
| `Type` | FixedBin / Float / Char | 

Не в hir_v0: BASED overlay, ON-units, полный SELECT, preprocessor.

Полная текстовая спека узлов: `docs/spec/chapters/07-hir-v0.md` (дописать до кода codegen).

## Consequences

- Parser/AST могут меняться свободно; codegen смотрит только HIR.
- LLVM позже читает тот же HIR.

## References

- STACK.md pipeline
- ADR-0001
