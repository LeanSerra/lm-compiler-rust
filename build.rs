use rustemo_compiler::{LexerType, Settings};
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=src/grammar.rustemo");
    println!("cargo:rerun-if-changed=src/lex.l");

    let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    // Compile parser
    let parser_rules = root_dir.join("src/grammar.rustemo");

    if let Err(e) = Settings::new()
        .force(false) // Don't force regeneration of files grammar.rs and grammar_actions.rs
        .out_dir_actions_root(root_dir.clone()) // Output directory for generated actions
        .out_dir_root(root_dir.clone()) // Output directory for generated parser
        .lexer_type(LexerType::Custom) // Use our own Lexer
        .process_grammar(&parser_rules)
    {
        eprintln!("Failed to compile rules: {e}");
        std::process::exit(1)
    }
    // Compile Lexer
    let lexer_src = root_dir.join("src/lex.l");
    let lexer_out = root_dir.join("src/lex.rs");

    if let Err(e) = rflex::process(lexer_src, Some(lexer_out)) {
        for cause in <dyn failure::Fail>::iter_chain(&e) {
            eprintln!("{}: {}", cause.name().unwrap_or("Error"), cause);
        }
        std::process::exit(1);
    }
}
