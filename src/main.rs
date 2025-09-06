use clap::Parser as ClapParser;
use lm_compiler::{
    CompilerError, GrammarParser, LexerAdapter, open_lexer_file, open_parser_file,
    open_symbol_table_file, read_parser_file_to_string, read_source_to_string,
    set_source_file_path,
};
use rustemo::Parser;
use std::path::PathBuf;

#[derive(ClapParser)]
#[command(
    version,
    about = "Simple compiler written in Rust for the Compilers & Languages class at UNLaM"
)]
struct Cli {
    #[arg(
        help = "Path to the source code file",
        default_value = "./Inputs/test.txt"
    )]
    input: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    set_source_file_path(cli.input);
    open_lexer_file()?;
    open_parser_file()?;
    open_symbol_table_file()?;

    let _program = GrammarParser::new(LexerAdapter::new())
        .parse(&read_source_to_string()?)
        .map_err(CompilerError::ParserInternal)?;

    println!("{}", read_parser_file_to_string()?);

    Ok(())
}
