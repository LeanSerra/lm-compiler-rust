use lm_compiler::{
    GrammarParser, LexerAdapter, open_lexer_file, open_parser_file, open_symbol_table_file,
    read_source_to_string, set_source_file_path,
};
use rustemo::Parser;
use std::path::Path;

fn integration_test(path: &Path) -> datatest_stable::Result<()> {
    set_source_file_path(path.to_str().ok_or("Failed to get path")?);
    open_lexer_file()?;
    open_parser_file()?;
    open_symbol_table_file()?;

    GrammarParser::new(LexerAdapter::new())
        .parse(&read_source_to_string().map_err(|err| err.to_string())?)
        .map_err(|err| err.to_string().into())
        .map(|_program| ())
}

#[cfg(test)]
datatest_stable::harness!( { test = integration_test, root = "./examples", pattern = r".*\.lm" },);
