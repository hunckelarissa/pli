//! Semantic checks before codegen (L0 slice).

use crate::diagnostic::{Diagnostic, Result};
use crate::hir::{ArrayBound, Block, Decl, Expr, Module, OnAction, Stmt, Type};
use crate::lex::eq_ignore_ascii_case;
use std::collections::{HashMap, HashSet};

pub fn check(module: &Module) -> Result<()> {
    let mains: Vec<_> = module.procs.iter().filter(|p| p.is_main).collect();
    if mains.len() != 1 {
        return Err(Diagnostic::error(format!(
            "expected exactly one OPTIONS(MAIN), found {}",
            mains.len()
        )));
    }

    let mut proc_arity: HashMap<String, usize> = HashMap::new();
    for proc in &module.procs {
        let key = proc.name.to_ascii_uppercase();
        if proc_arity.insert(key, proc.params.len()).is_some() {
            return Err(Diagnostic::error(format!(
                "duplicate procedure `{}`",
                proc.name
            )));
        }
    }

    for proc in &module.procs {
        let mut types: HashMap<String, Type> = HashMap::new();
        let mut dims: HashMap<String, Vec<ArrayBound>> = HashMap::new();
        for p in &proc.params {
            types.insert(p.to_ascii_uppercase(), Type::FixedBin { precision: 31 });
        }
        for d in &proc.body.decls {
            let key = d.name.to_ascii_uppercase();
            let is_param = proc
                .params
                .iter()
                .any(|p| eq_ignore_ascii_case(p, &d.name));
            if types.contains_key(&key) {
                if !is_param {
                    return Err(Diagnostic::error(format!(
                        "duplicate declaration of `{}` in procedure `{}`",
                        d.name, proc.name
                    )));
                }
            }
            types.insert(key.clone(), d.ty.clone());
            if let Some(dd) = &d.dims {
                dims.insert(key, dd.clone());
            }
        }
        for p in &proc.params {
            if !proc
                .body
                .decls
                .iter()
                .any(|d| eq_ignore_ascii_case(&d.name, p))
            {
                return Err(Diagnostic::error(format!(
                    "parameter `{p}` in `{}` lacks DECLARE",
                    proc.name
                )));
            }
        }
        for d in &proc.body.decls {
            check_decl_init(d, &types, &proc.name)?;
        }
        check_like_decls(&proc.body.decls, &types, &proc.name)?;
        check_defined_decls(&proc.body.decls, &types, &proc.name)?;
        let mut labels = HashSet::new();
        for stmt in &proc.body.stmts {
            collect_labels(stmt, &mut labels, &proc.name)?;
        }
        for stmt in &proc.body.stmts {
            check_stmt(
                stmt,
                &types,
                &dims,
                &proc.body.decls,
                &proc.name,
                &proc_arity,
                false,
                &labels,
            )?;
        }
    }
    Ok(())
}

fn collect_labels(stmt: &Stmt, labels: &mut HashSet<String>, proc: &str) -> Result<()> {
    match stmt {
        Stmt::Labeled { labels: labs, body } => {
            for lab in labs {
                let key = lab.to_ascii_uppercase();
                if !labels.insert(key) {
                    return Err(Diagnostic::error(format!(
                        "duplicate label `{lab}` in procedure `{proc}`"
                    )));
                }
            }
            collect_labels(body, labels, proc)
        }
        Stmt::If {
            then_branch,
            else_branch,
            ..
        } => {
            collect_labels(then_branch, labels, proc)?;
            if let Some(e) = else_branch {
                collect_labels(e, labels, proc)?;
            }
            Ok(())
        }
        Stmt::DoCounted { body, .. }
        | Stmt::DoGroup { body }
        | Stmt::DoWhile { body, .. }
        | Stmt::DoUntil { body, .. } => {
            for s in body {
                collect_labels(s, labels, proc)?;
            }
            Ok(())
        }
        Stmt::Begin { body } => {
            for s in &body.stmts {
                collect_labels(s, labels, proc)?;
            }
            Ok(())
        }
        Stmt::Select {
            whens, otherwise, ..
        } => {
            for w in whens {
                collect_labels(&w.action, labels, proc)?;
            }
            if let Some(o) = otherwise {
                collect_labels(o, labels, proc)?;
            }
            Ok(())
        }
        Stmt::On { action, .. } => match action {
            OnAction::Stmt(s) => collect_labels(s, labels, proc),
            _ => Ok(()),
        },
        _ => Ok(()),
    }
}

