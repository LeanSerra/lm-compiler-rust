use rustemo_compiler::{BuilderType, LexerType, Settings};
use std::path::PathBuf;

const GRAMMAR_FILE_PATH: &str = "src/grammar/rules.rustemo";
const LEXER_FILE_PATH: &str = "src/lexer/lex.l";

fn main() {
    println!("cargo:rerun-if-changed={GRAMMAR_FILE_PATH}");
    println!("cargo:rerun-if-changed={LEXER_FILE_PATH}");

    let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    // Compile parser
    if let Err(e) = Settings::new()
        .out_dir_root(root_dir.clone())
        .lexer_type(LexerType::Custom)
        .builder_type(BuilderType::Custom)
        .process_grammar(&root_dir.join(GRAMMAR_FILE_PATH))
    {
        eprintln!("Failed to compile rules: {e}");
        std::process::exit(1)
    }
    // Compile Lexer
    let lexer_src = root_dir.join(LEXER_FILE_PATH);
    let lexer_out = root_dir.join("src/lexer/lex.rs");

    if let Err(e) = rflex::process(lexer_src, Some(lexer_out)) {
        for cause in <dyn failure::Fail>::iter_chain(&e) {
            eprintln!("{}: {}", cause.name().unwrap_or("Error"), cause);
        }
        std::process::exit(1);
    }
}
