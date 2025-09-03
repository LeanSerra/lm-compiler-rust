use lm_compiler::{GrammarParser, LexerAdapter};
use rustemo::Parser;
use std::path::Path;

fn integration_test(path: &Path) -> datatest_stable::Result<()> {
    GrammarParser::new(LexerAdapter::new())
        .parse(&std::fs::read_to_string(path).unwrap())
        .map_err(|err| err.to_string().into())
        .map(|_program| ())
}

#[cfg(test)]
datatest_stable::harness!( { test = integration_test, root = "./examples", pattern = r".*\.lm" },);