fn check_decl_init(
    d: &crate::hir::Decl,
    types: &HashMap<String, Type>,
    proc: &str,
) -> Result<()> {
    let Some(init) = &d.init else {
        return Ok(());
    };
    if matches!(d.storage, crate::hir::Storage::Parameter) {
        return Err(Diagnostic::error(format!(
            "INIT not allowed on parameter `{}`",
            d.name
        )));
    }
    if init.is_empty() {
        return Err(Diagnostic::error(format!(
            "empty INIT for `{}`",
            d.name
        )));
    }
    let expected = if let Some(dims) = &d.dims {
        dims.iter()
            .map(|b| match b {
                ArrayBound::Upper(n) => *n,
                ArrayBound::Range { from, to } => to - from + 1,
            })
            .product::<i64>() as usize
    } else {
        1
    };
    if init.len() > expected {
        return Err(Diagnostic::error(format!(
            "INIT for `{}` has {} values, array extent is {expected}",
            d.name,
            init.len()
        )));
    }
    if d.dims.is_none() && init.len() != 1 {
        return Err(Diagnostic::error(format!(
            "scalar INIT for `{}` expects one value, got {}",
            d.name,
            init.len()
        )));
    }
    for e in init {
        match e {
            Expr::IntLit(_) | Expr::FloatLit(_) | Expr::StringLit(_) | Expr::BitLit { .. } => {}
            Expr::Null if matches!(d.ty, Type::Pointer) => {}
            _ => {
                return Err(Diagnostic::error(format!(
                    "L0 INIT for `{}` must be a literal",
                    d.name
                )));
            }
        }
        check_expr(e, types, &HashMap::new(), proc)?;
        check_assignable(&d.ty, e, types)?;
    }
    Ok(())
}

