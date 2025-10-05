use lm_compiler::{
    compiler::{
        context::{COMPILER_CONTEXT, CompilerContext},
        error::CompilerError,
    },
    grammar::{RulesParser, rules_lexer::LexerAdapter},
};
use rustemo::Parser;
use std::path::Path;

fn integration_test(path: &Path) -> datatest_stable::Result<()> {
    // This is done because the COMPILER_CONTEXT needs to be reset between test runs.
    COMPILER_CONTEXT.replace(CompilerContext::new());

    let source = COMPILER_CONTEXT.with(|ctx| -> Result<String, CompilerError> {
        let mut context = ctx.borrow_mut();
        context.init_compiler_context(path.into())?;
        Ok(context.source().clone())
    })?;

    Ok(RulesParser::new(LexerAdapter::new())
        .parse(&source)
        .map_err(|err| err.to_string())
        .map(|_| ())?)
}

#[cfg(test)]
datatest_stable::harness!(
    { test = integration_test, root = "./examples", pattern = r".*\.lm" },
    { test = integration_test, root = "./inputs", pattern = r"test.txt" }
);
