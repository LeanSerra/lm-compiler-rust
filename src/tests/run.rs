use lm_compiler::{compiler::context::Compiler, grammar::RulesParser};
use rustemo::Parser;
use std::path::Path;

fn integration_test(path: &Path) -> datatest_stable::Result<()> {
    let compiler = Compiler::new();
    compiler
        .inner
        .borrow_mut()
        .init_compiler_context(path.into())
        .unwrap();
    let _program = RulesParser::new(compiler.clone(), compiler.clone())
        .parse(&compiler.source())
        .unwrap();
    Ok(())
}

#[cfg(test)]
datatest_stable::harness!(
    { test = integration_test, root = "./examples", pattern = r".*\.lm" },
    { test = integration_test, root = "./inputs", pattern = r"test.txt" }
);
