//! PL/I-G compiler library (`pl1c`).

pub mod codegen_c;
pub mod diagnostic;
pub mod hir;
pub mod lex;
pub mod parse;
pub mod sema;

use crate::codegen_c::{emit_c, EmitOptions};
use crate::diagnostic::{Diagnostic, Result};
use crate::hir::{Block, Decl, FormatItem, Module, Stmt, Storage, Type};
use crate::lex::{eq_ignore_ascii_case, Lexer};
use crate::parse::Parser;

pub fn compile_source(source: &str, path: &str) -> Result<Module> {
    let tokens = Lexer::new(source).tokenize()?;
    let mut parser = Parser::new(&tokens, path);
    let module = parser.parse_module()?;
    sema::check(&module)?;
    Ok(module)
}

pub fn compile_to_c(source: &str, path: &str, emit_line: bool) -> Result<String> {
    let module = compile_source(source, path)?;
    reject_unimplemented(&module)?;
    Ok(emit_c(
        &module,
        &EmitOptions {
            emit_line,
        },
    ))
}

fn reject_unimplemented(module: &Module) -> Result<()> {
    for proc in &module.procs {
        reject_block(&proc.body)?;
    }
    Ok(())
}

fn reject_block(body: &Block) -> Result<()> {
    for d in &body.decls {
        reject_decl(d)?;
    }
    for stmt in &body.stmts {
        reject_stmt(stmt)?;
    }
    Ok(())
}

fn reject_decl(d: &Decl) -> Result<()> {
    if d.defined_on.is_some() {
        if d.dims.is_some() || d.level.is_some() {
            return Err(Diagnostic::error(format!(
                "DEFINED array/STRUCTURE `{}` not implemented",
                d.name
            )));
        }
        if matches!(d.storage, Storage::Based) || d.based_on.is_some() {
            return Err(Diagnostic::error(format!(
                "DEFINED+BASED `{}` not implemented",
                d.name
            )));
        }
        return match &d.ty {
            Type::FixedBin { .. } | Type::Float | Type::Char { .. } => Ok(()),
            _ => Err(Diagnostic::error(format!(
                "DEFINED `{}` type not implemented on L0",
                d.name
            ))),
        };
    }
    if matches!(d.storage, Storage::Based) || d.based_on.is_some() {
        if d.based_on.is_none() {
            return Err(Diagnostic::error(format!(
                "BASED `{}` requires a pointer, e.g. BASED(P)",
                d.name
            )));
        }
        if d.dims.is_some() {
            return Err(Diagnostic::error(format!(
                "BASED array `{}` not implemented",
                d.name
            )));
        }
        if d.level == Some(1) && matches!(d.ty, Type::Struct) {
            return Ok(());
        }
        if d.level.is_some() {
            return Err(Diagnostic::error(format!(
                "BASED structure member `{}` not implemented (use BASED on level-1)",
                d.name
            )));
        }
        return match &d.ty {
            Type::FixedBin { .. } | Type::Float | Type::Char { .. } => Ok(()),
            Type::Bit { len } if (1..=8).contains(len) => Ok(()),
            _ => Err(Diagnostic::error(format!(
                "BASED `{}` type not implemented on L0",
                d.name
            ))),
        };
    }
    match d.level {
        Some(1) if matches!(d.ty, Type::Struct) => {
            if d.dims.is_some() {
                return Err(Diagnostic::error(format!(
                    "STRUCTURE array `{}` not implemented",
                    d.name
                )));
            }
            // LIKE expansion is handled in codegen/sema.
            return Ok(());
        }
        Some(lev) if (2..=3).contains(&lev) => {
            if d.dims.is_some() {
                return Err(Diagnostic::error(format!(
                    "structure member array `{}` not implemented",
                    d.name
                )));
            }
            match &d.ty {
                Type::Struct | Type::FixedBin { .. } | Type::Float | Type::Char { .. } => {
                    return Ok(());
                }
                _ => {
                    return Err(Diagnostic::error(format!(
                        "structure member `{}` type not implemented on L0",
                        d.name
                    )));
                }
            }
        }
        Some(_) => {
            return Err(Diagnostic::error(format!(
                "structure level for `{}` not implemented (only 1..=3)",
                d.name
            )));
        }
        None => {}
    }
    if let Some(dims) = &d.dims {
        match &d.ty {
            Type::FixedBin { .. } | Type::Float | Type::Char { .. } => {
                if dims.is_empty() {
                    return Err(Diagnostic::error(format!(
                        "array `{}` has empty bounds",
                        d.name
                    )));
                }
            }
            Type::Bit { .. } | Type::Pointer | Type::Struct => {
                return Err(Diagnostic::error(format!(
                    "array of BIT/POINTER/STRUCT `{}` not implemented",
                    d.name
                )));
            }
        }
    }
    match d.ty {
        Type::Bit { len } => {
            if d.dims.is_some() {
                Err(Diagnostic::error(format!(
                    "BIT array `{}` not implemented",
                    d.name
                )))
            } else if !(1..=8).contains(&len) {
                Err(Diagnostic::error(format!(
                    "BIT({}) `{}` not implemented (L0: BIT(1..8))",
                    len, d.name
                )))
            } else {
                Ok(())
            }
        }
        Type::Pointer => {
            if d.dims.is_some() {
                Err(Diagnostic::error(format!(
                    "POINTER array `{}` not implemented",
                    d.name
                )))
            } else {
                Ok(())
            }
        }
        Type::Struct => Err(Diagnostic::error(format!(
            "STRUCTURE `{}` without level is not implemented",
            d.name
        ))),
        Type::FixedBin { .. } | Type::Char { .. } | Type::Float => Ok(()),
    }
}

