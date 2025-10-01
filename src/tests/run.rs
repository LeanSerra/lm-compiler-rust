use lm_compiler::{
    GrammarParser, LexerAdapter, dump_symbol_table_to_file, open_lexer_file, open_parser_file,
    open_symbol_table_file, read_source_to_string, set_source_file_path,
};
use rustemo::Parser;
use std::path::Path;

fn integration_test(path: &Path) -> datatest_stable::Result<()> {
    set_source_file_path(path.into());
    open_lexer_file()?;
    open_parser_file()?;
    open_symbol_table_file()?;

    GrammarParser::new(LexerAdapter::new())
        .parse(&read_source_to_string().map_err(|err| err.to_string())?)
        .map_err(|err| err.to_string().into())
        .and_then(|_program| dump_symbol_table_to_file().map_err(|e| e.to_string().into()))
}

#[cfg(test)]
datatest_stable::harness!(
    { test = integration_test, root = "./examples", pattern = r".*\.lm" },
    { test = integration_test, root = "./inputs", pattern = r"test.txt" }
);
