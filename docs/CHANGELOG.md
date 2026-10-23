# Changelog — pl1c / libpl1rt

## 0.1.0-dev (2026-10-21)

First vertical L0/L1-start slice of the PL/I-G toolchain (`pl1c` → C + `libpl1rt`).

### Must-run

- **MR001–003, 005–079** (+ **MP016**): control flow (`IF`, `SELECT`, `DO`/`BY` (incl. negative)/`WHILE`/`UNTIL`, `GOTO`, `LEAVE`/`ITERATE`, `RETURN`/`STOP`), `BEGIN` (incl. nested DATA visibility), `CALL` by-ref, `CHAR` pad/trunc (+ CHAR arrays), `BIT(1..8)` + `'…'B` (+ BASED / INIT), scalar/`array INIT`, aggregate array/STRUCTURE assign, `FLOAT`, STRUCTURE levels 1..=3 (FIXED/FLOAT/CHAR) + `S.N.A` / `LIKE` / `T = S`, `POINTER` + `NULL`/`NULL()`/`ADDR`/`INIT(NULL)`/`<>` + assign, `BASED(P)` FIXED/FLOAT/CHAR/BIT/STRUCTURE(+nest/LIKE) + `ALLOCATE`/`FREE`, `DEFINED`/`DEF` scalar (incl. CHAR / on BASED), `OPEN`/`CLOSE FILE` + `TITLE('path')`/`ENV(…)` + `PUT`/`GET FILE(f)` LIST/EDIT/DATA (fopen), edit formats F/A/E/X/COLUMN/PAGE/LINE/SKIP/P ($/V/comma), `ON ENDFILE` (GOTO/BEGIN/SNAP/null/SYSTEM→exit) / `SIGNAL`/`REVERT ENDFILE` (setjmp), arrays (FIXED/FLOAT/CHAR, 1D/2D, RANGE) with subscripts. Must-run may set sibling `.exitcode`.

### Must-parse / must-fail

- **MP001–016**: keywords-as-names, SELECT/GET/PUT stubs, BASED/BIT/POINTER/`DEFINED`, `ALLOCATE`/`FREE`, multi-`END`, labels, array DCL, ON/SIGNAL/FILE, STRUCTURE/`LIKE` parse stubs.
- **MF001**: `%` preprocessor rejected on L0 (ADR-0009 / ADR-0011).

### Tooling

- Version string: [`VERSION`](../VERSION) → `pl1c --version`.
- Gates: `tools/run_must_run.ps1`, `run_must_parse.ps1`, `run_must_fail.ps1`.
