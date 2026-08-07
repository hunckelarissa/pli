# ADR-0001: Product direction (baseline)

- **Status:** Accepted  
- **Date:** 2026-08-05  
- **Decides:** baseline closed by MANIFEST (D-series remains for engineering choices)

## Context

Нужна зафиксированная продуктовая рамка до ТЗ/SPEC/кода, чтобы не спорить снова о GCC frontend, Enterprise и «сначала весь парсер без эмита».

## Decision

1. Горизонт совместимости — **Subset G**, не Enterprise / не full ANSI как KPI.  
2. Вкус языка — **классический PL/I**, keywords-as-names в парсере с L0.  
3. Оси: **широкий parse / узкий execute**; not-implemented явный.  
4. Backend L0 — **транспиляция в C** + runtime на C; GCC/Clang только как `cc`.  
5. `pl1gcc` и сырые PDF — только `research/`; чтение законов языка — `docs/`.  
6. LLVM / native — позже, через отдельные ADR; не условие L0.  
7. Смена фундамента — только новым ADR.

## Consequences

- OPEN.md не пересматривает эти пункты без supersede этого ADR.  
- Инженерные выборы (язык фронта, парсер, лицензия, параметры) остаются в OPEN D1–D7.  
- ТЗ и SPEC обязаны цитировать это ADR как продуктовую рамку.

## References

- `MANIFEST.md`  
- `STACK.md`  
- `docs/tz/TZ.md`