fn check_stmt(
    stmt: &Stmt,
    types: &HashMap<String, Type>,
    dims: &HashMap<String, Vec<ArrayBound>>,
    decls: &[Decl],
    proc: &str,
    arity: &HashMap<String, usize>,
    in_do: bool,
    labels: &HashSet<String>,
) -> Result<()> {
    match stmt {
        Stmt::Labeled { body, .. } => {
            return check_stmt(body, types, dims, decls, proc, arity, in_do, labels);
        }
        Stmt::Assign { name, value } => {
            let key = name.to_ascii_uppercase();
            if let Some(dd) = dims.get(&key) {
                // Aggregate array assign: `A = B;`
                let Expr::Name(src) = value else {
                    return Err(Diagnostic::error(format!(
                        "array `{name}` requires subscripts on assignment"
                    )));
                };
                let sk = src.to_ascii_uppercase();
                let Some(sd) = dims.get(&sk) else {
                    return Err(Diagnostic::error(format!(
                        "aggregate assign to `{name}` requires array RHS, got `{src}`"
                    )));
                };
                let llen: i64 = dd
                    .iter()
                    .map(|b| match b {
                        ArrayBound::Upper(n) => *n,
                        ArrayBound::Range { from, to } => to - from + 1,
                    })
                    .product();
                let rlen: i64 = sd
                    .iter()
                    .map(|b| match b {
                        ArrayBound::Upper(n) => *n,
                        ArrayBound::Range { from, to } => to - from + 1,
                    })
                    .product();
                if llen != rlen {
                    return Err(Diagnostic::error(format!(
                        "aggregate assign `{name} = {src}` size mismatch ({llen} vs {rlen})"
                    )));
                }
                let Some(lhs_ty) = types.get(&key) else {
                    return Err(Diagnostic::error(format!(
                        "undeclared name `{name}` in procedure `{proc}`"
                    )));
                };
                let Some(rhs_ty) = types.get(&sk) else {
                    return Err(Diagnostic::error(format!(
                        "undeclared name `{src}` in procedure `{proc}`"
                    )));
                };
                if lhs_ty != rhs_ty {
                    return Err(Diagnostic::error(format!(
                        "aggregate assign `{name} = {src}` type mismatch"
                    )));
                }
                return Ok(());
            }
            let Some(lhs_ty) = types.get(&key) else {
                return Err(Diagnostic::error(format!(
                    "undeclared name `{name}` in procedure `{proc}`"
                )));
            };
            if matches!(lhs_ty, Type::Struct) {
                let Expr::Name(src) = value else {
                    return Err(Diagnostic::error(format!(
                        "STRUCTURE `{name}` requires STRUCTURE RHS on assignment"
                    )));
                };
                let sk = src.to_ascii_uppercase();
                match types.get(&sk) {
                    Some(Type::Struct) => {}
                    Some(_) => {
                        return Err(Diagnostic::error(format!(
                            "aggregate assign to STRUCTURE `{name}` requires STRUCTURE `{src}`"
                        )));
                    }
                    None => {
                        return Err(Diagnostic::error(format!(
                            "undeclared name `{src}` in procedure `{proc}`"
                        )));
                    }
                }
                if !struct_layout_eq(decls, name, src) {
                    return Err(Diagnostic::error(format!(
                        "STRUCTURE assign `{name} = {src}` layout mismatch"
                    )));
                }
                return Ok(());
            }
            check_expr(value, types, dims, proc)?;
            check_assignable(lhs_ty, value, types)?;
        }
        Stmt::AssignIndex {
            name,
            indices,
            value,
        } => {
            check_index_use(name, indices, types, dims, proc)?;
            let lhs_ty = types.get(&name.to_ascii_uppercase()).unwrap();
            check_expr(value, types, dims, proc)?;
            check_assignable(lhs_ty, value, types)?;
        }
        Stmt::AssignField { path, value } => {
            let lhs_ty = field_type(path, types, proc)?;
            check_expr(value, types, dims, proc)?;
            check_assignable(lhs_ty, value, types)?;
        }
        Stmt::PutList { items, .. } => {
            for e in items {
                check_expr(e, types, dims, proc)?;
            }
        }
        Stmt::If {
            cond,
            then_branch,
            else_branch,
        } => {
            check_expr(cond, types, dims, proc)?;
            check_stmt(then_branch, types, dims, decls, proc, arity, in_do, labels)?;
            if let Some(e) = else_branch {
                check_stmt(e, types, dims, decls, proc, arity, in_do, labels)?;
            }
        }
        Stmt::DoCounted {
            var,
            from,
            to,
            step,
            body,
        } => {
            require_fixed(var, types, dims, proc)?;
            check_expr(from, types, dims, proc)?;
            check_expr(to, types, dims, proc)?;
            if let Some(s) = step {
                check_expr(s, types, dims, proc)?;
            }
            for s in body {
                check_stmt(s, types, dims, decls, proc, arity, true, labels)?;
            }
        }
        Stmt::DoGroup { body } => {
            for s in body {
                check_stmt(s, types, dims, decls, proc, arity, true, labels)?;
            }
        }
        Stmt::DoWhile { cond, body } | Stmt::DoUntil { cond, body } => {
            check_expr(cond, types, dims, proc)?;
            for s in body {
                check_stmt(s, types, dims, decls, proc, arity, true, labels)?;
            }
        }
        Stmt::Leave | Stmt::Iterate => {
            if !in_do {
                return Err(Diagnostic::error(
                    "LEAVE/ITERATE is only valid inside a DO group or counted DO",
                ));
            }
        }
        Stmt::Return | Stmt::Stop | Stmt::Null => {}
        Stmt::GetList { items, .. } => {
            for name in items {
                require_name(name, types, proc)?;
            }
        }
        Stmt::PutEdit { items, formats, .. } => {
            for e in items {
                check_expr(e, types, dims, proc)?;
            }
            for f in formats {
                check_format(f, types, dims, proc)?;
            }
        }
        Stmt::GetEdit { items, formats, .. } => {
            for name in items {
                require_name(name, types, proc)?;
            }
            for f in formats {
                check_format(f, types, dims, proc)?;
            }
        }
        Stmt::PutData { items, .. } | Stmt::GetData { items, .. } => {
            if let Some(names) = items {
                for name in names {
                    require_name(name, types, proc)?;
                }
            }
        }
        Stmt::Goto { label } => {
            if !labels.contains(&label.to_ascii_uppercase()) {
                return Err(Diagnostic::error(format!(
                    "GOTO to unknown label `{label}` in procedure `{proc}`"
                )));
            }
        }
        Stmt::Call { name, args } => {
            let key = name.to_ascii_uppercase();
            let Some(n) = arity.get(&key) else {
                return Err(Diagnostic::error(format!(
                    "CALL to unknown procedure `{name}`"
                )));
            };
            if *n != args.len() {
                return Err(Diagnostic::error(format!(
                    "CALL `{name}` expects {n} args, got {}",
                    args.len()
                )));
            }
            for a in args {
                match a {
                    Expr::Name(n) => {
                        require_name(n, types, proc)?;
                    }
                    _ => {
                        return Err(Diagnostic::error(
                            "L0 CALL arguments must be variable names (by-reference)",
                        ));
                    }
                }
            }
        }
        Stmt::Begin { body } => check_block(body, types, dims, decls, proc, arity, in_do, labels)?,
        Stmt::Select {
            expr,
            whens,
            otherwise,
        } => {
            if let Some(e) = expr {
                check_expr(e, types, dims, proc)?;
            }
            for w in whens {
                for e in &w.exprs {
                    check_expr(e, types, dims, proc)?;
                }
                check_stmt(&w.action, types, dims, decls, proc, arity, in_do, labels)?;
            }
            if let Some(o) = otherwise {
                check_stmt(o, types, dims, decls, proc, arity, in_do, labels)?;
            }
        }
        Stmt::On { action, .. } => match action {
            OnAction::Null | OnAction::Snap { .. } | OnAction::System => {}
            OnAction::Goto { label } => {
                if !labels.contains(&label.to_ascii_uppercase()) {
                    return Err(Diagnostic::error(format!(
                        "ON … GOTO to unknown label `{label}` in procedure `{proc}`"
                    )));
                }
            }
            OnAction::Stmt(s) => {
                check_stmt(s, types, dims, decls, proc, arity, in_do, labels)?;
            }
        },
        Stmt::Open { .. }
        | Stmt::Close { .. }
        | Stmt::Signal { .. }
        | Stmt::Revert { .. } => {}
        Stmt::Allocate { name, set: _ } => {
            let key = name.to_ascii_uppercase();
            let Some(d) = decls.iter().find(|d| d.name.eq_ignore_ascii_case(name)) else {
                return Err(Diagnostic::error(format!(
                    "ALLOCATE of undeclared `{name}` in procedure `{proc}`"
                )));
            };
            if d.based_on.is_none() {
                return Err(Diagnostic::error(format!(
                    "ALLOCATE `{name}` requires BASED(P) on L0"
                )));
            }
            match types.get(&key) {
                Some(
                    Type::FixedBin { .. }
                    | Type::Float
                    | Type::Char { .. }
                    | Type::Bit { .. }
                    | Type::Struct,
                ) => {}
                _ => {
                    return Err(Diagnostic::error(format!(
                        "ALLOCATE `{name}` type not implemented on L0"
                    )));
                }
            }
        }
        Stmt::Free { name } => {
            let Some(d) = decls.iter().find(|d| d.name.eq_ignore_ascii_case(name)) else {
                return Err(Diagnostic::error(format!(
                    "FREE of undeclared `{name}` in procedure `{proc}`"
                )));
            };
            if d.based_on.is_none() {
                return Err(Diagnostic::error(format!(
                    "FREE `{name}` requires BASED(P) on L0"
                )));
            }
        }
    }
    Ok(())
}

