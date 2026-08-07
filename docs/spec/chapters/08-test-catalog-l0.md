# 8.x Каталог тестов L0 (черновик)

**Статус:** черновик SPEC  
**Назначение:** начальный набор эталонов для приёмки L0; файлы появятся в `tests/` при старте кода.

---

## 8.1 Правила

- Имя теста: `L0_MR_###_shortname.pli` (must-run) или `L0_MP_###_shortname.pli` (must-parse).  
- Must-run: фиксированный ожидаемый stdout (или exit code) в соседнем `.expected`.  
- Must-parse: компилятор завершает parse без syntax error; дальше допустим not-implemented **только если** тест явно помечен `parse-only`.  
- Keywords-as-names — всегда в must-parse; в must-run — когда семантика L0 это покрывает.

---

## 8.2 Must-run L0 (начальный список)

| ID | Имя | Суть | Ожидание (дух) |
|----|-----|------|----------------|
| MR001 | hello | `PUT SKIP LIST('Hello, PL/I')` | строка приветствия |
| MR002 | add | FIXED BIN присваивание и `A+B` | печать суммы |
| MR003 | if_then | IF THEN ELSE на FIXED | одна из двух меток/значений |
| MR004 | do_loop | простой DO i = 1 TO n | накопление/печать |
| MR005 | char_assign | CHAR(n) присваивание и PUT | строка |
| MR006 | static_auto | STATIC vs AUTOMATIC (базово) | различимое поведение |
| MR007 | nested_begin | BEGIN/END внутри PROC | выполняется тело |
| MR008 | call_ref | CALL + by-ref increment | значение после CALL увеличено |
| MR009 | do_to | DO I = 1 TO 3 + PUT | ожидаемая последовательность |
| MR010 | if_else | IF/ELSE ветка | 'f' или 't' по условию |

### MR001 (текст эталона)

```pli
HELLO: PROC OPTIONS(MAIN);
  PUT SKIP LIST('Hello, PL/I');
END HELLO;
```

### MR002 (текст эталона)

```pli
ADD: PROC OPTIONS(MAIN);
  DCL (A, B, C) FIXED BIN(31);
  A = 2;
  B = 3;
  C = A + B;
  PUT LIST(C);
END ADD;
```

### MR008 (текст эталона)

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

Ожидание: печать `2` (by-ref, ADR-0005).

---

## 8.3 Must-parse L0 (начальный список)

| ID | Имя | Суть |
|----|-----|------|
| MP001 | kw_as_names | `DCL (IF, THEN, ELSE)` и `IF THEN THEN = ELSE;` |
| MP002 | select_stub | SELECT/WHEN/OTHERWISE (exec может быть later) |
| MP003 | dcl_based_stub | BASED/POINTER в DECLARE без codegen |
| MP004 | multi_end | классическое закрытие scopes (по Classic LRM) |
| MP005 | label_prefix | метки перед statements |

### MP001 (текст эталона)

```pli
X: PROC OPTIONS(MAIN);
  DCL (IF, THEN, ELSE) FIXED BIN(31);
  IF THEN THEN = ELSE;
  ELSE ELSE = THEN;
END X;
```

*(Семантика выполнения MP001 — не L0 must-run, пока не закроем модель и не решим, исполнять ли такой код на L0.)*

---

## 8.4 Out-of-scope (реестр-заготовка)

| Тема | Почему |
|------|--------|
| Multitasking / TASK / EVENT | вне G-горизонта продукта |
| CICS/DL/I preprocessor | Enterprise/mainframe |
| Record I/O industrial | не L0–L2 приоритет |
| Полный ON-unit зоопарк | позже / урезать отдельно |

---

## 8.5 Следующий шаг

Развернуть MR003–MR007 в полные тексты после черновика глав 3.1 и 4.1–4.2.  
Не добавлять Enterprise-примеры «на вырост».
