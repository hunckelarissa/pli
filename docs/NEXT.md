# NEXT — что делать после берега L0

**Дата:** 2026-08-05  
**Статус документов L0:** gate закрыт (ТЗ + ADR + SPEC core + матрица G).

---

## Сделано на берегу

- Манифест / STACK / LITERATURE / `docs/` / `research/`  
- ТЗ, ADR-0001…0010  
- SPEC: intro, G-matrix, keywords, program structure, statements table, types, control/IO, HIR, tests  

---

## Развилка

### A. Продолжать только документы (ещё берег)

- Углубить multi-END, полный список abbrev keywords  
- Черновик L1 (arrays/structures) без кода  
- Больше текстов must-run в `08-…`  

### B. Скелет продукта (код) — нужна фраза вроде «делай скелет pl1c»

Минимальный вертикальный срез:

```text
compiler/   (Rust: pl1c driver, lex, parse stub, hir, codegen_c stub)
runtime/    (C: libpl1rt — put_list, start)
tests/      (MR001 hello — цель первого зелёного)
LICENSE     Apache-2.0
Cargo.toml / CMake
```

Первый KPI кода: **MR001 hello** → бинарник (даже с узким парсером только под hello, затем расширение по SPEC).

### C. Параллельно

ULM Modula-2 не смешивать с этим репо.

---

## Рекомендация

Документальный берег L0 **достаточен**. Дальнейшая отдача — от **B** (скелет).  
Без явной команды на код агент остаётся в зоне A.
