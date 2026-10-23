//! pl1c — PL/I-G compiler driver (transpile to C, link with libpl1rt).

use pl1c::{compile_source, compile_to_c};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{self, Command};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    if args.is_empty() || args.iter().any(|a| a == "-h" || a == "--help") {
        print_help();
        return Ok(());
    }
    if args.iter().any(|a| a == "-V" || a == "--version") {
        println!("pl1c {VERSION} (PL/I-G)");
        return Ok(());
    }

    let mut emit_c_only: Option<PathBuf> = None;
    let mut output: Option<PathBuf> = None;
    let mut no_line = false;
    let mut parse_only = false;
    let mut cc = env::var("PL1C_CC").unwrap_or_else(|_| "gcc".to_string());
    let mut inputs = Vec::new();

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--emit-c" => {
                i += 1;
                let p = args
                    .get(i)
                    .ok_or("--emit-c requires a path")?
                    .clone();
                emit_c_only = Some(PathBuf::from(p));
            }
            "-o" => {
                i += 1;
                let p = args.get(i).ok_or("-o requires a path")?.clone();
                output = Some(PathBuf::from(p));
            }
            "--no-line" => no_line = true,
            "--parse-only" => parse_only = true,
            "--cc" => {
                i += 1;
                cc = args.get(i).ok_or("--cc requires a program")?.clone();
            }
            s if s.starts_with('-') => {
                return Err(format!("unknown option: {s}").into());
            }
            s => inputs.push(PathBuf::from(s)),
        }
        i += 1;
    }

    if inputs.len() != 1 {
        return Err("exactly one input .pli file required".into());
    }
    let input = &inputs[0];
    let source = fs::read_to_string(input)?;
    let path_str = input.display().to_string();

    if parse_only {
        compile_source(&source, &path_str).map_err(|d| d.to_string())?;
        eprintln!("ok parse {}", input.display());
        return Ok(());
    }

    let c_text = compile_to_c(&source, &path_str, !no_line).map_err(|d| d.to_string())?;

    if let Some(c_path) = emit_c_only {
        fs::write(&c_path, &c_text)?;
        eprintln!("wrote {}", c_path.display());
        return Ok(());
    }

    let out_exe = output.unwrap_or_else(|| {
        input.with_extension(exe_ext())
    });
    let c_path = out_exe.with_extension("c");
    fs::write(&c_path, &c_text)?;

    let runtime_root = find_runtime_root()?;
    build_runtime(&cc, &runtime_root)?;

    let include = runtime_root.join("include");
    let lib = runtime_root.join("libpl1rt.a");

    let status = Command::new(&cc)
        .arg("-std=c11")
        .arg(format!("-I{}", include.display()))
        .arg(&c_path)
        .arg(&lib)
        .arg("-o")
        .arg(&out_exe)
        .status()?;

    if !status.success() {
        return Err(format!("{cc} failed with {status}").into());
    }
    eprintln!("ok {}", out_exe.display());
    Ok(())
}

fn exe_ext() -> &'static str {
    if cfg!(windows) {
        "exe"
    } else {
        ""
    }
}

fn find_runtime_root() -> Result<PathBuf, Box<dyn std::error::Error>> {
    // Prefer PL1C_RUNTIME, else walk up from cwd / executable-adjacent.
    if let Ok(p) = env::var("PL1C_RUNTIME") {
        return Ok(PathBuf::from(p));
    }
    let cwd = env::current_dir()?;
    for dir in cwd.ancestors().take(6) {
        let cand = dir.join("runtime");
        if cand.join("include").join("pl1rt.h").is_file() {
            return Ok(cand);
        }
    }
    // workspace: compiler/../runtime
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let cand = manifest_dir.join("../runtime");
    if cand.join("include").join("pl1rt.h").is_file() {
        return Ok(cand.canonicalize().unwrap_or(cand));
    }
    Err("cannot find runtime/ (set PL1C_RUNTIME)".into())
}

fn build_runtime(cc: &str, runtime_root: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let obj = runtime_root.join("src/put.o");
    let lib = runtime_root.join("libpl1rt.a");
    let src = runtime_root.join("src/put.c");
    let include = runtime_root.join("include");

    let need_rebuild = !lib.is_file()
        || file_mtime(&src)? > file_mtime(&lib).unwrap_or(0)
        || file_mtime(&include.join("pl1rt.h"))? > file_mtime(&lib).unwrap_or(0);
    if !need_rebuild {
        return Ok(());
    }

    let status = Command::new(cc)
        .arg("-std=c11")
        .arg("-O2")
        .arg("-Wall")
        .arg(format!("-I{}", include.display()))
        .arg("-c")
        .arg(&src)
        .arg("-o")
        .arg(&obj)
        .status()?;
    if !status.success() {
        return Err("failed to compile runtime put.c".into());
    }

    // Use gcc-ar or ar
    let ar = env::var("PL1C_AR").unwrap_or_else(|_| "ar".to_string());
    let _ = fs::remove_file(&lib);
    let status = Command::new(&ar)
        .arg("rcs")
        .arg(&lib)
        .arg(&obj)
        .status()?;
    if !status.success() {
        return Err(format!("failed to archive runtime with {ar}").into());
    }
    Ok(())
}

fn file_mtime(p: &Path) -> Result<u64, Box<dyn std::error::Error>> {
    Ok(fs::metadata(p)?
        .modified()?
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs())
}

fn print_help() {
    println!(
        "pl1c {VERSION} — PL/I-G compiler (transpile to C)

USAGE:
  pl1c [OPTIONS] <file.pli>

OPTIONS:
  -o <path>         Output executable
  --emit-c <path>   Write C only (no link)
  --parse-only      Lex/parse/sema only (must-parse tests)
  --no-line         Do not emit source markers in C
  --cc <prog>       C compiler (default: gcc, or $PL1C_CC)
  -V, --version     Print version
  -h, --help        Help

ENV:
  PL1C_RUNTIME      Path to runtime/ directory
  PL1C_CC           C compiler
  PL1C_AR           archiver (default: ar)
"
    );
}
