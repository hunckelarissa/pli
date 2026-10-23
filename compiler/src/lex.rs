//! Lexer for PL/I-G (L0 slice).
//! Keywords are not reserved; classification happens in the parser.

use crate::diagnostic::{Diagnostic, Result};

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Ident(String),
    StringLit(String),
    /// Bit string literal `'1010'B` (binary digits, value + bit width).
    BitLit { value: u64, width: u32 },
    IntLit(i64),
    FloatLit(f64),
    /// Single-character / operator punctuation as text.
    Punct(&'static str),
    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
    pub column: usize,
}

pub struct Lexer<'a> {
    src: &'a str,
    bytes: &'a [u8],
    pos: usize,
    line: usize,
    column: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            src,
            bytes: src.as_bytes(),
            pos: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn tokenize(mut self) -> Result<Vec<Token>> {
        let mut out = Vec::new();
        loop {
            let tok = self.next_token()?;
            let is_eof = matches!(tok.kind, TokenKind::Eof);
            out.push(tok);
            if is_eof {
                break;
            }
        }
        Ok(out)
    }

    fn next_token(&mut self) -> Result<Token> {
        self.skip_ws_and_comments()?;
        let line = self.line;
        let column = self.column;
        if self.pos >= self.bytes.len() {
            return Ok(Token {
                kind: TokenKind::Eof,
                line,
                column,
            });
        }
        let b = self.bytes[self.pos];
        // string literal '...' or bit string `'1010'B`
        if b == b'\'' {
            let s = self.lex_string()?;
            if self.pos < self.bytes.len()
                && (self.bytes[self.pos] == b'B' || self.bytes[self.pos] == b'b')
                && (self.pos + 1 >= self.bytes.len()
                    || !is_ident_continue(self.bytes[self.pos + 1]))
            {
                self.bump();
                let kind = parse_bit_lit(&s, line, column)?;
                return Ok(Token {
                    kind,
                    line,
                    column,
                });
            }
            return Ok(Token {
                kind: TokenKind::StringLit(s),
                line,
                column,
            });
        }
        // number (integer or float)
        if b.is_ascii_digit() {
            let kind = self.lex_number()?;
            return Ok(Token {
                kind,
                line,
                column,
            });
        }
        // identifier / keyword text
        if is_ident_start(b) {
            let id = self.lex_ident();
            return Ok(Token {
                kind: TokenKind::Ident(id),
                line,
                column,
            });
        }
        // multi-char / single punctuation
        if b == b'<' {
            self.bump();
            if self.pos < self.bytes.len() && self.bytes[self.pos] == b'>' {
                self.bump();
                return Ok(Token {
                    kind: TokenKind::Punct("<>"),
                    line,
                    column,
                });
            }
            if self.pos < self.bytes.len() && self.bytes[self.pos] == b'=' {
                self.bump();
                return Ok(Token {
                    kind: TokenKind::Punct("<="),
                    line,
                    column,
                });
            }
            return Ok(Token {
                kind: TokenKind::Punct("<"),
                line,
                column,
            });
        }
        if b == b'>' {
            self.bump();
            if self.pos < self.bytes.len() && self.bytes[self.pos] == b'=' {
                self.bump();
                return Ok(Token {
                    kind: TokenKind::Punct(">="),
                    line,
                    column,
                });
            }
            return Ok(Token {
                kind: TokenKind::Punct(">"),
                line,
                column,
            });
        }
        let punct = match b {
            b';' => ";",
            b',' => ",",
            b'(' => "(",
            b')' => ")",
            b':' => ":",
            b'=' => "=",
            b'.' => ".",
            b'+' => "+",
            b'-' => "-",
            b'*' => "*",
            b'/' => "/",
            b'%' => "%",
            b'&' => "&",
            b'|' => "|",
            other => {
                return Err(Diagnostic::error(format!(
                    "unexpected character {:?}",
                    other as char
                ))
                .at(line, column));
            }
        };
        self.bump();
        Ok(Token {
            kind: TokenKind::Punct(punct),
            line,
            column,
        })
    }

    fn skip_ws_and_comments(&mut self) -> Result<()> {
        loop {
            while self.pos < self.bytes.len() {
                let b = self.bytes[self.pos];
                if b == b' ' || b == b'\t' || b == b'\r' {
                    self.bump();
                    continue;
                }
                if b == b'\n' {
                    self.bump();
                    continue;
                }
                break;
            }
            // /* ... */
            if self.pos + 1 < self.bytes.len()
                && self.bytes[self.pos] == b'/'
                && self.bytes[self.pos + 1] == b'*'
            {
                self.bump();
                self.bump();
                while self.pos + 1 < self.bytes.len() {
                    if self.bytes[self.pos] == b'*' && self.bytes[self.pos + 1] == b'/' {
                        self.bump();
                        self.bump();
                        break;
                    }
                    self.bump();
                }
                continue;
            }
            break;
        }
        Ok(())
    }

    fn lex_string(&mut self) -> Result<String> {
        let line = self.line;
        let column = self.column;
        self.bump(); // opening '
        let mut s = String::new();
        while self.pos < self.bytes.len() {
            let b = self.bytes[self.pos];
            if b == b'\'' {
                self.bump();
                // doubled quote escape
                if self.pos < self.bytes.len() && self.bytes[self.pos] == b'\'' {
                    s.push('\'');
                    self.bump();
                    continue;
                }
                return Ok(s);
            }
            s.push(b as char);
            self.bump();
        }
        Err(Diagnostic::error("unterminated string literal").at(line, column))
    }

    fn lex_number(&mut self) -> Result<TokenKind> {
        let start = self.pos;
        let line = self.line;
        let column = self.column;
        while self.pos < self.bytes.len() && self.bytes[self.pos].is_ascii_digit() {
            self.bump();
        }
        let mut is_float = false;
        if self.pos < self.bytes.len() && self.bytes[self.pos] == b'.' {
            // Distinguish `1.5` from `1..` / lone trailing dot before non-digit.
            if self.pos + 1 < self.bytes.len() && self.bytes[self.pos + 1].is_ascii_digit() {
                is_float = true;
                self.bump(); // .
                while self.pos < self.bytes.len() && self.bytes[self.pos].is_ascii_digit() {
                    self.bump();
                }
            }
        }
        let text = &self.src[start..self.pos];
        if is_float {
            text.parse::<f64>().map(TokenKind::FloatLit).map_err(|_| {
                Diagnostic::error(format!("invalid float literal `{text}`")).at(line, column)
            })
        } else {
            text.parse::<i64>().map(TokenKind::IntLit).map_err(|_| {
                Diagnostic::error(format!("invalid integer literal `{text}`")).at(line, column)
            })
        }
    }

    fn lex_ident(&mut self) -> String {
        let start = self.pos;
        self.bump();
        while self.pos < self.bytes.len() && is_ident_continue(self.bytes[self.pos]) {
            self.bump();
        }
        self.src[start..self.pos].to_string()
    }

    fn bump(&mut self) {
        if self.pos >= self.bytes.len() {
            return;
        }
        if self.bytes[self.pos] == b'\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        self.pos += 1;
    }
}

fn parse_bit_lit(digits: &str, line: usize, column: usize) -> Result<TokenKind> {
    if digits.is_empty() {
        return Err(Diagnostic::error("empty bit string literal").at(line, column));
    }
    if digits.len() > 64 {
        return Err(
            Diagnostic::error("bit string literal longer than 64 bits").at(line, column)
        );
    }
    let mut value: u64 = 0;
    for ch in digits.chars() {
        value <<= 1;
        match ch {
            '0' => {}
            '1' => value |= 1,
            _ => {
                return Err(Diagnostic::error(format!(
                    "invalid bit in bit string literal `{digits}'B`"
                ))
                .at(line, column));
            }
        }
    }
    Ok(TokenKind::BitLit {
        value,
        width: digits.len() as u32,
    })
}

fn is_ident_start(b: u8) -> bool {
    b.is_ascii_alphabetic() || b == b'_' || b == b'$' || b == b'#' || b == b'@'
}

fn is_ident_continue(b: u8) -> bool {
    is_ident_start(b) || b.is_ascii_digit()
}

pub fn eq_ignore_ascii_case(a: &str, b: &str) -> bool {
    a.eq_ignore_ascii_case(b)
}
