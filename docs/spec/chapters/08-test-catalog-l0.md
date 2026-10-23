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
| MR004 | *(зарезервирован / слит с MR009)* | — | — |
| MR005 | char_assign | CHAR(n) присваивание и PUT | строка |
| MR006 | static_auto | STATIC vs AUTOMATIC (базово) | различимое поведение |
| MR007 | nested_begin | BEGIN/END внутри PROC | выполняется тело |
| MR008 | call_ref | CALL + by-ref increment | значение после CALL увеличено |
| MR009 | do_to | DO I = 1 TO 3 + PUT | ожидаемая последовательность |
| MR010 | if_else | IF/ELSE ветка | 'f' или 't' по условию |
| MR011 | char_pad | CHAR truncate/pad | фиксированная ширина |
| MR012 | init | `INIT` / `INITIAL` | стартовые значения |
| MR013 | do_group | `DO; … END;` | тело один раз |
| MR014 | leave | `LEAVE` из DO | ранний выход |
| MR015 | float | FLOAT BIN arith | `PUT` числа |
| MR016 | iterate | `ITERATE` в DO | пропуск итерации |
| MR017 | return | `RETURN;` из MAIN | exit 0 |
| MR018 | stop | `STOP;` | exit 0 |
| MR019 | goto | `GOTO` / label: | прыжок |
| MR020 | multi_end | multi-`END name` | закрытие scopes |
| MR021 | get_list | `GET LIST` + stdin | сумма / CHAR |
| MR022 | do_while | `DO WHILE` | цикл |
| MR023 | do_until | `DO UNTIL` | цикл |
| MR024 | select | `SELECT (expr)` | WHEN/OTHERWISE |
| MR025 | put_edit | `PUT EDIT` F/A/X | форматный вывод |
| MR026 | select_truth | `SELECT;` WHEN truth | без expr |
| MR027 | array | 1D FIXED `A(i)=` | subscript |
| MR028 | get_edit | `GET EDIT` F/A/X | fixed-width stdin |
| MR029 | array2d | 2D + RANGE `B(0:2)` | multi-dim |
| MR030 | put_data | `PUT DATA (names)` | `A=1 B=2` |
| MR031 | char_array | CHAR array subscript | pad + PUT |
| MR032 | get_data | `GET DATA (names)` | `A=3 B=4` → сумма |
| MR033 | do_by | `DO I = 1 TO 5 BY 2` | `135` |
| MR034 | put_data_all | `PUT DATA;` all scalars | `A=1 B=2` |
| MR035 | get_data_all | `GET DATA;` any-order fields | сумма |
| MR036 | do_by_neg | `DO I = 5 TO 1 BY -2` | `531` |
| MR037 | put_edit_e_col | `PUT EDIT` E + COLUMN | scientific + col pad |
| MR038 | array_init | `DCL A(3) … INIT(10,20,30)` | element list |
| MR039 | array_assign | aggregate `B = A` | memcpy elements |
| MR040 | put_edit_page_line | SKIP(n) / PAGE / LINE | control spacing |
| MR041 | get_edit_e | `GET EDIT` E(w) | scientific field |
| MR042 | begin_data | DATA in nested BEGIN + shadow | outer/inner names |
| MR043 | picture | `PUT EDIT` P'ZZ9' / P('999') | ` 42 042` |
| MR044 | sysprint | `PUT FILE(SYSPRINT) LIST` | `ok` |
| MR045 | picture_rich | P `$` / `,` / `V.99` | `$123.45  1,234` |
| MR046 | user_file | OPEN/PUT/GET/CLOSE FILE | round-trip `hi` |
| MR047 | on_endfile | `ON ENDFILE(SYSIN) GOTO` + GET LIST EOF | `123eof` |
| MR048 | revert | `REVERT ENDFILE` clears ON handler | `70ok` |
| MR049 | open_title | `OPEN … TITLE('path')` (+ ENV parse) | round-trip `hi` |
| MR050 | on_endfile_file | `ON ENDFILE(f) BEGIN` + GET FILE EOF | `123eof` |
| MR051 | signal_endfile | `SIGNAL ENDFILE` raises ON unit | `aok` |
| MR052 | edit_file | PUT/GET EDIT FILE(f) round-trip | `42` / `ABC` |
| MR053 | on_snap | `ON ENDFILE SNAP` then GOTO override | `aok` |
| MR054 | data_file | PUT/GET DATA FILE(f) round-trip | `7` |
| MR055 | on_system | `ON ENDFILE SYSTEM` → exit 1 | `a` + exit 1 |
| MR056 | structure | STRUCTURE `S.A` / `S.B` | `3` |
| MR057 | like | `DCL 1 T LIKE S` layout copy | `7` |
| MR058 | structure_char | STRUCTURE CHAR member | `7` / `ok` |
| MR059 | structure_nest | nested STRUCTURE `S.N.A` | `7` |
| MR060 | structure_float | STRUCTURE FLOAT members | `4` |
| MR061 | structure_assign | aggregate `T = S` (LIKE) | `7` |
| MR062 | based_allocate | `ALLOCATE`/`FREE` BASED FIXED | `42` |
| MR063 | defined_overlay | `DEFINED` same FIXED overlay | `7` |
| MR064 | pointer_null | POINTER `= NULL` / copy + BASED | `3` |
| MR065 | based_char_float | BASED CHAR/FLOAT ALLOCATE | `ok` / `4` |
| MR066 | defined_based_char | DEFINED CHAR + DEFINED on BASED | `hi` / `5` |
| MR067 | pointer_ne | POINTER `<>` / `=` NULL | `1` |
| MR068 | based_structure | BASED STRUCTURE ALLOCATE | `7` |
| MR069 | addr | `P = ADDR(X)` + BASED overlay | `8` |
| MR070 | based_struct_nest | BASED nested `S.N.A` | `7` |
| MR071 | based_like | BASED STRUCTURE `LIKE` | `7` |
| MR072 | addr_structure | `ADDR(S)` + BASED LIKE overlay | `7` |
| MR073 | addr_char | `ADDR(C)` + BASED CHAR | `ok` |
| MR074 | pointer_init | `POINTER INIT(NULL)` | `1` |
| MR075 | bit | `BIT(1)` assign / PUT | `1` |
| MR076 | bit_lit | `'1'B` + `IF B` | `1` |
| MR077 | based_bit | BASED `BIT` ALLOCATE | `1` |
| MR078 | bit8 | `BIT(8)` + `'00000101'B` | `5` |
| MR079 | bit_init | `BIT INIT('1'B)` | `1` |

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
| MP001 | kw_as_names | `DCL (IF, THEN, ELSE)` и `IF THEN THEN THEN = ELSE;` |
| MP002 | select_stub | SELECT/WHEN/OTHERWISE (exec: MR024/026) |
| MP003 | dcl_based_stub | BASED/POINTER/BIT в DECLARE без codegen |
| MP004 | multi_end | классическое закрытие scopes (по Classic LRM) |
| MP005 | label_prefix | метки перед statements |
| MP006 | get_list_stub | GET LIST parse (exec: MR021) |
| MP007 | put_edit_stub | PUT/GET EDIT parse (exec: MR025/028) |
| MP008 | put_data_stub | PUT/GET DATA parse (exec: MR030/032) |
| MP009 | array_dcl_stub | array DCL parse (exec: MR027/029/031) |
| MP010 | array_assign_stub | aggregate `A = B` parse (exec: MR039) |
| MP011 | picture_stub | `P'…'` / `P('…')` edit parse |
| MP012 | on_endfile_stub | `ON ENDFILE` / `OPEN`/`CLOSE FILE` |
| MP013 | signal_stub | `SIGNAL` condition parse |
| MP014 | file_io_stub | `FILE(f)` GET/PUT + `REVERT` |
| MP015 | structure_stub | STRUCTURE levels + `LIKE` (parse-only) |
| MP016 | defined_based | `DEFINED`/`DEF` + `ALLOCATE`/`FREE` (parse-only) |

### Must-fail

| ID | Имя | Суть |
|----|-----|------|
| MF001 | pct_include | `%…` отвергается на L0 (ADR-0009) |

### MP001 (текст эталона)

```pli
X: PROC OPTIONS(MAIN);
  DCL (IF, THEN, ELSE) FIXED BIN(31);
  IF THEN THEN THEN = ELSE;
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

Каталог синхронизирован с `tests/` на **MR035 / MP009 / MF001** (2026-10-21).  
Дальше: negative BY; EDIT E/COLUMN; не добавлять Enterprise-примеры «на вырост».