fn check_block(
    body: &Block,
    outer: &HashMap<String, Type>,
    outer_dims: &HashMap<String, Vec<ArrayBound>>,
    decls: &[Decl],
    proc: &str,
    arity: &HashMap<String, usize>,
    in_do: bool,
    labels: &HashSet<String>,
) -> Result<()> {
    let mut types = outer.clone();
    let mut dims = outer_dims.clone();
    let mut locals = HashSet::new();
    for d in &body.decls {
        let key = d.name.to_ascii_uppercase();
        if !locals.insert(key.clone()) {
            return Err(Diagnostic::error(format!(
                "duplicate declaration of `{}` in BEGIN in procedure `{proc}`",
                d.name
            )));
        }
        types.insert(key.clone(), d.ty.clone());
        if let Some(dd) = &d.dims {
            dims.insert(key, dd.clone());
        }
    }
    for d in &body.decls {
        check_decl_init(d, &types, proc)?;
    }
    for stmt in &body.stmts {
        check_stmt(stmt, &types, &dims, decls, proc, arity, in_do, labels)?;
    }
    Ok(())
}

fn check_assignable(
    lhs: &Type,
    value: &Expr,
    types: &HashMap<String, Type>,
) -> Result<()> {
    match lhs {
        Type::FixedBin { .. } => match value {
            Expr::StringLit(_) => Err(Diagnostic::error(
                "cannot assign CHAR literal to FIXED BIN (L0)",
            )),
            Expr::Name(n) => match types.get(&n.to_ascii_uppercase()) {
                Some(Type::Char { .. }) => Err(Diagnostic::error(
                    "cannot assign CHAR to FIXED BIN (L0)",
                )),
                Some(Type::Bit { .. } | Type::Pointer) => Err(Diagnostic::error(
                    "cannot assign BIT/POINTER to FIXED BIN (L0)",
                )),
                _ => Ok(()),
            },
            Expr::Field { path } => match types.get(&path[path.len() - 1].to_ascii_uppercase()) {
                Some(Type::Char { .. } | Type::Bit { .. } | Type::Pointer | Type::Struct) => {
                    Err(Diagnostic::error(
                        "cannot assign CHAR/BIT/POINTER/STRUCTURE to FIXED BIN (L0)",
                    ))
                }
                _ => Ok(()),
            },
            Expr::Null | Expr::Addr { .. } | Expr::BitLit { .. } => Err(Diagnostic::error(
                "cannot assign NULL/ADDR/bit literal to FIXED BIN (L0)",
            )),
            Expr::IntLit(_) | Expr::FloatLit(_) | Expr::Binary { .. } | Expr::Index { .. } => {
                Ok(())
            }
        },
        Type::Float => match value {
            Expr::StringLit(_) => Err(Diagnostic::error(
                "cannot assign CHAR literal to FLOAT (L0)",
            )),
            Expr::Name(n) => match types.get(&n.to_ascii_uppercase()) {
                Some(Type::Char { .. } | Type::Bit { .. } | Type::Pointer) => Err(
                    Diagnostic::error("cannot assign CHAR/BIT/POINTER to FLOAT (L0)"),
                ),
                _ => Ok(()),
            },
            Expr::Field { path } => match types.get(&path[path.len() - 1].to_ascii_uppercase()) {
                Some(Type::Char { .. } | Type::Bit { .. } | Type::Pointer | Type::Struct) => Err(
                    Diagnostic::error("cannot assign CHAR/BIT/POINTER/STRUCTURE to FLOAT (L0)"),
                ),
                _ => Ok(()),
            },
            Expr::Null | Expr::Addr { .. } | Expr::BitLit { .. } => Err(Diagnostic::error(
                "cannot assign NULL/ADDR/bit literal to FLOAT (L0)",
            )),
            Expr::IntLit(_) | Expr::FloatLit(_) | Expr::Binary { .. } | Expr::Index { .. } => {
                Ok(())
            }
        },
        Type::Char { .. } => match value {
            Expr::StringLit(_) => Ok(()),
            Expr::Name(n) => match types.get(&n.to_ascii_uppercase()) {
                Some(Type::Char { .. }) => Ok(()),
                Some(_) => Err(Diagnostic::error(
                    "cannot assign non-CHAR to CHAR (L0)",
                )),
                None => Ok(()),
            },
            Expr::Field { path } => match types.get(&path[path.len() - 1].to_ascii_uppercase()) {
                Some(Type::Char { .. }) => Ok(()),
                _ => Err(Diagnostic::error(
                    "cannot assign non-CHAR to CHAR (L0)",
                )),
            },
            Expr::Null | Expr::Addr { .. } | Expr::BitLit { .. } => Err(Diagnostic::error(
                "cannot assign NULL/ADDR/bit literal to CHAR (L0)",
            )),
            Expr::IntLit(_) | Expr::FloatLit(_) | Expr::Binary { .. } | Expr::Index { .. } => {
                Err(Diagnostic::error(
                    "cannot assign numeric expression to CHAR (L0)",
                ))
            }
        },
        Type::Pointer => match value {
            Expr::Null => Ok(()),
            Expr::Addr { name } => {
                if types.contains_key(&name.to_ascii_uppercase()) {
                    Ok(())
                } else {
                    Err(Diagnostic::error(format!(
                        "ADDR of undeclared `{name}`"
                    )))
                }
            }
            Expr::Name(n) => match types.get(&n.to_ascii_uppercase()) {
                Some(Type::Pointer) => Ok(()),
                _ => Err(Diagnostic::error(
                    "POINTER assignment requires POINTER, NULL, or ADDR (L0)",
                )),
            },
            _ => Err(Diagnostic::error(
                "POINTER assignment requires POINTER, NULL, or ADDR (L0)",
            )),
        },
        Type::Bit { .. } => match value {
            Expr::IntLit(n) if *n == 0 || *n == 1 => Ok(()),
            Expr::BitLit { .. } => Ok(()),
            Expr::Name(n) => match types.get(&n.to_ascii_uppercase()) {
                Some(Type::Bit { .. } | Type::FixedBin { .. }) => Ok(()),
                _ => Err(Diagnostic::error(
                    "BIT assignment requires 0/1, bit literal, or BIT/FIXED (L0)",
                )),
            },
            Expr::Binary { .. } => Ok(()),
            _ => Err(Diagnostic::error(
                "BIT assignment requires 0/1, bit literal, or BIT/FIXED (L0)",
            )),
        },
        Type::Struct => Err(Diagnostic::error(
            "assignment to STRUCTURE not implemented on L0",
        )),
    }
}

