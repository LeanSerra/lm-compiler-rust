use lm_compiler::{compiler::context::Compiler, grammar::RulesParser};
use rustemo::Parser;
use std::path::Path;

fn integration_test(path: &Path) -> datatest_stable::Result<()> {
    let compiler = Compiler::new(path.into())?;
    RulesParser::new(compiler.clone(), compiler.clone())
        .parse_file(path)
        .map_err(|e| e.to_string())
        .map(|_| ())?;
    Ok(compiler.inner.borrow_mut().generate_asm()?)
}

#[cfg(test)]
datatest_stable::harness!(
    { test = integration_test, root = "./examples", pattern = r".*\.lm" },
    { test = integration_test, root = "./inputs", pattern = r"test.txt" }
);
