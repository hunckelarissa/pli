# 3.1–3.2, 3.5–3.6 Типы, хранение, присваивание, параметры (L0)

**Статус:** черновик SPEC  
**Уровень:** L0 must-run  
**Якоря:** G-ref; Classic LRM; SC26-3114; ADR-0005; ADR-0007 (`Type` в hir_v0)

---

## 3.1 Типы данных L0

### Must-run types

| Тип PL/I-G (L0) | Запись в DECLARE (примеры) | HIR / C (дух) |
|-----------------|----------------------------|---------------|
| Fixed binary | `FIXED BIN(31)`, `FIXED BINARY(15)` | `FixedBin { precision }` → `int32_t` / `int16_t` по precision |
| Float | `FLOAT`, `FLOAT BIN` | `Float` → `double` (L0 единообразно) |
| Character fixed | `CHAR(n)`, `CHARACTER(n)` | `Char { len: n }` → `char[n]` или runtime-строка фикс. длины |

Precision: для L0 must-run достаточно **15 и 31** для FIXED BIN; иные precision — must-parse / not-implemented или усечение с diagnostic (предпочтительно: поддержать 8/15/31 или diagnostic).

### Parse-only / later (не L0 codegen)

| Тип / атрибут | Статус |
|---------------|--------|
| `BIT` | must-parse / not-implemented (ADR-0010) |
| `POINTER` | must-parse / not-implemented |
| `FIXED DEC` / PICTURE | not-implemented |
| `COMPLEX` | not-implemented |
| `AREA` / `OFFSET` | out-of-scope / later |
| `FILE` | not-implemented |

### Defaults

Если атрибуты неполные — L0: **явный DCL обязателен** для must-run имён (никакого implicit DECLARE в must-run тестах).  
Implicit / DEFAULT statement — not-implemented на L0.

---

## 3.2 Классы хранения L0

| Класс | Статус | Семантика L0 |
|-------|--------|--------------|
| AUTOMATIC | must-run (default если не указано иное для локальных) | stack / local; жизнь = активация блока |
| STATIC | must-run | статическое хранение; сохраняет значение между вызовами |
| PARAMETER | must-run | storage вне процедуры; см. 3.6 |
| BASED | must-parse / not-implemented | |
| CONTROLLED | not-implemented | |
| DEFINED | L0 partial (scalar same-type overlay) | MR063 |

`INTERNAL` / `EXTERNAL`: L0 must-run — всё internal в одном модуле; `EXTERNAL` — not-implemented.

---

## 3.5 Присваивание и преобразования L0

### Assignment

`reference = expression;`

- Левая часть: имя переменной L0-типа (индексация массивов — L1).
- Правая часть: выражение L0 (литералы, имена, `+ - * /`, сравнения для IF).

### Conversions (минимальный набор must-run)

| Из → В | Правило L0 |
|--------|------------|
| FIXED → FIXED | значение; при сужении precision — implementation-defined truncate с optional warning |
| FIXED → FLOAT | обычное числовое |
| FLOAT → FIXED | trunc toward zero |
| CHAR → CHAR | copy; если dest короче — truncate; длиннее — pad spaces |
| число ↔ CHAR | not-implemented на L0 (кроме явных позже) |

Смешение в одном выражении FIXED и FLOAT: обычные arithmetic conversions → FLOAT.

Ошибка: присваивание несовместимых классов без правила — diagnostic.

---

## 3.6 Параметры и вызовы (L0)

Норматив: **ADR-0005** (by reference).

### Declare

```pli
CALC: PROC (A, B);
  DCL (A, B) FIXED BIN(31);
  ...
END CALC;
```

Атрибут `PARAMETER` может быть явным или выводиться из списка PROC — L0 допускает оба стиля, если однозначно; иначе diagnostic.

### Call

```pli
CALL CALC (X, Y);
```

- `CALL` must-run для процедур **того же compilation unit**, уже проанализированных, без not-implemented тела.
- Аргументы-переменные: адрес.
- Аргументы-литералы/выражения: temporary + адрес; модификация через параметр temporary не обязана быть видна вызывающему (тесты L0 не опираются на это).

### RETURNS / functional call in expression

**not-implemented** на L0.

---

## DECLARE syntax (L0 bridge)

```text
declaration := (DECLARE|DCL) declare_item_list ';'

declare_item  /* упрощённо */
  := name_fact attributes?
   | '(' name_list ')' attributes?
```

Факторы/факторные скобки Classic — must-parse широкий; must-run тесты пишут простые формы.

Структуры `1 A, 2 B` — L1.  
Массивы `DCL A(10)` — L1.

---

## Examples

### Must-run types + static

```pli
T: PROC OPTIONS(MAIN);
  DCL C FIXED BIN(31) STATIC;
  DCL X FIXED BIN(31);
  X = 2;
  C = C + X;
  PUT LIST(C);
END T;
```

### Must-run call by reference

```pli
MAINP: PROC OPTIONS(MAIN);
  DCL V FIXED BIN(31);
  V = 1;
  CALL INC(V);
  PUT LIST(V);
END MAINP;

INC: PROC(P);
  DCL P FIXED BIN(31);
  P = P + 1;
END INC;
```

*(Порядок: оба внешних PROC в одном файле; правила «кто кому виден» — оба в module scope L0.)*

---

## Implementation status summary

| Тема | L0 |
|------|-----|
| FIXED BIN / FLOAT / CHAR | must-run |
| AUTO / STATIC / PARAM | must-run |
| assignment + numeric convert | must-run |
| CALL by ref same module | must-run |
| arrays/structures | L1 |
| BASED/BIT/POINTER codegen | not-implemented |
| DECIMAL/PICTURE | not-implemented |

---

## References

- ADR-0005, ADR-0007, ADR-0010  
- G-ref data attributes  
- Classic LRM DECLARE  
- `07-hir-v0.md`, `08-test-catalog-l0.md`