fn check_format(
    fmt: &crate::hir::FormatItem,
    types: &HashMap<String, Type>,
    dims: &HashMap<String, Vec<ArrayBound>>,
    proc: &str,
) -> Result<()> {
    match fmt {
        crate::hir::FormatItem::Name(_) => Ok(()),
        crate::hir::FormatItem::Call { args, .. } => {
            for a in args {
                check_expr(a, types, dims, proc)?;
            }
            Ok(())
        }
    }
}

fn check_index_use(
    name: &str,
    indices: &[Expr],
    types: &HashMap<String, Type>,
    dims: &HashMap<String, Vec<ArrayBound>>,
    proc: &str,
) -> Result<()> {
    require_name(name, types, proc)?;
    let Some(dd) = dims.get(&name.to_ascii_uppercase()) else {
        return Err(Diagnostic::error(format!(
            "`{name}` is not an array"
        )));
    };
    if dd.len() != indices.len() {
        return Err(Diagnostic::error(format!(
            "array `{name}` expects {} subscripts, got {}",
            dd.len(),
            indices.len()
        )));
    }
    for e in indices {
        check_expr(e, types, dims, proc)?;
    }
    Ok(())
}

fn check_expr(
    expr: &Expr,
    types: &HashMap<String, Type>,
    dims: &HashMap<String, Vec<ArrayBound>>,
    proc: &str,
) -> Result<()> {
    match expr {
        Expr::StringLit(_)
        | Expr::IntLit(_)
        | Expr::FloatLit(_)
        | Expr::Null
        | Expr::BitLit { .. } => Ok(()),
        Expr::Addr { name } => {
            if dims.contains_key(&name.to_ascii_uppercase()) {
                return Err(Diagnostic::error(format!(
                    "ADDR of array `{name}` not implemented on L0"
                )));
            }
            require_name(name, types, proc)
        }
        Expr::Name(n) => {
            if dims.contains_key(&n.to_ascii_uppercase()) {
                return Err(Diagnostic::error(format!(
                    "array `{n}` requires subscripts"
                )));
            }
            require_name(n, types, proc)
        }
        Expr::Index { base, indices } => check_index_use(base, indices, types, dims, proc),
        Expr::Field { path } => {
            field_type(path, types, proc)?;
            Ok(())
        }
        Expr::Binary { left, right, .. } => {
            check_expr(left, types, dims, proc)?;
            check_expr(right, types, dims, proc)?;
            if expr_is_char(left, types) || expr_is_char(right, types) {
                return Err(Diagnostic::error(
                    "CHAR arithmetic not supported on L0",
                ));
            }
            Ok(())
        }
    }
}

