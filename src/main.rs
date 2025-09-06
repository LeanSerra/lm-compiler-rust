use lm_compiler::{
    CompilerError, GrammarParser, LexerAdapter, open_lexer_file, open_parser_file,
    open_symbol_table_file, read_parser_file_to_string, read_source_to_string,
    set_source_file_path,
};
use rustemo::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    set_source_file_path("./examples/hello_world.lm");
    open_lexer_file()?;
    open_parser_file()?;
    open_symbol_table_file()?;

    let _program = GrammarParser::new(LexerAdapter::new())
        .parse(&read_source_to_string()?)
        .map_err(CompilerError::Parser)?;

    println!("{}", read_parser_file_to_string()?);

    Ok(())
}
