# NEXT — статус после скелета

**Дата:** 2026-10-21  
**Версия инструмента:** `0.1.0-dev` ([`VERSION`](../VERSION), `pl1c --version`)

---

## Сделано

- Документальный берег L0 (SPEC / ADR / TZ)
- Скелет: `compiler/` (`pl1c`), `runtime/` (`libpl1rt`)
- **MR001–003, 005–079** OK; **MP001–016** (+ BIT(8) / INIT / BASED BIT)
- Changelog: [`CHANGELOG.md`](CHANGELOG.md) for `0.1.0-dev`
- **MF001** `%` n-i
- Loop `/loop 5m продолжай` — активен (PID 24092, 300s)
- must-run: optional `.exitcode` sibling (default 0)

## Текущий срез языка в коде

`PROC[(params)] OPTIONS(MAIN)?` · `DCL FIXED BIN` / `FLOAT` / `CHAR(n)` / `BIT(1..8)` + `'…'B` / arrays (FIXED/FLOAT/CHAR) · `POINTER` + `NULL`/`NULL()`/`ADDR`/`INIT(NULL)` + assign · `BASED(P)` FIXED/FLOAT/CHAR/BIT/STRUCTURE(+nest/LIKE) + `ALLOCATE`/`FREE` · `DEFINED`/`DEF` scalar (incl. on BASED / CHAR) · STRUCTURE levels 1..=3 (FIXED/FLOAT/CHAR) + `S.N.A` / `LIKE` / aggregate `T = S` · `STATIC`/`AUTO` · `INIT`/`INITIAL` · label: · `GOTO`/`GO TO` · `;` null · assign / `A(i)=` · arith/relops · `IF` · `SELECT`/`SELECT;` · `DO;` / `DO WHILE` / `DO UNTIL` / `DO…TO`/`BY` · `LEAVE`/`ITERATE` · `RETURN`/`STOP` · `BEGIN` · `OPEN`/`CLOSE FILE` + `TITLE('path')` / `ENV(…)` + `PUT`/`GET FILE(f)` LIST/EDIT/DATA · edit F/A/E/X/COLUMN/PAGE/LINE/SKIP/P($,V,comma) · `ON ENDFILE` (GOTO/BEGIN/SNAP/null/SYSTEM→exit) / `SIGNAL`/`REVERT ENDFILE` · `CALL` (by-ref) · `END` / multi-`END name` · `%…` rejected (L0) · keywords-as-names (parse)

## Дальше (код)

1. BIT `&` / `|` ops stub  
2. BIT `^` compare polish  
3. DEFINED BIT overlay polish

```powershell
powershell -File tools/run_must_run.ps1
```

## Версия

1. [`VERSION`](../VERSION)  
2. `pl1c --version`
