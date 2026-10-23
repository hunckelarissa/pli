//! HIR v0 — stable input for codegen_c (ADR-0007).

#[derive(Debug, Clone)]
pub struct Module {
    pub source_path: String,
    pub procs: Vec<Proc>,
}

#[derive(Debug, Clone)]
pub struct Proc {
    pub name: String,
    pub is_main: bool,
    /// Parameter names in order (by-reference, ADR-0005).
    pub params: Vec<String>,
    pub body: Block,
}

#[derive(Debug, Clone, Default)]
pub struct Block {
    pub decls: Vec<Decl>,
    pub stmts: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub struct Decl {
    pub name: String,
    pub ty: Type,
    pub storage: Storage,
    /// INITIAL/INIT value list (scalar: one literal; array: element literals).
    pub init: Option<Vec<Expr>>,
    /// BASED[(ptr)] — parse-only on L0 (MP003).
    pub based_on: Option<String>,
    /// Array bounds after name (`A(10)`, `B(1:5)`).
    pub dims: Option<Vec<ArrayBound>>,
    /// Structure level number (`1 S, 2 A …`) — parse-only (MP015).
    pub level: Option<u32>,
    /// `LIKE other` — parse-only (MP015).
    pub like_of: Option<String>,
    /// `DEFINED(base)` / `DEF(base)` — overlay alias (MR063).
    pub defined_on: Option<String>,
}

#[derive(Debug, Clone)]
pub enum ArrayBound {
    /// `(n)` — extent 1:n (classic default).
    Upper(i64),
    /// `(lo:hi)`
    Range { from: i64, to: i64 },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    FixedBin { precision: u32 },
    Char { len: u32 },
    Float,
    /// Parse-only on L0.
    Bit { len: u32 },
    /// Parse-only on L0.
    Pointer,
    /// Major/minor structure node (no elementary type) — parse-only (MP015).
    Struct,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Storage {
    Automatic,
    Static,
    Parameter,
    Based,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    /// One or more `label:` prefixes on a statement (MP005).
    Labeled {
        labels: Vec<String>,
        body: Box<Stmt>,
    },
    Assign { name: String, value: Expr },
    /// `A(i) = expr;` / `A(i,j) = expr;`
    AssignIndex {
        name: String,
        indices: Vec<Expr>,
        value: Expr,
    },
    /// `S.A = expr;` structure member assign (MR056).
    AssignField {
        path: Vec<String>,
        value: Expr,
    },
    If {
        cond: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    DoCounted {
        var: String,
        from: Expr,
        to: Expr,
        step: Option<Expr>,
        body: Vec<Stmt>,
    },
    /// `DO; … END;` — execute body once (L0 must-run).
    DoGroup { body: Vec<Stmt> },
    /// `DO WHILE (expr); … END;`
    DoWhile { cond: Expr, body: Vec<Stmt> },
    /// `DO UNTIL (expr); … END;`
    DoUntil { cond: Expr, body: Vec<Stmt> },
    /// Exit innermost DO group / counted DO.
    Leave,
    /// Next iteration of innermost DO (counted) / no-op end of DO group.
    Iterate,
    /// `RETURN;` — leave current procedure (MAIN → exit 0).
    Return,
    /// `STOP;` — terminate process (exit 0).
    Stop,
    /// `GOTO label;` / `GO TO label;`
    Goto { label: String },
    /// Null statement `;`
    Null,
    /// `GET [FILE(f)] [SKIP] LIST(...);`
    GetList {
        file: Option<String>,
        skip: bool,
        items: Vec<String>,
    },
    /// `PUT [FILE(f)] [SKIP] EDIT (...) (...);`
    PutEdit {
        file: Option<String>,
        skip: bool,
        items: Vec<Expr>,
        formats: Vec<FormatItem>,
    },
    /// `GET [FILE(f)] [SKIP] EDIT (...) (...);`
    GetEdit {
        file: Option<String>,
        skip: bool,
        items: Vec<String>,
        formats: Vec<FormatItem>,
    },
    /// `PUT [FILE(f)] [SKIP] DATA [(names)];`
    PutData {
        file: Option<String>,
        skip: bool,
        items: Option<Vec<String>>,
    },
    /// `GET [FILE(f)] [SKIP] DATA [(names)];`
    GetData {
        file: Option<String>,
        skip: bool,
        items: Option<Vec<String>>,
    },
    /// `PUT [FILE(f)] [SKIP] LIST(...);`
    PutList {
        file: Option<String>,
        skip: bool,
        items: Vec<Expr>,
    },
    Call { name: String, args: Vec<Expr> },
    Begin { body: Block },
    /// `SELECT [(expr)]; WHEN … OTHERWISE … END;`
    Select {
        expr: Option<Expr>,
        whens: Vec<WhenClause>,
        otherwise: Option<Box<Stmt>>,
    },
    /// `ON condition [(ref)] …;` — parse-only on L0 (MP012).
    On {
        condition: String,
        /// e.g. file constant in `ON ENDFILE(SYSIN)`.
        refer: Option<String>,
        action: OnAction,
    },
    /// `OPEN FILE(name) [attrs…] [TITLE('path')] [ENV(…)];`
    Open {
        file: String,
        options: Vec<String>,
        /// OS path from `TITLE('…')`; None → use `file` as path.
        title: Option<String>,
    },
    /// `CLOSE FILE(name);` — parse-only stub.
    Close { file: String },
    /// `SIGNAL condition [(ref)];` — parse-only stub.
    Signal {
        condition: String,
        refer: Option<String>,
    },
    /// `REVERT condition [(ref)];` — parse-only stub.
    Revert {
        condition: String,
        refer: Option<String>,
    },
    /// `ALLOCATE name [SET(ptr)];` — BASED storage (MR062).
    Allocate {
        name: String,
        set: Option<String>,
    },
    /// `FREE name;` — BASED storage (MR062).
    Free { name: String },
}

#[derive(Debug, Clone)]
pub enum OnAction {
    /// `ON … ;` — null ON-unit
    Null,
    /// `SNAP` / `SNAP SYSTEM`
    Snap { system: bool },
    /// bare `SYSTEM;`
    System,
    Goto { label: String },
    Stmt(Box<Stmt>),
}

#[derive(Debug, Clone)]
pub enum FormatItem {
    /// Bare format code: `A`, `SKIP`, …
    Name(String),
    /// Format with args: `F(5)`, `X(2)`, …
    Call { name: String, args: Vec<Expr> },
}

#[derive(Debug, Clone)]
pub struct WhenClause {
    pub exprs: Vec<Expr>,
    pub action: Box<Stmt>,
}

#[derive(Debug, Clone)]
pub enum Expr {
    StringLit(String),
    IntLit(i64),
    FloatLit(f64),
    /// `NULL` / `NULL()` enquiry (pointer null).
    Null,
    /// `'1010'B` bit string literal.
    BitLit { value: u64, width: u32 },
    /// `ADDR(name)` — address of a variable (MR069).
    Addr { name: String },
    Name(String),
    /// Array element `A(i)` / `A(i,j)`.
    Index {
        base: String,
        indices: Vec<Expr>,
    },
    /// Structure member `S.A` (path length ≥ 2).
    Field { path: Vec<String> },
    Binary {
        op: BinOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    BitAnd,
    BitOr,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}
