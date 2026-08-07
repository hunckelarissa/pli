# ADR-0006: CLI and runtime naming

- **Status:** Accepted
- **Date:** 2026-08-05
- **Decides:** D4

## Context

Имя `plic` занято Iron Spring. Нужны уникальные имена артефактов.

## Decision

| Артефакт | Имя |
|----------|-----|
| Компилятор (CLI) | **`pl1c`** |
| Runtime library | **`libpl1rt`** (файл: `libpl1rt.a` / `pl1rt.lib`) |
| Диалект в сообщениях (до бренда) | **`PL/I-G`** (working title; см. также D10) |
| Расширение исходников | **`.pli`** (также принимать `.pl1`) |
| Эмит C по умолчанию | `*.c` рядом или в `-o` / `--outdir` (уточнить в CLI SPEC) |

Примеры:

```text
pl1c hello.pli -o hello
pl1c hello.pli --emit-c hello.c
```

## Consequences

- Не конфликтовать с Iron Spring `plic`.
- Документация и тесты используют `pl1c` / `libpl1rt`.

## References

- OPEN D4, D10
