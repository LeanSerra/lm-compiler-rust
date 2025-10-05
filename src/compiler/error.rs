use owo_colors::OwoColorize;
use std::ops::Range;
use thiserror::Error;

use crate::compiler::context::{COMPILER_CONTEXT, CompilerContext};

#[derive(Debug, Error)]
pub enum CompilerError {
    #[error("Parser internal error: {0:?}")]
    ParserInternal(rustemo::Error),
    #[error("Parser error: {0}")]
    Parser(String),
    #[error("Lexer error: {0}")]
    Lexer(String),
    #[error("Context error: {0}")]
    Context(String),
    #[error("IO error: {0}")]
    IO(String),
}

pub fn log_error_and_exit(
    pos: Range<usize>,
    error: CompilerError,
    offset: usize,
    trace: bool,
) -> ! {
    COMPILER_CONTEXT.with(|ctx| {
        let mut context = ctx.borrow_mut();
        if let Err(e) = context.dump_symbol_table_to_file() {
            eprintln!("Failed to write symbol table: {e}")
        }
        log_error(pos, error, offset, &mut context, trace);
    });
    std::process::exit(1)
}

fn log_error(
    token_pos: Range<usize>,
    err: CompilerError,
    offset: usize,
    context: &mut CompilerContext,
    trace: bool,
) {
    let path = context.path();
    let source = &context.source();

    let (line_starts, (line_in_file, col_in_file)) =
        pos_to_line_col(source, token_pos.start + offset);

    let line_start = line_starts[line_in_file - 1];
    let line_end = source[line_start..]
        .find('\n')
        .map(|e| line_start + e)
        .unwrap_or(source.len());
    let line_text = &source[line_start..line_end];

    let span_len = std::cmp::min(
        token_pos.end - token_pos.start,
        line_text.len() - (col_in_file - 1),
    );
    let mut underline = String::new();
    underline.push_str(&" ".repeat(col_in_file - 1));
    underline.push_str(&"^".repeat(span_len));
    eprintln!("{}: {}", "error".red().bold(), err.to_string().bold());
    eprintln!(
        "  --> {}:{}:{}",
        path.bright_blue(),
        line_in_file.blue(),
        col_in_file.blue()
    );
    if trace {
        eprintln!("   {}", "|".dimmed());
        eprintln!(
            "{:>3}{} {}",
            line_in_file.to_string().blue(),
            "|".dimmed(),
            line_text
        );
        eprintln!(
            "   {} {} {}",
            "|".dimmed(),
            underline.bold().red(),
            err.bold().red()
        );
    } else {
        eprintln!("   {}", err.bold().red());
    }
    eprintln!()
}

fn pos_to_line_col(source: &str, pos: usize) -> (Vec<usize>, (usize, usize)) {
    let mut line_starts = vec![0];
    for (i, ch) in source.char_indices() {
        if ch == '\n' {
            line_starts.push(i + 1);
        }
    }

    let line = line_starts.partition_point(|&start| start <= pos);

    let col = pos - line_starts[line - 1] + 1;

    (line_starts, (line, col))
}