fn struct_layout_eq(decls: &[Decl], a: &str, b: &str) -> bool {
    match (struct_layout_sig(decls, a), struct_layout_sig(decls, b)) {
        (Some(la), Some(lb)) => la == lb,
        _ => false,
    }
}

fn struct_layout_sig(decls: &[Decl], major: &str) -> Option<String> {
    let mut seen = HashSet::new();
    let mut name = major.to_ascii_uppercase();
    let idx = loop {
        if !seen.insert(name.clone()) {
            return None;
        }
        let idx = decls.iter().position(|d| {
            d.level == Some(1)
                && matches!(d.ty, Type::Struct)
                && eq_ignore_ascii_case(&d.name, &name)
        })?;
        if let Some(src) = &decls[idx].like_of {
            name = src.to_ascii_uppercase();
            continue;
        }
        break idx;
    };
    let mut i = idx + 1;
    let mut sig = String::new();
    struct_sig_items(decls, &mut i, 1, &mut sig);
    Some(sig)
}

fn struct_sig_items(decls: &[Decl], i: &mut usize, parent_level: u32, out: &mut String) {
    while *i < decls.len() {
        let Some(lev) = decls[*i].level else {
            break;
        };
        if lev <= parent_level {
            break;
        }
        if lev != parent_level + 1 {
            *i += 1;
            continue;
        }
        let d = &decls[*i];
        *i += 1;
        out.push('{');
        out.push_str(&d.name.to_ascii_uppercase());
        out.push(':');
        if matches!(d.ty, Type::Struct) {
            out.push_str("STRUCT");
            struct_sig_items(decls, i, lev, out);
        } else {
            match &d.ty {
                Type::FixedBin { precision } => out.push_str(&format!("FB{precision}")),
                Type::Float => out.push_str("FL"),
                Type::Char { len } => out.push_str(&format!("CH{len}")),
                _ => out.push('?'),
            }
        }
        out.push('}');
    }
}

