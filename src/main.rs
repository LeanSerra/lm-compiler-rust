use clap::Parser as ClapParser;
use lm_compiler::{
    compiler::{context::Compiler, error::CompilerError},
    grammar::RulesParser,
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
    let compiler = Compiler::new(cli.input.clone())?;

    let rules = RulesParser::new(compiler.clone(), compiler.clone())
        .parse_file(cli.input)
        .map_err(CompilerError::ParserInternal)?;

    println!("{rules}");

    Ok(())
}