fn reject_stmt(stmt: &Stmt) -> Result<()> {
    match stmt {
        Stmt::Labeled { body, .. } => reject_stmt(body),
        Stmt::Select {
            whens,
            otherwise,
            ..
        } => {
            for w in whens {
                reject_stmt(&w.action)?;
            }
            if let Some(o) = otherwise {
                reject_stmt(o)?;
            }
            Ok(())
        }
        Stmt::If {
            then_branch,
            else_branch,
            ..
        } => {
            reject_stmt(then_branch)?;
            if let Some(e) = else_branch {
                reject_stmt(e)?;
            }
            Ok(())
        }
        Stmt::DoCounted { body, .. }
        | Stmt::DoGroup { body }
        | Stmt::DoWhile { body, .. }
        | Stmt::DoUntil { body, .. } => {
            for s in body {
                reject_stmt(s)?;
            }
            Ok(())
        }
        Stmt::Begin { body } => reject_block(body),
        Stmt::PutList { file, .. }
        | Stmt::GetList { file, .. }
        | Stmt::PutData { file, .. }
        | Stmt::GetData { file, .. } => reject_io_file(file),
        Stmt::PutEdit { file, formats, .. } | Stmt::GetEdit { file, formats, .. } => {
            reject_io_file(file)?;
            for f in formats {
                reject_format(f)?;
            }
            Ok(())
        }
        Stmt::On {
            condition,
            action,
            ..
        } => {
            if let crate::hir::OnAction::Stmt(s) = action {
                reject_stmt(s)?;
            }
            if eq_ignore_ascii_case(condition, "ENDFILE") {
                return Ok(());
            }
            Err(Diagnostic::error(
                "ON condition is parse-only on L0 (only ON ENDFILE is implemented)",
            ))
        }
        Stmt::Signal { condition, .. } => {
            if eq_ignore_ascii_case(condition, "ENDFILE") {
                Ok(())
            } else {
                Err(Diagnostic::error(
                    "SIGNAL is parse-only on L0 (only SIGNAL ENDFILE is implemented)",
                ))
            }
        }
        Stmt::Revert { condition, .. } => {
            if eq_ignore_ascii_case(condition, "ENDFILE") {
                Ok(())
            } else {
                Err(Diagnostic::error(
                    "REVERT is parse-only on L0 (only REVERT ENDFILE is implemented)",
                ))
            }
        }
        Stmt::Allocate { .. } | Stmt::Free { .. } => Ok(()),
        Stmt::Leave
        | Stmt::Iterate
        | Stmt::Return
        | Stmt::Stop
        | Stmt::Goto { .. }
        | Stmt::Null => Ok(()),
        _ => Ok(()),
    }
}

fn reject_io_file(_file: &Option<String>) -> Result<()> {
    Ok(())
}

fn reject_format(fmt: &FormatItem) -> Result<()> {
    match fmt {
        FormatItem::Call { name, args }
            if eq_ignore_ascii_case(name, "P") || eq_ignore_ascii_case(name, "PICTURE") =>
        {
            match args.as_slice() {
                [crate::hir::Expr::StringLit(_)] => Ok(()),
                _ => Err(Diagnostic::error(
                    "P/PICTURE requires a string picture argument",
                )),
            }
        }
        FormatItem::Name(name)
            if eq_ignore_ascii_case(name, "P") || eq_ignore_ascii_case(name, "PICTURE") =>
        {
            Err(Diagnostic::error(
                "bare P/PICTURE requires a picture string (P'…' or P('…'))",
            ))
        }
        _ => Ok(()),
    }
}