fn check_defined_decls(
    decls: &[Decl],
    types: &HashMap<String, Type>,
    proc: &str,
) -> Result<()> {
    for d in decls {
        let Some(base) = &d.defined_on else {
            continue;
        };
        let bk = base.to_ascii_uppercase();
        let Some(base_ty) = types.get(&bk) else {
            return Err(Diagnostic::error(format!(
                "DEFINED `{base}` undeclared for `{}` in procedure `{proc}`",
                d.name
            )));
        };
        let compatible = match (&d.ty, base_ty) {
            (Type::FixedBin { .. }, Type::FixedBin { .. }) => true,
            (Type::Float, Type::Float) => true,
            (Type::Char { len: a }, Type::Char { len: b }) => a == b,
            _ => false,
        };
        if !compatible {
            return Err(Diagnostic::error(format!(
                "DEFINED `{}` type must match base `{base}` on L0",
                d.name
            )));
        }
        if decls
            .iter()
            .find(|x| eq_ignore_ascii_case(&x.name, base))
            .is_some_and(|x| x.defined_on.is_some())
        {
            return Err(Diagnostic::error(format!(
                "DEFINED chain via `{base}` not implemented on L0"
            )));
        }
    }
    Ok(())
}

fn check_like_decls(
    decls: &[Decl],
    types: &HashMap<String, Type>,
    proc: &str,
) -> Result<()> {
    for d in decls {
        let Some(src) = &d.like_of else {
            continue;
        };
        if d.level != Some(1) || !matches!(d.ty, Type::Struct) {
            return Err(Diagnostic::error(format!(
                "LIKE on `{}` is only supported for level-1 STRUCTURE",
                d.name
            )));
        }
        let mut seen = HashSet::new();
        let mut name = src.to_ascii_uppercase();
        loop {
            if !seen.insert(name.clone()) {
                return Err(Diagnostic::error(format!(
                    "LIKE cycle involving `{}` in procedure `{proc}`",
                    d.name
                )));
            }
            match types.get(&name) {
                Some(Type::Struct) => {}
                Some(_) => {
                    return Err(Diagnostic::error(format!(
                        "LIKE `{src}` is not a STRUCTURE in procedure `{proc}`"
                    )));
                }
                None => {
                    return Err(Diagnostic::error(format!(
                        "LIKE `{src}` undeclared in procedure `{proc}`"
                    )));
                }
            }
            let idx = decls
                .iter()
                .position(|x| {
                    x.level == Some(1)
                        && matches!(x.ty, Type::Struct)
                        && eq_ignore_ascii_case(&x.name, &name)
                })
                .ok_or_else(|| {
                    Diagnostic::error(format!(
                        "LIKE `{src}` undeclared in procedure `{proc}`"
                    ))
                })?;
            if let Some(next) = &decls[idx].like_of {
                name = next.to_ascii_uppercase();
                continue;
            }
            // Must have at least one level-2 member (or allow empty).
            break;
        }
    }
    Ok(())
}

