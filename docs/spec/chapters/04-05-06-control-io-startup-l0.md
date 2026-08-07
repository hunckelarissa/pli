# 4.1–4.2, 5.1, 6.3 Управление потоком, PUT, старт (L0)

**Статус:** черновик SPEC  
**Уровень:** L0 must-run  
**Связь:** главы 2–3; ADR-0008; runtime `libpl1rt`

---

## 4.1 IF

### Syntax

```text
IF expression THEN statement
IF expression THEN statement ELSE statement
```

`statement` — в т.ч. `DO; … END;` или одиночный statement.

### Semantics

- `expression` приводится к истине/лжи: числовая ≠ 0 → true; 0 → false.  
- CHAR в условии — not-implemented на L0.  
- Keywords THEN/ELSE в этой конструкции — keywords (глава 1.3 R1), не имена.

### Status

must-run.

---

## 4.2 DO (простой L0)

### Must-run forms

1. **Do-group:** `DO; statements END;`  
2. **Counted:** `DO var = exp1 TO exp2;` … `END;`  
   - шаг по умолчанию `+1`;  
   - `BY exp` — must-run если успеем; иначе BY — not-implemented на первом срезе (предпочтительно поддержать BY константу).  
3. `WHILE` / `UNTIL` / `DO i = … TO … WHILE` — L1 или not-implemented на L0.

**Решение L0:** must-run = (1) и (2) с целым FIXED var и TO; **BY** — must-run для целочисленного шага; WHILE/UNTIL — not-implemented.

### Status

см. выше.

---

## 4.5 RETURN / STOP

- `RETURN;` в MAIN — завершение программы (код 0).  
- `RETURN;` в не-MAIN — возврат вызывающему (когда CALL must-run).  
- `RETURN(expr)` — not-implemented.  
- `STOP;` — завершение процесса (код 0) must-run.

---

## 5.1 PUT LIST (минимум I/O)

### Must-run

```pli
PUT LIST(item, item, ...);
PUT SKIP LIST(item, ...);
```

- `item` — выражение FIXED/FLOAT/CHAR или строковый литерал.  
- Вывод: runtime `pl1rt_put_*` в stdout; элементы разделяются пробелом; `SKIP` — перевод строки до вывода (как минимум: newline then items).  
- Точная совместимость с IBM spacing — **не** KPI; зафиксировать в тестах `.expected`.

### Not-implemented on L0

`PUT EDIT`, `PUT DATA`, file specifications, `GET` всех форм.

---

## 6.3 Старт MAIN и линковка

1. `pl1c` транспилирует модуль в C.  
2. Генерируется `int main(void)` (или `main` вызывает инициализацию STATIC и вход в MAIN-PROC).  
3. Линковка с **`libpl1rt`**.  
4. Статические `STATIC` инициализируются нулями / явный INIT later; L0 без INIT — zero/blank.  
5. `#line` — ADR-0008.

---

## Examples

```pli
F: PROC OPTIONS(MAIN);
  DCL I FIXED BIN(31);
  DO I = 1 TO 3;
    PUT LIST(I);
  END;
  PUT SKIP LIST('done');
END F;
```

```pli
G: PROC OPTIONS(MAIN);
  DCL A FIXED BIN(31);
  A = 0;
  IF A THEN
    PUT LIST('t');
  ELSE
    PUT LIST('f');
END G;
```

---

## Implementation status

| Тема | L0 |
|------|-----|
| IF/THEN/ELSE | must-run |
| DO group + DO TO | must-run |
| DO WHILE/UNTIL | not-implemented |
| PUT LIST / SKIP LIST | must-run |
| GET / EDIT | not-implemented |
| main + libpl1rt | must-run |

---

## References

- Classic LRM IF/DO/PUT  
- ADR-0006, ADR-0008  
- `08-test-catalog-l0.md` MR001–MR004
