//! Recursive-descent parser — L0 slice (hello + DCL/assign/arith).

use crate::diagnostic::{Diagnostic, Result};
use crate::hir::{
    ArrayBound, BinOp, Block, Decl, Expr, FormatItem, Module, OnAction, Proc, Stmt, Storage, Type,
    WhenClause,
};
use crate::lex::{eq_ignore_ascii_case, Token, TokenKind};

enum EndKind {
    Bare,
    Named(String),
}

struct DeclName {
    name: String,
    dims: Option<Vec<ArrayBound>>,
}

struct StmtParse {
    stmt: Stmt,
    /// `END name` still needs to close an outer labeled scope / procedure.
    pending_end: Option<String>,
}

impl StmtParse {
    fn done(stmt: Stmt) -> Self {
        Self {
            stmt,
            pending_end: None,
        }
    }
}

pub struct Parser<'a> {
    tokens: &'a [Token],
    pos: usize,
    source_path: String,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token], source_path: impl Into<String>) -> Self {
        Self {
            tokens,
            pos: 0,
            source_path: source_path.into(),
        }
    }

    pub fn parse_module(&mut self) -> Result<Module> {
        let mut procs = Vec::new();
        while !self.is_eof() {
            if self.at_punct("%") {
                return Err(self.reject_preprocessor());
            }
            procs.push(self.parse_procedure()?);
        }
        if procs.is_empty() {
            return Err(Diagnostic::error("empty compilation unit"));
        }
        if !procs.iter().any(|p| p.is_main) {
            return Err(Diagnostic::error(
                "no PROCEDURE OPTIONS(MAIN) in compilation unit",
            ));
        }
        Ok(Module {
            source_path: self.source_path.clone(),
            procs,
        })
    }

    fn parse_procedure(&mut self) -> Result<Proc> {
        let name = self.expect_ident()?;
        self.expect_punct(":")?;
        self.expect_kw("PROC")
            .or_else(|_| self.expect_kw("PROCEDURE"))?;

        let mut params = Vec::new();
        if self.at_punct("(") {
            self.bump();
            if !self.at_punct(")") {
                loop {
                    params.push(self.expect_ident()?);
                    if self.at_punct(",") {
                        self.bump();
                        continue;
                    }
                    break;
                }
            }
            self.expect_punct(")")?;
        }

        let mut is_main = false;
        if self.at_kw("OPTIONS") {
            self.bump();
            self.expect_punct("(")?;
            self.expect_kw("MAIN")?;
            self.expect_punct(")")?;
            is_main = true;
        }
        self.expect_punct(";")?;

        let mut decls = Vec::new();
        let mut stmts = Vec::new();
        loop {
            if self.at_punct("%") {
                return Err(self.reject_preprocessor());
            }
            if self.at_kw("END") {
                match self.parse_end()? {
                    EndKind::Bare => {}
                    EndKind::Named(end_name) => {
                        if !eq_ignore_ascii_case(&end_name, &name) {
                            return Err(self.err(format!(
                                "END name `{end_name}` does not match procedure `{name}`"
                            )));
                        }
                    }
                }
                break;
            }
            if self.at_kw("DCL") || self.at_kw("DECLARE") {
                decls.extend(self.parse_declare()?);
                continue;
            }
            let sp = self.parse_statement()?;
            stmts.push(sp.stmt);
            if let Some(end_name) = sp.pending_end {
                if !eq_ignore_ascii_case(&end_name, &name) {
                    return Err(self.err(format!(
                        "END name `{end_name}` does not match procedure `{name}`"
                    )));
                }
                break;
            }
        }

        // Mark parameter storage on matching DCL names.
        let param_keys: Vec<String> = params.iter().map(|p| p.to_ascii_uppercase()).collect();
        for d in &mut decls {
            if param_keys.iter().any(|p| p == &d.name.to_ascii_uppercase()) {
                d.storage = Storage::Parameter;
            }
        }

        Ok(Proc {
            name,
            is_main,
            params,
            body: Block { decls, stmts },
        })
    }

    fn parse_declare(&mut self) -> Result<Vec<Decl>> {
        if self.at_kw("DCL") {
            self.bump();
        } else {
            self.expect_kw("DECLARE")?;
        }

        if matches!(self.peek().kind, TokenKind::IntLit(_)) {
            return self.parse_structure_declare();
        }

        let items = if self.at_punct("(") {
            // Lookahead: `(A, B)` name list vs cannot be dims without a name.
            self.bump();
            let mut items = Vec::new();
            loop {
                items.push(self.parse_decl_name()?);
                if self.at_punct(",") {
                    self.bump();
                    continue;
                }
                break;
            }
            self.expect_punct(")")?;
            items
        } else {
            vec![self.parse_decl_name()?]
        };

        let mut storage = Storage::Automatic;
        let mut based_on = None;
        if self.at_kw("STATIC") {
            self.bump();
            storage = Storage::Static;
        } else if self.at_kw("AUTOMATIC") || self.at_kw("AUTO") {
            self.bump();
        } else if self.at_kw("BASED") {
            self.bump();
            storage = Storage::Based;
            based_on = self.parse_based_on()?;
        }
        let ty = self.parse_type_attrs()?;
        let mut init = None;
        let mut defined_on = None;
        loop {
            if self.at_kw("STATIC") {
                self.bump();
                storage = Storage::Static;
                continue;
            }
            if self.at_kw("AUTOMATIC") || self.at_kw("AUTO") {
                self.bump();
                storage = Storage::Automatic;
                continue;
            }
            if self.at_kw("BASED") {
                self.bump();
                storage = Storage::Based;
                based_on = self.parse_based_on()?;
                continue;
            }
            if self.at_kw("DEFINED") || self.at_kw("DEF") {
                self.bump();
                self.expect_punct("(")?;
                defined_on = Some(self.expect_ident()?);
                self.expect_punct(")")?;
                continue;
            }
            if self.at_kw("INIT") || self.at_kw("INITIAL") {
                if init.is_some() {
                    return Err(self.err("duplicate INIT/INITIAL attribute"));
                }
                self.bump();
                self.expect_punct("(")?;
                let mut vals = Vec::new();
                loop {
                    vals.push(self.parse_expr()?);
                    if self.at_punct(",") {
                        self.bump();
                        continue;
                    }
                    break;
                }
                self.expect_punct(")")?;
                init = Some(vals);
                continue;
            }
            break;
        }
        self.expect_punct(";")?;

        Ok(items
            .into_iter()
            .map(|item| Decl {
                name: item.name,
                ty: ty.clone(),
                storage,
                init: init.clone(),
                based_on: based_on.clone(),
                dims: item.dims,
                level: None,
                like_of: None,
                defined_on: defined_on.clone(),
            })
            .collect())
    }

    /// `DCL 1 S, 2 A FIXED BIN(31), 2 B CHAR(4);` / `DCL 1 T LIKE S;`
    fn parse_structure_declare(&mut self) -> Result<Vec<Decl>> {
        let mut decls = Vec::new();
        loop {
            let level = self.expect_int_lit()?;
            if level <= 0 || level > 255 {
                return Err(self.err("structure level out of range"));
            }
            let level = level as u32;
            let name = self.expect_ident()?;
            let dims = if self.at_punct("(") {
                Some(self.parse_array_bounds()?)
            } else {
                None
            };
            let mut storage = Storage::Automatic;
            let mut based_on = None;
            let mut like_of = None;
            let mut defined_on = None;
            let mut ty = Type::Struct;
            let mut init = None;
            let mut saw_elem_ty = false;
            loop {
                if self.at_kw("LIKE") {
                    self.bump();
                    like_of = Some(self.expect_ident()?);
                    continue;
                }
                if self.at_kw("DEFINED") || self.at_kw("DEF") {
                    self.bump();
                    self.expect_punct("(")?;
                    defined_on = Some(self.expect_ident()?);
                    self.expect_punct(")")?;
                    continue;
                }
                if self.at_kw("STATIC") {
                    self.bump();
                    storage = Storage::Static;
                    continue;
                }
                if self.at_kw("AUTOMATIC") || self.at_kw("AUTO") {
                    self.bump();
                    storage = Storage::Automatic;
                    continue;
                }
                if self.at_kw("BASED") {
                    self.bump();
                    storage = Storage::Based;
                    based_on = self.parse_based_on()?;
                    continue;
                }
                if self.at_kw("INIT") || self.at_kw("INITIAL") {
                    if init.is_some() {
                        return Err(self.err("duplicate INIT/INITIAL attribute"));
                    }
                    self.bump();
                    self.expect_punct("(")?;
                    let mut vals = Vec::new();
                    loop {
                        vals.push(self.parse_expr()?);
                        if self.at_punct(",") {
                            self.bump();
                            continue;
                        }
                        break;
                    }
                    self.expect_punct(")")?;
                    init = Some(vals);
                    continue;
                }
                if self.at_type_kw() {
                    ty = self.parse_type_attrs()?;
                    saw_elem_ty = true;
                    continue;
                }
                break;
            }
            if like_of.is_some() && !saw_elem_ty {
                ty = Type::Struct;
            }
            decls.push(Decl {
                name,
                ty,
                storage,
                init,
                based_on,
                dims,
                level: Some(level),
                like_of,
                defined_on,
            });
            if self.at_punct(",") {
                self.bump();
                continue;
            }
            break;
        }
        self.expect_punct(";")?;
        Ok(decls)
    }

    fn at_type_kw(&self) -> bool {
        self.at_kw("FIXED")
            || self.at_kw("FLOAT")
            || self.at_kw("CHAR")
            || self.at_kw("CHARACTER")
            || self.at_kw("BIT")
            || self.at_kw("POINTER")
            || self.at_kw("PTR")
    }

    fn parse_decl_name(&mut self) -> Result<DeclName> {
        let name = self.expect_ident()?;
        let dims = if self.at_punct("(") {
            Some(self.parse_array_bounds()?)
        } else {
            None
        };
        Ok(DeclName { name, dims })
    }

    fn parse_array_bounds(&mut self) -> Result<Vec<ArrayBound>> {
        self.expect_punct("(")?;
        let mut bounds = Vec::new();
        loop {
            let first = self.expect_int_lit()?;
            if self.at_punct(":") {
                self.bump();
                let to = self.expect_int_lit()?;
                bounds.push(ArrayBound::Range { from: first, to });
            } else {
                bounds.push(ArrayBound::Upper(first));
            }
            if self.at_punct(",") {
                self.bump();
                continue;
            }
            break;
        }
        self.expect_punct(")")?;
        Ok(bounds)
    }

    fn expect_int_lit(&mut self) -> Result<i64> {
        match &self.peek().kind {
            TokenKind::IntLit(n) => {
                let n = *n;
                self.bump();
                Ok(n)
            }
            _ => Err(self.err("expected integer bound")),
        }
    }

    fn parse_based_on(&mut self) -> Result<Option<String>> {
        if !self.at_punct("(") {
            return Ok(None);
        }
        self.bump();
        let name = self.expect_ident()?;
        self.expect_punct(")")?;
        Ok(Some(name))
    }

    fn parse_type_attrs(&mut self) -> Result<Type> {
        if self.at_kw("POINTER") || self.at_kw("PTR") {
            self.bump();
            return Ok(Type::Pointer);
        }
        if self.at_kw("FLOAT") {
            self.bump();
            if self.at_kw("BIN") || self.at_kw("BINARY") {
                self.bump();
            }
            // optional precision FLOAT BIN(53) — accepted, ignored on L0
            if self.at_punct("(") {
                self.bump();
                match &self.peek().kind {
                    TokenKind::IntLit(_) => {
                        self.bump();
                    }
                    _ => return Err(self.err("expected FLOAT precision integer")),
                }
                self.expect_punct(")")?;
            }
            return Ok(Type::Float);
        }
        if self.at_kw("BIT") {
            self.bump();
            let len = if self.at_punct("(") {
                self.bump();
                let n = match &self.peek().kind {
                    TokenKind::IntLit(n) => {
                        let n = *n;
                        self.bump();
                        if n <= 0 || n > 32767 {
                            return Err(self.err("BIT length out of range"));
                        }
                        n as u32
                    }
                    _ => return Err(self.err("expected BIT length integer")),
                };
                self.expect_punct(")")?;
                n
            } else {
                1
            };
            return Ok(Type::Bit { len });
        }
        if self.at_kw("CHAR") || self.at_kw("CHARACTER") {
            self.bump();
            self.expect_punct("(")?;
            let len = match &self.peek().kind {
                TokenKind::IntLit(n) => {
                    let n = *n;
                    self.bump();
                    if n <= 0 || n > 4096 {
                        return Err(self.err("CHAR length out of range"));
                    }
                    n as u32
                }
                _ => return Err(self.err("expected CHAR length integer")),
            };
            self.expect_punct(")")?;
            return Ok(Type::Char { len });
        }

        // FIXED BIN(31)  or  FIXED BINARY(31)
        self.expect_kw("FIXED")?;
        if self.at_kw("BIN") || self.at_kw("BINARY") {
            self.bump();
        } else {
            return Err(self.err("expected BIN/BINARY after FIXED"));
        }
        self.expect_punct("(")?;
        let precision = match &self.peek().kind {
            TokenKind::IntLit(n) => {
                let n = *n;
                self.bump();
                if n <= 0 || n > 64 {
                    return Err(self.err("FIXED BIN precision out of range"));
                }
                n as u32
            }
            _ => return Err(self.err("expected precision integer")),
        };
        self.expect_punct(")")?;
        Ok(Type::FixedBin { precision })
    }

    fn reject_preprocessor(&mut self) -> Diagnostic {
        let _ = self.expect_punct("%");
        let directive = if matches!(self.peek().kind, TokenKind::Ident(_)) {
            self.expect_ident().unwrap_or_else(|_| String::from("?"))
        } else {
            String::from("?")
        };
        while !self.is_eof() && !self.at_punct(";") {
            self.bump();
        }
        if self.at_punct(";") {
            self.bump();
        }
        self.err(format!(
            "preprocessor `%{directive}` is not implemented on L0 (see ADR-0009; %INCLUDE planned for L1 / ADR-0011)"
        ))
    }

    fn parse_statement(&mut self) -> Result<StmtParse> {
        if self.at_punct("%") {
            return Err(self.reject_preprocessor());
        }
        // R4: label prefix — Ident `:` (incl. keyword spellings like THEN:)
        let mut labels = Vec::new();
        while matches!(self.peek().kind, TokenKind::Ident(_)) && self.lookahead_is_label() {
            labels.push(self.expect_ident()?);
            self.expect_punct(":")?;
        }
        if !labels.is_empty() {
            let mut sp = self.parse_statement_core(&labels)?;
            sp.stmt = Stmt::Labeled {
                labels,
                body: Box::new(sp.stmt),
            };
            return Ok(sp);
        }
        self.parse_statement_core(&[])
    }

    fn parse_statement_core(&mut self, labels: &[String]) -> Result<StmtParse> {
        // R2: Ident + `=` is assignment even when the spelling is a keyword
        // (`THEN = ELSE`, `IF = 0`, …). Check before statement-keyword arms.
        if matches!(self.peek().kind, TokenKind::Ident(_)) && self.lookahead_is_assign() {
            let name = self.expect_ident()?;
            self.expect_punct("=")?;
            let value = self.parse_expr()?;
            self.expect_punct(";")?;
            return Ok(StmtParse::done(Stmt::Assign { name, value }));
        }
        // S.A = expr;
        if matches!(self.peek().kind, TokenKind::Ident(_)) && self.lookahead_is_assign_field() {
            let mut path = vec![self.expect_ident()?];
            while self.at_punct(".") {
                self.bump();
                path.push(self.expect_ident()?);
            }
            self.expect_punct("=")?;
            let value = self.parse_expr()?;
            self.expect_punct(";")?;
            return Ok(StmtParse::done(Stmt::AssignField { path, value }));
        }
        // A(i) = expr;  — must see `=` after the subscript list so
        // `SELECT (X);` / `WHEN (1)` are not mistaken for AssignIndex.
        if matches!(self.peek().kind, TokenKind::Ident(_)) && self.lookahead_is_assign_index() {
            let name = self.expect_ident()?;
            let indices = self.parse_expr_list()?;
            self.expect_punct("=")?;
            let value = self.parse_expr()?;
            self.expect_punct(";")?;
            return Ok(StmtParse::done(Stmt::AssignIndex {
                name,
                indices,
                value,
            }));
        }
        if self.at_kw("BEGIN") {
            return self.parse_begin(labels);
        }
        if self.at_kw("SELECT") {
            return self.parse_select(labels);
        }
        if self.at_kw("IF") {
            return self.parse_if();
        }
        if self.at_kw("DO") {
            return self.parse_do(labels);
        }
        if self.at_kw("LEAVE") {
            self.bump();
            self.expect_punct(";")?;
            return Ok(StmtParse::done(Stmt::Leave));
        }
        if self.at_kw("ITERATE") {
            self.bump();
            self.expect_punct(";")?;
            return Ok(StmtParse::done(Stmt::Iterate));
        }
        if self.at_kw("RETURN") {
            self.bump();
            if self.at_punct("(") {
                return Err(self.err("RETURN(expr) not implemented on L0"));
            }
            self.expect_punct(";")?;
            return Ok(StmtParse::done(Stmt::Return));
        }
        if self.at_kw("STOP") {
            self.bump();
            self.expect_punct(";")?;
            return Ok(StmtParse::done(Stmt::Stop));
        }
        if self.at_kw("GOTO") {
            self.bump();
            let label = self.expect_ident()?;
            self.expect_punct(";")?;
            return Ok(StmtParse::done(Stmt::Goto { label }));
        }
        if self.at_kw("GO") {
            self.bump();
            self.expect_kw("TO")?;
            let label = self.expect_ident()?;
            self.expect_punct(";")?;
            return Ok(StmtParse::done(Stmt::Goto { label }));
        }
        if self.at_kw("CALL") {
            return self.parse_call();
        }
        if self.at_kw("PUT") {
            return self.parse_put();
        }
        if self.at_kw("GET") {
            return self.parse_get();
        }
        if self.at_kw("ON") {
            return self.parse_on();
        }
        if self.at_kw("SIGNAL") {
            return self.parse_signal();
        }
        if self.at_kw("REVERT") {
            return self.parse_revert();
        }
        if self.at_kw("OPEN") {
            return self.parse_open();
        }
        if self.at_kw("CLOSE") {
            return self.parse_close();
        }
        if self.at_kw("ALLOCATE") {
            return self.parse_allocate();
        }
        if self.at_kw("FREE") {
            return self.parse_free();
        }
        // Null statement
        if self.at_punct(";") {
            self.bump();
            return Ok(StmtParse::done(Stmt::Null));
        }
        Err(self.err(format!(
            "unsupported statement at current L0 slice (got {:?})",
            self.peek().kind
        )))
    }

    fn parse_on(&mut self) -> Result<StmtParse> {
        self.expect_kw("ON")?;
        let condition = self.expect_ident()?;
        let refer = if self.at_punct("(") {
            self.bump();
            let name = self.expect_ident()?;
            self.expect_punct(")")?;
            Some(name)
        } else {
            None
        };
        if self.at_punct(";") {
            self.bump();
            return Ok(StmtParse::done(Stmt::On {
                condition,
                refer,
                action: OnAction::Null,
            }));
        }
        if self.at_kw("SNAP") {
            self.bump();
            let system = if self.at_kw("SYSTEM") {
                self.bump();
                true
            } else {
                false
            };
            self.expect_punct(";")?;
            return Ok(StmtParse::done(Stmt::On {
                condition,
                refer,
                action: OnAction::Snap { system },
            }));
        }
        if self.at_kw("SYSTEM") {
            self.bump();
            self.expect_punct(";")?;
            return Ok(StmtParse::done(Stmt::On {
                condition,
                refer,
                action: OnAction::System,
            }));
        }
        if self.at_kw("GOTO") {
            self.bump();
            let label = self.expect_ident()?;
            self.expect_punct(";")?;
            return Ok(StmtParse::done(Stmt::On {
                condition,
                refer,
                action: OnAction::Goto { label },
            }));
        }
        if self.at_kw("GO") {
            self.bump();
            self.expect_kw("TO")?;
            let label = self.expect_ident()?;
            self.expect_punct(";")?;
            return Ok(StmtParse::done(Stmt::On {
                condition,
                refer,
                action: OnAction::Goto { label },
            }));
        }
        let mut sp = self.parse_statement()?;
        sp.stmt = Stmt::On {
            condition,
            refer,
            action: OnAction::Stmt(Box::new(sp.stmt)),
        };
        Ok(sp)
    }

    fn parse_signal(&mut self) -> Result<StmtParse> {
        self.expect_kw("SIGNAL")?;
        let condition = self.expect_ident()?;
        let refer = if self.at_punct("(") {
            self.bump();
            let name = self.expect_ident()?;
            self.expect_punct(")")?;
            Some(name)
        } else {
            None
        };
        self.expect_punct(";")?;
        Ok(StmtParse::done(Stmt::Signal { condition, refer }))
    }

    fn parse_revert(&mut self) -> Result<StmtParse> {
        self.expect_kw("REVERT")?;
        let condition = self.expect_ident()?;
        let refer = if self.at_punct("(") {
            self.bump();
            let name = self.expect_ident()?;
            self.expect_punct(")")?;
            Some(name)
        } else {
            None
        };
        self.expect_punct(";")?;
        Ok(StmtParse::done(Stmt::Revert { condition, refer }))
    }

    fn parse_open(&mut self) -> Result<StmtParse> {
        self.expect_kw("OPEN")?;
        self.expect_kw("FILE")?;
        self.expect_punct("(")?;
        let file = self.expect_ident()?;
        self.expect_punct(")")?;
        let mut options = Vec::new();
        let mut title = None;
        while !self.at_punct(";") && !self.is_eof() {
            if self.at_kw("TITLE") {
                self.bump();
                self.expect_punct("(")?;
                let path = match &self.peek().kind {
                    TokenKind::StringLit(s) => {
                        let s = s.clone();
                        self.bump();
                        s
                    }
                    _ => {
                        return Err(self.err(
                            "TITLE expects a string literal, e.g. TITLE('file.dat')",
                        ));
                    }
                };
                self.expect_punct(")")?;
                title = Some(path);
                continue;
            }
            if self.at_kw("ENV") || self.at_kw("ENVIRONMENT") {
                self.bump();
                if self.at_punct("(") {
                    self.skip_balanced_parens()?;
                }
                continue;
            }
            options.push(self.expect_ident()?);
        }
        self.expect_punct(";")?;
        Ok(StmtParse::done(Stmt::Open {
            file,
            options,
            title,
        }))
    }

    fn skip_balanced_parens(&mut self) -> Result<()> {
        self.expect_punct("(")?;
        let mut depth = 1i32;
        while depth > 0 && !self.is_eof() {
            match &self.peek().kind {
                TokenKind::Punct("(") => {
                    depth += 1;
                    self.bump();
                }
                TokenKind::Punct(")") => {
                    depth -= 1;
                    self.bump();
                }
                TokenKind::Eof => break,
                _ => self.bump(),
            }
        }
        if depth != 0 {
            return Err(self.err("unclosed '(' in ENV/ENVIRONMENT option"));
        }
        Ok(())
    }

    fn parse_close(&mut self) -> Result<StmtParse> {
        self.expect_kw("CLOSE")?;
        self.expect_kw("FILE")?;
        self.expect_punct("(")?;
        let file = self.expect_ident()?;
        self.expect_punct(")")?;
        self.expect_punct(";")?;
        Ok(StmtParse::done(Stmt::Close { file }))
    }

    fn parse_allocate(&mut self) -> Result<StmtParse> {
        self.expect_kw("ALLOCATE")?;
        let name = self.expect_ident()?;
        let set = if self.at_kw("SET") {
            self.bump();
            self.expect_punct("(")?;
            let p = self.expect_ident()?;
            self.expect_punct(")")?;
            Some(p)
        } else {
            None
        };
        self.expect_punct(";")?;
        Ok(StmtParse::done(Stmt::Allocate { name, set }))
    }

    fn parse_free(&mut self) -> Result<StmtParse> {
        self.expect_kw("FREE")?;
        let name = self.expect_ident()?;
        self.expect_punct(";")?;
        Ok(StmtParse::done(Stmt::Free { name }))
    }

    fn parse_end(&mut self) -> Result<EndKind> {
        self.expect_kw("END")?;
        if matches!(self.peek().kind, TokenKind::Ident(_)) {
            let name = self.expect_ident()?;
            self.expect_punct(";")?;
            Ok(EndKind::Named(name))
        } else {
            self.expect_punct(";")?;
            Ok(EndKind::Bare)
        }
    }

    fn resolve_end(&self, labels: &[String], end: EndKind) -> Option<String> {
        match end {
            EndKind::Bare => None,
            EndKind::Named(n) => {
                if labels
                    .iter()
                    .any(|l| eq_ignore_ascii_case(l, &n))
                {
                    None
                } else {
                    Some(n)
                }
            }
        }
    }

    fn parse_begin(&mut self, labels: &[String]) -> Result<StmtParse> {
        self.expect_kw("BEGIN")?;
        self.expect_punct(";")?;
        let mut decls = Vec::new();
        let mut stmts = Vec::new();
        let pending_end = loop {
            if self.at_punct("%") {
                return Err(self.reject_preprocessor());
            }
            if self.at_kw("END") {
                let end = self.parse_end()?;
                break self.resolve_end(labels, end);
            }
            if self.is_eof() {
                return Err(self.err("unterminated BEGIN (missing END)"));
            }
            if self.at_kw("DCL") || self.at_kw("DECLARE") {
                decls.extend(self.parse_declare()?);
                continue;
            }
            let sp = self.parse_statement()?;
            stmts.push(sp.stmt);
            if let Some(n) = sp.pending_end {
                break if labels.iter().any(|l| eq_ignore_ascii_case(l, &n)) {
                    None
                } else {
                    Some(n)
                };
            }
        };
        Ok(StmtParse {
            stmt: Stmt::Begin {
                body: Block { decls, stmts },
            },
            pending_end,
        })
    }

    fn parse_select(&mut self, labels: &[String]) -> Result<StmtParse> {
        self.expect_kw("SELECT")?;
        let expr = if self.at_punct("(") {
            self.bump();
            let e = self.parse_expr()?;
            self.expect_punct(")")?;
            Some(e)
        } else {
            None
        };
        self.expect_punct(";")?;

        let mut whens = Vec::new();
        let mut otherwise = None;
        let pending_end = loop {
            if self.at_kw("END") {
                let end = self.parse_end()?;
                break self.resolve_end(labels, end);
            }
            if self.is_eof() {
                return Err(self.err("unterminated SELECT (missing END)"));
            }
            if self.at_kw("WHEN") {
                self.bump();
                if self.at_kw("ANY") || self.at_kw("ALL") {
                    self.bump();
                }
                self.expect_punct("(")?;
                let mut exprs = Vec::new();
                loop {
                    exprs.push(self.parse_expr()?);
                    if self.at_punct(",") {
                        self.bump();
                        continue;
                    }
                    break;
                }
                self.expect_punct(")")?;
                let sp = self.parse_statement()?;
                whens.push(WhenClause {
                    exprs,
                    action: Box::new(sp.stmt),
                });
                if let Some(n) = sp.pending_end {
                    break if labels.iter().any(|l| eq_ignore_ascii_case(l, &n)) {
                        None
                    } else {
                        Some(n)
                    };
                }
                continue;
            }
            if self.at_kw("OTHERWISE") || self.at_kw("OTHER") {
                if otherwise.is_some() {
                    return Err(self.err("duplicate OTHERWISE in SELECT"));
                }
                self.bump();
                let sp = self.parse_statement()?;
                otherwise = Some(Box::new(sp.stmt));
                if let Some(n) = sp.pending_end {
                    break if labels.iter().any(|l| eq_ignore_ascii_case(l, &n)) {
                        None
                    } else {
                        Some(n)
                    };
                }
                continue;
            }
            return Err(self.err(
                "expected WHEN, OTHERWISE, or END in SELECT",
            ));
        };
        Ok(StmtParse {
            stmt: Stmt::Select {
                expr,
                whens,
                otherwise,
            },
            pending_end,
        })
    }

    fn parse_if(&mut self) -> Result<StmtParse> {
        self.expect_kw("IF")?;
        let cond = self.parse_expr()?;
        self.expect_kw("THEN")?;
        let then_sp = self.parse_statement()?;
        if then_sp.pending_end.is_some() {
            return Ok(StmtParse {
                stmt: Stmt::If {
                    cond,
                    then_branch: Box::new(then_sp.stmt),
                    else_branch: None,
                },
                pending_end: then_sp.pending_end,
            });
        }
        let else_branch = if self.at_kw("ELSE") {
            self.bump();
            let else_sp = self.parse_statement()?;
            Some((Box::new(else_sp.stmt), else_sp.pending_end))
        } else {
            None
        };
        let (else_branch, pending_end) = match else_branch {
            Some((s, p)) => (Some(s), p),
            None => (None, None),
        };
        Ok(StmtParse {
            stmt: Stmt::If {
                cond,
                then_branch: Box::new(then_sp.stmt),
                else_branch,
            },
            pending_end,
        })
    }

    fn parse_call(&mut self) -> Result<StmtParse> {
        self.expect_kw("CALL")?;
        let name = self.expect_ident()?;
        let mut args = Vec::new();
        if self.at_punct("(") {
            self.bump();
            if !self.at_punct(")") {
                loop {
                    args.push(self.parse_expr()?);
                    if self.at_punct(",") {
                        self.bump();
                        continue;
                    }
                    break;
                }
            }
            self.expect_punct(")")?;
        }
        self.expect_punct(";")?;
        Ok(StmtParse::done(Stmt::Call { name, args }))
    }

    fn parse_do(&mut self, labels: &[String]) -> Result<StmtParse> {
        self.expect_kw("DO")?;
        // Group: DO; … END;
        if self.at_punct(";") {
            self.bump();
            let (body, pending_end) = self.parse_do_body(labels)?;
            return Ok(StmtParse {
                stmt: Stmt::DoGroup { body },
                pending_end,
            });
        }
        // DO WHILE (expr); … END;
        if self.at_kw("WHILE") {
            self.bump();
            self.expect_punct("(")?;
            let cond = self.parse_expr()?;
            self.expect_punct(")")?;
            self.expect_punct(";")?;
            let (body, pending_end) = self.parse_do_body(labels)?;
            return Ok(StmtParse {
                stmt: Stmt::DoWhile { cond, body },
                pending_end,
            });
        }
        // DO UNTIL (expr); … END;
        if self.at_kw("UNTIL") {
            self.bump();
            self.expect_punct("(")?;
            let cond = self.parse_expr()?;
            self.expect_punct(")")?;
            self.expect_punct(";")?;
            let (body, pending_end) = self.parse_do_body(labels)?;
            return Ok(StmtParse {
                stmt: Stmt::DoUntil { cond, body },
                pending_end,
            });
        }
        // Counted: DO var = from TO to [BY step] ;
        if !(matches!(self.peek().kind, TokenKind::Ident(_)) && self.lookahead_is_assign()) {
            return Err(self.err("expected DO;, DO WHILE, DO UNTIL, or DO var = … TO …;"));
        }
        let var = self.expect_ident()?;
        self.expect_punct("=")?;
        let from = self.parse_expr()?;
        self.expect_kw("TO")?;
        let to = self.parse_expr()?;
        let step = if self.at_kw("BY") {
            self.bump();
            Some(self.parse_expr()?)
        } else {
            None
        };
        self.expect_punct(";")?;
        let (body, pending_end) = self.parse_do_body(labels)?;
        Ok(StmtParse {
            stmt: Stmt::DoCounted {
                var,
                from,
                to,
                step,
                body,
            },
            pending_end,
        })
    }

    fn parse_do_body(&mut self, labels: &[String]) -> Result<(Vec<Stmt>, Option<String>)> {
        let mut body = Vec::new();
        loop {
            if self.at_kw("END") {
                let end = self.parse_end()?;
                let pending = self.resolve_end(labels, end);
                return Ok((body, pending));
            }
            if self.is_eof() {
                return Err(self.err("unterminated DO (missing END)"));
            }
            let sp = self.parse_statement()?;
            body.push(sp.stmt);
            if let Some(n) = sp.pending_end {
                let pending = if labels.iter().any(|l| eq_ignore_ascii_case(l, &n)) {
                    None
                } else {
                    Some(n)
                };
                return Ok((body, pending));
            }
        }
    }

    fn lookahead_is_assign(&self) -> bool {
        matches!(
            self.tokens.get(self.pos + 1).map(|t| &t.kind),
            Some(TokenKind::Punct("="))
        )
    }

    /// `Ident . Ident [= . Ident…] =`
    fn lookahead_is_assign_field(&self) -> bool {
        let mut i = self.pos;
        if !matches!(self.tokens.get(i).map(|t| &t.kind), Some(TokenKind::Ident(_))) {
            return false;
        }
        i += 1;
        let mut saw_dot = false;
        loop {
            match self.tokens.get(i).map(|t| &t.kind) {
                Some(TokenKind::Punct(".")) => {
                    saw_dot = true;
                    i += 1;
                    if !matches!(self.tokens.get(i).map(|t| &t.kind), Some(TokenKind::Ident(_))) {
                        return false;
                    }
                    i += 1;
                }
                Some(TokenKind::Punct("=")) => return saw_dot,
                _ => return false,
            }
        }
    }

    fn lookahead_is_label(&self) -> bool {
        matches!(
            self.tokens.get(self.pos + 1).map(|t| &t.kind),
            Some(TokenKind::Punct(":"))
        )
    }

    fn lookahead_is_lparen(&self) -> bool {
        matches!(
            self.tokens.get(self.pos + 1).map(|t| &t.kind),
            Some(TokenKind::Punct("("))
        )
    }

    /// `Ident ( … ) =` with balanced parentheses (array element assign).
    fn lookahead_is_assign_index(&self) -> bool {
        if !self.lookahead_is_lparen() {
            return false;
        }
        let mut i = self.pos + 1;
        let mut depth = 0i32;
        while let Some(t) = self.tokens.get(i) {
            match &t.kind {
                TokenKind::Punct("(") => depth += 1,
                TokenKind::Punct(")") => {
                    depth -= 1;
                    if depth == 0 {
                        return matches!(
                            self.tokens.get(i + 1).map(|t| &t.kind),
                            Some(TokenKind::Punct("="))
                        );
                    }
                }
                TokenKind::Eof => return false,
                _ => {}
            }
            i += 1;
        }
        false
    }

    /// Parse optional `FILE(name)` and `SKIP` in either order before EDIT/DATA/LIST.
    fn parse_io_file_skip(&mut self) -> Result<(Option<String>, bool)> {
        let mut file = None;
        let mut skip = false;
        loop {
            if file.is_none() && self.at_kw("FILE") {
                self.bump();
                self.expect_punct("(")?;
                file = Some(self.expect_ident()?);
                self.expect_punct(")")?;
                continue;
            }
            if !skip && self.at_kw("SKIP") {
                self.bump();
                skip = true;
                continue;
            }
            break;
        }
        Ok((file, skip))
    }

    fn parse_put(&mut self) -> Result<StmtParse> {
        self.expect_kw("PUT")?;
        let (file, skip) = self.parse_io_file_skip()?;
        if self.at_kw("EDIT") {
            self.bump();
            let items = self.parse_expr_list()?;
            let formats = self.parse_format_list()?;
            self.expect_punct(";")?;
            return Ok(StmtParse::done(Stmt::PutEdit {
                file,
                skip,
                items,
                formats,
            }));
        }
        if self.at_kw("DATA") {
            self.bump();
            let items = if self.at_punct("(") {
                Some(self.parse_name_list()?)
            } else {
                None
            };
            self.expect_punct(";")?;
            return Ok(StmtParse::done(Stmt::PutData { file, skip, items }));
        }
        self.expect_kw("LIST")?;
        let items = self.parse_expr_list()?;
        self.expect_punct(";")?;
        Ok(StmtParse::done(Stmt::PutList { file, skip, items }))
    }

    fn parse_get(&mut self) -> Result<StmtParse> {
        self.expect_kw("GET")?;
        let (file, skip) = self.parse_io_file_skip()?;
        if self.at_kw("EDIT") {
            self.bump();
            let items = self.parse_name_list()?;
            let formats = self.parse_format_list()?;
            self.expect_punct(";")?;
            return Ok(StmtParse::done(Stmt::GetEdit {
                file,
                skip,
                items,
                formats,
            }));
        }
        if self.at_kw("DATA") {
            self.bump();
            let items = if self.at_punct("(") {
                Some(self.parse_name_list()?)
            } else {
                None
            };
            self.expect_punct(";")?;
            return Ok(StmtParse::done(Stmt::GetData { file, skip, items }));
        }
        self.expect_kw("LIST")?;
        let items = self.parse_name_list()?;
        self.expect_punct(";")?;
        Ok(StmtParse::done(Stmt::GetList { file, skip, items }))
    }

    fn parse_expr_list(&mut self) -> Result<Vec<Expr>> {
        self.expect_punct("(")?;
        let mut items = Vec::new();
        loop {
            items.push(self.parse_expr()?);
            if self.at_punct(",") {
                self.bump();
                continue;
            }
            break;
        }
        self.expect_punct(")")?;
        Ok(items)
    }

    fn parse_name_list(&mut self) -> Result<Vec<String>> {
        self.expect_punct("(")?;
        let mut items = Vec::new();
        loop {
            items.push(self.expect_ident()?);
            if self.at_punct(",") {
                self.bump();
                continue;
            }
            break;
        }
        self.expect_punct(")")?;
        Ok(items)
    }

    fn parse_format_list(&mut self) -> Result<Vec<FormatItem>> {
        self.expect_punct("(")?;
        let mut formats = Vec::new();
        loop {
            let name = self.expect_ident()?;
            // Picture: P'$$$9.99' (string immediately after P)
            if eq_ignore_ascii_case(&name, "P") {
                if let TokenKind::StringLit(s) = &self.peek().kind {
                    let s = s.clone();
                    self.bump();
                    formats.push(FormatItem::Call {
                        name,
                        args: vec![Expr::StringLit(s)],
                    });
                    if self.at_punct(",") {
                        self.bump();
                        continue;
                    }
                    break;
                }
            }
            if self.at_punct("(") {
                self.bump();
                let mut args = Vec::new();
                if !self.at_punct(")") {
                    loop {
                        args.push(self.parse_expr()?);
                        if self.at_punct(",") {
                            self.bump();
                            continue;
                        }
                        break;
                    }
                }
                self.expect_punct(")")?;
                formats.push(FormatItem::Call { name, args });
            } else {
                formats.push(FormatItem::Name(name));
            }
            if self.at_punct(",") {
                self.bump();
                continue;
            }
            break;
        }
        self.expect_punct(")")?;
        Ok(formats)
    }

    fn parse_expr(&mut self) -> Result<Expr> {
        self.parse_bitor()
    }

    fn parse_bitor(&mut self) -> Result<Expr> {
        let mut left = self.parse_bitand()?;
        while self.at_punct("|") {
            self.bump();
            let right = self.parse_bitand()?;
            left = Expr::Binary {
                op: BinOp::BitOr,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_bitand(&mut self) -> Result<Expr> {
        let mut left = self.parse_comparison()?;
        while self.at_punct("&") {
            self.bump();
            let right = self.parse_comparison()?;
            left = Expr::Binary {
                op: BinOp::BitAnd,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_comparison(&mut self) -> Result<Expr> {
        let mut left = self.parse_additive()?;
        let op = if self.at_punct("=") {
            BinOp::Eq
        } else if self.at_punct("<>") {
            BinOp::Ne
        } else if self.at_punct("<=") {
            BinOp::Le
        } else if self.at_punct(">=") {
            BinOp::Ge
        } else if self.at_punct("<") {
            BinOp::Lt
        } else if self.at_punct(">") {
            BinOp::Gt
        } else {
            return Ok(left);
        };
        self.bump();
        let right = self.parse_additive()?;
        left = Expr::Binary {
            op,
            left: Box::new(left),
            right: Box::new(right),
        };
        Ok(left)
    }

    fn parse_additive(&mut self) -> Result<Expr> {
        let mut left = self.parse_term()?;
        loop {
            let op = if self.at_punct("+") {
                BinOp::Add
            } else if self.at_punct("-") {
                BinOp::Sub
            } else {
                break;
            };
            self.bump();
            let right = self.parse_term()?;
            left = Expr::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Expr> {
        if self.at_punct("+") {
            self.bump();
            return self.parse_unary();
        }
        if self.at_punct("-") {
            self.bump();
            let e = self.parse_unary()?;
            return Ok(match e {
                Expr::IntLit(n) => Expr::IntLit(-n),
                Expr::FloatLit(n) => Expr::FloatLit(-n),
                other => Expr::Binary {
                    op: BinOp::Sub,
                    left: Box::new(Expr::IntLit(0)),
                    right: Box::new(other),
                },
            });
        }
        self.parse_primary()
    }

    fn parse_term(&mut self) -> Result<Expr> {
        let mut left = self.parse_unary()?;
        loop {
            let op = if self.at_punct("*") {
                BinOp::Mul
            } else if self.at_punct("/") {
                BinOp::Div
            } else {
                break;
            };
            self.bump();
            let right = self.parse_primary()?;
            left = Expr::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_primary(&mut self) -> Result<Expr> {
        match &self.peek().kind {
            TokenKind::StringLit(s) => {
                let s = s.clone();
                self.bump();
                Ok(Expr::StringLit(s))
            }
            TokenKind::BitLit { value, width } => {
                let value = *value;
                let width = *width;
                self.bump();
                Ok(Expr::BitLit { value, width })
            }
            TokenKind::IntLit(n) => {
                let n = *n;
                self.bump();
                Ok(Expr::IntLit(n))
            }
            TokenKind::FloatLit(n) => {
                let n = *n;
                self.bump();
                Ok(Expr::FloatLit(n))
            }
            TokenKind::Ident(id) => {
                let id = id.clone();
                self.bump();
                if eq_ignore_ascii_case(&id, "NULL") {
                    if self.at_punct("(") {
                        self.bump();
                        self.expect_punct(")")?;
                    }
                    return Ok(Expr::Null);
                }
                if eq_ignore_ascii_case(&id, "ADDR") && self.at_punct("(") {
                    self.bump();
                    let name = self.expect_ident()?;
                    self.expect_punct(")")?;
                    return Ok(Expr::Addr { name });
                }
                if self.at_punct(".") {
                    let mut path = vec![id];
                    while self.at_punct(".") {
                        self.bump();
                        path.push(self.expect_ident()?);
                    }
                    return Ok(Expr::Field { path });
                }
                if self.at_punct("(") {
                    let indices = self.parse_expr_list()?;
                    return Ok(Expr::Index {
                        base: id,
                        indices,
                    });
                }
                Ok(Expr::Name(id))
            }
            TokenKind::Punct("(") => {
                self.bump();
                let e = self.parse_expr()?;
                self.expect_punct(")")?;
                Ok(e)
            }
            _ => Err(self.err("expected expression")),
        }
    }

    fn expect_kw(&mut self, kw: &str) -> Result<()> {
        match &self.peek().kind {
            TokenKind::Ident(id) if eq_ignore_ascii_case(id, kw) => {
                self.bump();
                Ok(())
            }
            _ => Err(self.err(format!("expected keyword `{kw}`"))),
        }
    }

    fn expect_ident(&mut self) -> Result<String> {
        match &self.peek().kind {
            TokenKind::Ident(id) => {
                let id = id.clone();
                self.bump();
                Ok(id)
            }
            _ => Err(self.err("expected identifier")),
        }
    }

    fn expect_punct(&mut self, p: &str) -> Result<()> {
        match &self.peek().kind {
            TokenKind::Punct(q) if *q == p => {
                self.bump();
                Ok(())
            }
            _ => Err(self.err(format!("expected `{p}`"))),
        }
    }

    fn at_kw(&self, kw: &str) -> bool {
        matches!(&self.peek().kind, TokenKind::Ident(id) if eq_ignore_ascii_case(id, kw))
    }

    fn at_punct(&self, p: &str) -> bool {
        matches!(&self.peek().kind, TokenKind::Punct(q) if *q == p)
    }

    fn is_eof(&self) -> bool {
        matches!(self.peek().kind, TokenKind::Eof)
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.pos.min(self.tokens.len().saturating_sub(1))]
    }

    fn bump(&mut self) {
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
    }

    fn err(&self, message: impl Into<String>) -> Diagnostic {
        let t = self.peek();
        Diagnostic::error(message).at(t.line, t.column)
    }
}