fn field_type<'a>(
    path: &[String],
    types: &'a HashMap<String, Type>,
    proc: &str,
) -> Result<&'a Type> {
    if path.len() < 2 {
        return Err(Diagnostic::error(
            "structure reference needs at least MAJOR.MEMBER",
        ));
    }
    if path.len() > 3 {
        return Err(Diagnostic::error(
            "structure qualification deeper than 3 levels not implemented",
        ));
    }
    for (i, part) in path.iter().enumerate() {
        let key = part.to_ascii_uppercase();
        let Some(ty) = types.get(&key) else {
            return Err(Diagnostic::error(format!(
                "undeclared name `{part}` in procedure `{proc}`"
            )));
        };
        let is_last = i + 1 == path.len();
        if is_last {
            return match ty {
                Type::FixedBin { .. } | Type::Float | Type::Char { .. } => Ok(ty),
                Type::Struct => Err(Diagnostic::error(format!(
                    "`{part}` is a STRUCTURE; qualify an elementary member"
                ))),
                _ => Err(Diagnostic::error(format!(
                    "member `{part}` type not usable on L0"
                ))),
            };
        }
        if !matches!(ty, Type::Struct) {
            return Err(Diagnostic::error(format!(
                "`{part}` is not a STRUCTURE in qualification"
            )));
        }
    }
    unreachable!("path length >= 2")
}

fn expr_is_char(expr: &Expr, types: &HashMap<String, Type>) -> bool {
    match expr {
        Expr::StringLit(_) => true,
        Expr::Name(n) => matches!(
            types.get(&n.to_ascii_uppercase()),
            Some(Type::Char { .. })
        ),
        Expr::Field { path } => matches!(
            types.get(&path[path.len() - 1].to_ascii_uppercase()),
            Some(Type::Char { .. })
        ),
        _ => false,
    }
}

fn require_name(name: &str, types: &HashMap<String, Type>, proc: &str) -> Result<()> {
    if types.contains_key(&name.to_ascii_uppercase()) {
        Ok(())
    } else {
        Err(Diagnostic::error(format!(
            "undeclared name `{name}` in procedure `{proc}`"
        )))
    }
}

fn require_fixed(
    name: &str,
    types: &HashMap<String, Type>,
    dims: &HashMap<String, Vec<ArrayBound>>,
    proc: &str,
) -> Result<()> {
    if dims.contains_key(&name.to_ascii_uppercase()) {
        return Err(Diagnostic::error(format!(
            "DO control variable `{name}` must be a scalar FIXED BIN"
        )));
    }
    match types.get(&name.to_ascii_uppercase()) {
        Some(Type::FixedBin { .. }) => Ok(()),
        Some(_) => Err(Diagnostic::error(format!(
            "DO control variable `{name}` must be FIXED BIN"
        ))),
        None => Err(Diagnostic::error(format!(
            "undeclared name `{name}` in procedure `{proc}`"
        ))),
    }
}
