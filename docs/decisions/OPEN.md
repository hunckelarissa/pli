# Реестр архитектурных решений

**Статус:** критичные решения **закрыты** (2026-08-05)  
**Правило:** новое решение или смена = новый ADR; не «тихо в чате».

Связь: [`../tz/TZ.md`](../tz/TZ.md), [`ADR-0001-product-direction.md`](ADR-0001-product-direction.md)

---

## Сводка (было OPEN)

| ID | Тема | Решение | ADR |
|----|------|---------|-----|
| D1 | Язык фронтенда | **Rust** + runtime **C11** | [0003](ADR-0003-implementation-language.md) |
| D2 | Парсер | **Свой lexer + recursive descent** (Rust) | [0004](ADR-0004-parser-stack.md) |
| D3 | Параметры L0 | **By reference** (+ temporaries для литералов) | [0005](ADR-0005-parameter-model.md) |
| D4 | Имена | **`pl1c`** / **`libpl1rt`** / диалект **PL/I-G** | [0006](ADR-0006-naming.md) |
| D5 | Лицензия | **Apache-2.0** | [0002](ADR-0002-license.md) |
| D6 | HIR | **`hir_v0`** узлы L0 | [0007](ADR-0007-hir-v0.md) |
| D7 | Debug / `#line` | `#line` по умолчанию; `--no-line`, `--emit-c`, `--dump-hir` | [0008](ADR-0008-debug-line.md) |
| D11 | Preprocessor L0 | **Не исполняется**; `%…` → not-implemented | [0009](ADR-0009-preprocessor-l0.md) |
| D8–D10, D12–D14 | Defaults | Windows+Linux, UTF-8, BIT/POINTER parse-only, … | [0010](ADR-0010-deferred-defaults.md) |

Продуктовая рамка: [ADR-0001](ADR-0001-product-direction.md).

---

## Список ADR

| ADR | Title |
|-----|-------|
| 0001 | Product direction |
| 0002 | License (Apache-2.0) |
| 0003 | Implementation language (Rust + C) |
| 0004 | Parser stack |
| 0005 | Parameter model L0 |
| 0006 | Naming (`pl1c`, `libpl1rt`) |
| 0007 | HIR v0 |
| 0008 | Debug / `#line` |
| 0009 | Preprocessor L0 |
| 0010 | Deferred defaults |

---

## Ещё не ADR (следующий берег, не блокер каркаса)

- Детальная текстовая спека каждого узла `hir_v0` → `docs/spec/chapters/07-hir-v0.md`
- Точная форма простого `DO` на L0 (глава 4.2)
- Точный CLI surface (`-o`, outdir) — в главе 7.3
- `%INCLUDE` на L1 — будущий ADR

---

## Шаблон нового ADR

```markdown
# ADR-00XX: Title

- Status: Accepted | Rejected | Superseded
- Date: YYYY-MM-DD
- Decides: D? / topic

## Context
## Decision
## Consequences
## References
```
