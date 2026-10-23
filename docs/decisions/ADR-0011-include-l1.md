# ADR-0011: `%INCLUDE` on L1

- **Status:** Accepted (forward plan)
- **Date:** 2026-10-21
- **Decides:** when/how `%INCLUDE` becomes executable
- **Supersedes in part:** deferred clause of [ADR-0009](ADR-0009-preprocessor-l0.md)

## Context

Subset G requires `%INCLUDE` (textual inclusion at compile time). ADR-0009 keeps the L0 preprocessor non-executing so the language core is not blocked. L0 now **recognizes** `%…;` and returns a clear not-implemented diagnostic (no silent skip).

## Decision

1. **L0 (current):** any `%directive …;` → diagnostic  
   `preprocessor %… is not implemented on L0` (ADR-0009). Must-run/must-parse suites stay free of `%`.
2. **L1:** implement **only** `%INCLUDE` (Subset G surface):
   - Forms: `%INCLUDE 'path';` and `%INCLUDE name;` (search path TBD in SPEC).
   - Semantics: textual splice **before** lex of the including unit (or equivalent single token stream); nested includes allowed with a cycle/depth guard.
   - Included text is subject to the same keywords-as-names / statement rules as the host file.
3. **Still deferred past L1:** `%REPLACE`, `%IF`/`%DO`/`%END`, `%ACTIVATE`, procedure-like preprocessor, `*PROCESS`.

## Consequences

- Negative test `tests/must-fail/L0_MF_001_pct_include.pli` locks the L0 diagnostic.
- L1 work item: driver-level include expander + search paths + SPEC ch. preprocessor.
- No change to hir_v0 until include is expanded away (preprocessor remains outside HIR).

## References

- ADR-0009
- ANSI X3.74 Subset G §10.1 `%INCLUDE`
- `docs/spec/chapters/00-g-compliance-matrix.md`
