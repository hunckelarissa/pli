# 2.1–2.2 Структура программы и области видимости (L0)

**Статус:** черновик SPEC  
**Уровень:** L0 (must-run ядро) + must-parse расширения  
**Якоря:** Classic LRM; G-ref; ADR-0005 (параметры); ADR-0006 (OPTIONS MAIN)

---

## 2.1 Программа и процедуры

### Statement

Единица компиляции L0 — один или несколько внешних `PROCEDURE` в одном файле (L0 must-run: достаточно **одной** внешней процедуры с `OPTIONS(MAIN)`).

### Syntax (дух, не формальная EBNF навсегда)

```text
compilation_unit := procedure+

procedure := label_list? (PROCEDURE|PROC) parameter_list? options? ';'
             block_body
             END name? ';'

parameter_list := '(' name_list ')'

options := OPTIONS '(' option_list ')'
option_list := MAIN   /* L0 must-run; прочие options — not-implemented */

block_body := (declaration | statement)*
```

`BEGIN; … END;` — вложенный блок (must-run L0).

`ENTRY` — **must-parse / not-implemented** на L0 (исполнение — L1+).

`PACKAGE` — out-of-scope или not-implemented (не L0).

### OPTIONS(MAIN)

- Ровно одна процедура в программе must-run должна быть точкой входа `OPTIONS(MAIN)` (или согласованное расширение позже).
- Runtime стартует с неё (глава 6.3).
- Имя в `END name` должно совпадать с именем процедуры, если указано; иначе diagnostic.

### Parameters

- Список имён в `PROC (A, B)`; типы — через `DCL` с атрибутом параметра / сопоставление по правилам главы 3.6.
- Передача: **by reference** (ADR-0005).

### Implementation status

| Конструкция | Статус L0 |
|-------------|-----------|
| PROC/PROCEDURE + END | must-run |
| OPTIONS(MAIN) | must-run |
| BEGIN/END | must-run |
| вложенный PROC | must-parse; exec — уточнить (предпочтительно must-run ограниченно или not-implemented до L1) |
| ENTRY | must-parse / not-implemented |
| RETURNS | not-implemented |
| OPTIONS кроме MAIN | not-implemented |

**Решение по вложенному PROC на L0 (временное):**  
**must-parse;** исполнение вложенного PROC — **not-implemented**, пока не закроем отдельным ADR/дополнением. Must-run тесты L0 — только «плоский» MAIN (+ BEGIN).

---

## 2.2 Метки и области видимости

### Labels

- Statement может иметь одну или несколько меток: `L: M: stmt`.
- Имя метки — Ident по главе 1.3 (может совпадать с keyword).
- `GOTO` — **not-implemented** на L0 (parse — по мере грамматики).

### Scopes (L0)

Иерархия:
1. Внешняя процедура  
2. Вложенный `BEGIN`  
3. (позже) вложенный PROC  

Правила имён L0:
- Объявление (`DCL`) видно от точки объявления до конца scope (упрощение; уточнение «до конца блока» — Classic; для L0: **весь блок после объявления**, forward reference к переменной до DCL — diagnostic).
- Внутренний scope может объявить имя, скрывающее внешнее (**shadowing** must-run).
- Параметры — в scope процедуры.
- Два объявления одного имени в одном scope — ошибка.

### END matching

- `END;` закрывает ближайший открытый блок/процедуру.
- `END name;` закрывает блок/процедуру с меткой/именем `name` (Classic multi-end дух): если имя не найдено — diagnostic.
- Множественное закрытие одним `END name` — **must-parse**; must-run L0 может ограничиться простыми END без multi-close, multi-close exec — L1, если сложно.

**L0 must-run:** парные `PROC`/`BEGIN` + `END` / `END name` один-в-один без «прыжка через несколько уровней».  
**Must-parse:** более сложные END — по грамматике; exec not-implemented при невозможности простого эмита.

---

## Statements overview (L0 bridge → ch.2.3)

| Statement | L0 |
|-----------|-----|
| присваивание | must-run |
| IF / THEN / ELSE | must-run (глава 4.1) |
| DO (простой) | must-run (глава 4.2) |
| PUT LIST / PUT SKIP LIST | must-run (глава 5.1) |
| RETURN / STOP | must-run (минимально) |
| CALL | must-run только для процедур того же модуля без not-implemented ограничений; иначе later |
| SELECT | must-parse |
| GET | must-parse / not-implemented |
| ON / SIGNAL | not-implemented / out-of-scope куски |
| DECLARE | must-run для L0-типов (глава 3) |

---

## Examples

### Must-run: MAIN + BEGIN

```pli
P: PROC OPTIONS(MAIN);
  DCL X FIXED BIN(31);
  X = 1;
  BEGIN;
    DCL Y FIXED BIN(31);
    Y = X + 1;
    PUT LIST(Y);
  END;
END P;
```

### Must-parse: nested PROC (exec not-implemented on L0)

```pli
OUTER: PROC OPTIONS(MAIN);
  INNER: PROC;
    PUT LIST('x');
  END INNER;
END OUTER;
```

---

## References

- Classic LRM — PROCEDURE, BEGIN, END  
- ADR-0005, ADR-0006  
- `08-test-catalog-l0.md`
