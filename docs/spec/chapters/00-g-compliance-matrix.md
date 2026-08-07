# 0.3 Матрица соответствия Subset G / классика / PL/I-G

**Статус:** черновик SPEC (живой реестр)  
**Назначение:** честно фиксировать, что обещаем относительно G-ref и Classic LRM.  
**Не обещаем:** Enterprise; полный X3.53 как KPI.

Легенда статусов: как в [`00-intro.md`](00-intro.md).  
Уровни: L0 / L1 / L2 / G (= G-horizon) / — (out-of-scope).

---

## Как читать

| Колонка | Смысл |
|---------|--------|
| G-ref | Есть в нашем рабочем Subset G reference |
| Classic | Типично в Checkout/Optimizer LRM |
| PL/I-G | Наш диалект: уровень + статус |

При конфликте: **наша SPEC** > якоря.

---

## Лексика и форма программы

| Фича | G-ref | Classic | PL/I-G |
|------|-------|---------|--------|
| Keywords not reserved | да | да | L0 must (ch.1.3) |
| Case-insensitive keywords | да | да | L0 must |
| PROCEDURE / PROC | да | да | L0 must-run |
| OPTIONS(MAIN) | да | да | L0 must-run |
| BEGIN / END | да | да | L0 must-run |
| END name (simple) | да | да | L0 must-run |
| Multi-level END name | да | да | L0 must-parse; exec L1 |
| ENTRY | да | да | L0 must-parse / n-i |
| PACKAGE | —/ext | ext | out-of-scope / n-i |
| Labels | да | да | L0 must-parse; GOTO n-i |

---

## Данные

| Фича | G-ref | Classic | PL/I-G |
|------|-------|---------|--------|
| FIXED BIN | да | да | L0 must-run |
| FLOAT | да | да | L0 must-run |
| CHAR(n) | да | да | L0 must-run |
| BIT | да | да | L0 must-parse / n-i → L1 |
| POINTER | да | да | L0 must-parse / n-i → L1 |
| FIXED DEC | да | да | L2 / G |
| PICTURE | да | да | L2 / G |
| COMPLEX | да | да | — / late |
| AUTOMATIC | да | да | L0 must-run |
| STATIC | да | да | L0 must-run |
| PARAMETER / by ref | да | да | L0 must-run (ADR-0005) |
| BASED | да | да | L1+ |
| CONTROLLED | да | да | L2 / G |
| DEFINED / LIKE | да | да | L1–L2 |
| Arrays | да | да | L1 |
| Structures | да | да | L1 |
| Implicit declare | да | да | L0 n-i (явный DCL) |
| DEFAULT | да | да | L1 / n-i |

---

## Управление и выражения

| Фича | G-ref | Classic | PL/I-G |
|------|-------|---------|--------|
| Assignment | да | да | L0 must-run |
| IF / THEN / ELSE | да | да | L0 must-run |
| DO; group | да | да | L0 must-run |
| DO i= TO [BY] | да | да | L0 must-run |
| DO WHILE/UNTIL | да | да | L1 |
| SELECT | да | да | L0 must-parse → L1 exec |
| CALL | да | да | L0 must-run (same module) |
| RETURN; | да | да | L0 must-run |
| RETURN(expr) | да | да | L1 |
| GOTO | да | да | L1 / n-i on L0 |
| STOP | да | да | L0 must-run |
| Nested PROC exec | да | да | L0 n-i → L1 |

---

## I/O и условия

| Фича | G-ref | Classic | PL/I-G |
|------|-------|---------|--------|
| PUT LIST / SKIP LIST | да | да | L0 must-run |
| PUT EDIT / DATA | да | да | L2 |
| GET LIST | да | да | L1 |
| Record I/O | да | да | late / limited |
| ON / SIGNAL | да | да | L2+ / урезать |
| DISPLAY | да | да | L1 optional |

---

## Preprocessor

| Фича | G-ref | Classic | PL/I-G |
|------|-------|---------|--------|
| %INCLUDE | да | да | L0 n-i → L1 (ADR-0009) |
| %REPLACE | да | да | L1 |
| %IF / %DO / %PROCEDURE | да | да | L2 |
| *PROCESS | — | да | L0 n-i |

---

## Builtins (сводка)

Полный список G builtins — не L0.  
L0: достаточно выражений и PUT; отдельные builtins — реестр в ch.6.1 later.

| Пример | PL/I-G |
|--------|--------|
| ABS, MIN, MAX, MOD | L1 |
| SUBSTR, LENGTH, INDEX | L1 |
| ADDR, NULL | с POINTER L1+ |

---

## Явный out-of-scope (не G-horizon продукта)

- Multitasking (TASK, EVENT, WAIT…)  
- CICS / DL/I / SQL preprocessors  
- Enterprise OO / JSON / и т.п.  
- Совместимость object-кода IBM LE  

---

## Правило обновления

Любое повышение статуса фичи (n-i → must-run) — правка этой матрицы + тест в `08-…` + при необходимости ADR.
