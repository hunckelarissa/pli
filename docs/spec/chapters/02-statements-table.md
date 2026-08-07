# 2.3 Таблица statements (L0–G)

**Статус:** черновик SPEC  
**Связь:** [`00-g-compliance-matrix.md`](00-g-compliance-matrix.md), главы 2/4/5

Статусы: must-run | must-parse | n-i (not-implemented) | oos (out-of-scope).

| Statement / форма | L0 | L1 | L2/G | Примечание |
|-------------------|----|----|------|------------|
| assignment | must-run | | | ch.3.5 |
| `CALL` | must-run | | | same module; by-ref |
| `RETURN;` | must-run | | | |
| `RETURN(expr)` | n-i | must-run | | |
| `STOP` | must-run | | | |
| `IF` / `THEN` / `ELSE` | must-run | | | ch.4.1 |
| `DO;` … `END;` | must-run | | | |
| `DO v = e TO e [BY e]` | must-run | | | |
| `DO WHILE` / `UNTIL` | n-i | must-run | | |
| `SELECT` / `WHEN` / `OTHERWISE` | must-parse | must-run | | |
| `GOTO` | n-i | must-run | | |
| `LEAVE` / `ITERATE` | n-i | must-run | | если в G/classic целевого пересечения |
| `DECLARE` / `DCL` | must-run | | | L0 types only |
| `PROCEDURE` / `PROC` | must-run | | | |
| `ENTRY` | must-parse | must-run | | |
| `BEGIN` | must-run | | | |
| `END` / `END name` | must-run | multi-END | | ch.2.2 |
| `PUT LIST` | must-run | | | |
| `PUT SKIP LIST` | must-run | | | |
| `PUT EDIT` / `DATA` | n-i | n-i | must-run | |
| `GET LIST` | n-i | must-run | | |
| `GET EDIT` / `DATA` | n-i | n-i | must-run | |
| `OPEN` / `CLOSE` / `READ` / `WRITE` | n-i | n-i | limited | |
| `ON` / `SIGNAL` / `REVERT` | n-i | n-i | limited | |
| `DISPLAY` | n-i | optional | | |
| `ALLOCATE` / `FREE` | n-i | BASED | | |
| `DEFAULT` | n-i | must-parse | | |
| `%INCLUDE` | n-i | must-run | | ADR-0009 |
| `%REPLACE` | n-i | must-run | | |
| `%IF` / `%DO` | n-i | n-i | must-run | |
| `FORMAT` | n-i | n-i | | |
| `WAIT` / tasking | oos | oos | oos | |

Пустые ячейки = «наследует левый уровень» или не применимо.
