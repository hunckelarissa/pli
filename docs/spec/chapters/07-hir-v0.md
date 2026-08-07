# 7.x HIR v0 (черновик узлов)

**Статус:** черновик по [ADR-0007](../../decisions/ADR-0007-hir-v0.md)  
**Схема:** `hir_v0`

Нормативная детализация полей — до начала `codegen_c`. Здесь — законченный минимум для согласования.

---

## Pipeline

```text
AST + sema → HIR (hir_v0) → codegen_c → .c + libpl1rt
```

Если конструкция must-parse, но не must-run: **не** создавать HIR-узел исполнения; diagnostic `not-implemented`.

---

## Узлы

### Module
- `procs: [Proc]`
- `source_path: string`

### Proc
- `name: string`
- `is_main: bool`
- `params: [Param]` — каждый: name + Type + (всегда ref)
- `body: Block`

### Block
- `decls: [Decl]`
- `stmts: [Stmt]`

### Decl
- `name`, `ty: Type`, `storage: Auto | Static | Param`
- `init: Option<Expr>` (L0: ограниченно)

### Stmt
`Assign | If | Do | Call | PutList | Return | Stop | Block`

### Expr
`Name | Literal | Binary | Unary | Call` (call-in-expr — осторожно на L0)

### Type (L0)
`FixedBin { precision: u32 }` | `Float` | `Char { len: u32 }`

### PutList
- `items: [Expr | StringLiteral]`
- `skip: bool` (для PUT SKIP LIST)

---

## Инварианты

1. Имена после sema уникальны в scope (правила scope — глава 2.2).  
2. Типы на Assign согласованы или имеют L0 conversion rule (глава 3.5).  
3. Call args соответствуют Param count/types.  
4. Codegen не видит keywords-as-names как отдельную проблему — только `Name`.

---

## Версии

Ломающее изменение → `hir_v1` + новый ADR, не молчаливый edit этого файла вразрез с эмиттером.
