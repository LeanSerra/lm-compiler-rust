use clap::Parser as ClapParser;
use lm_compiler::{
    compiler::{
        context::{COMPILER_CONTEXT, dump_symbol_table_to_file, read_parser_file_to_string},
        error::CompilerError,
    },
    grammar::{RulesParser, rules_lexer::LexerAdapter},
};
use rustemo::Parser;
use std::path::PathBuf;

#[derive(ClapParser)]
#[command(
    version,
    about = "Simple compiler written in Rust for the Compilers & Languages class at UNLaM"
)]
struct Cli {
    #[arg(help = "Path to the source code file", value_name = "INPUT_FILE")]
    input: PathBuf,
}

fn main() -> Result<(), CompilerError> {
    let cli = Cli::parse();
    let source = COMPILER_CONTEXT.with(|ctx| -> Result<String, CompilerError> {
        let mut context = ctx.borrow_mut();
        context.init_compiler_context(cli.input)?;
        Ok(context.source().clone())
    })?;

    let _program = RulesParser::new(LexerAdapter::new())
        .parse(&source)
        .map_err(CompilerError::ParserInternal)?;

    dump_symbol_table_to_file()?;
    println!("{}", read_parser_file_to_string()?);

    Ok(())
}
