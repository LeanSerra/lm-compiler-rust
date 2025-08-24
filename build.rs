use std::path::Path;

fn main() {
    let dest = Path::new("src").join("lex.rs");
    let path = Path::new("src").join("lex.l");
    if let Err(e) = rflex::process(path, Some(dest)) {
        for cause in <dyn failure::Fail>::iter_chain(&e) {
            eprintln!("{}: {}", cause.name().unwrap_or("Error"), cause);
        }
        std::process::exit(1);
    }
}
