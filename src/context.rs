use std::{
    cell::RefCell,
    fs::{File, OpenOptions, read_to_string},
    io::{self, Read, Seek, Write},
    path::PathBuf,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CompilerError {
    #[error("Parser internal error: {0:?}")]
    ParserInternal(rustemo::Error),
    #[error("Parser error: {0}")]
    Parser(String),
    #[error("Lexer error: {0}")]
    Lexer(String),
    #[error("Context error: {0}")]
    Context(String),
    #[error("IO error: {0}")]
    IO(String),
}

thread_local! {
    pub static SOURCE_CODE_PATH: RefCell<Option<PathBuf>> = const { RefCell::new(None) };
    pub static LEXER_FILE: RefCell<Option<File>> = const { RefCell::new(None) };
    pub static PARSER_FILE: RefCell<Option<File>> = const { RefCell::new(None) };
    pub static SYMBOL_TABLE_FILE: RefCell<Option<File>> = const { RefCell::new(None) };
}

pub fn set_source_file_path(path: PathBuf) {
    SOURCE_CODE_PATH.set(Some(path));
}

pub fn open_lexer_file() -> Result<(), io::Error> {
    SOURCE_CODE_PATH.with(|f| -> Result<(), io::Error> {
        if let Some(path) = f.borrow().as_ref() {
            LEXER_FILE.set(Some(
                OpenOptions::new()
                    .create(true)
                    .truncate(true)
                    .write(true)
                    .open(path.with_extension("lexer"))?,
            ));
        }
        Ok(())
    })
}

pub fn write_to_lexer_file(line: &str) -> Result<(), io::Error> {
    LEXER_FILE.with(|f| {
        if let Some(mut file) = f.borrow_mut().as_ref() {
            writeln!(file, "{line}")?;
        }
        Ok(())
    })
}

pub fn open_parser_file() -> Result<(), io::Error> {
    SOURCE_CODE_PATH.with(|f| -> Result<(), io::Error> {
        if let Some(path) = f.borrow().as_ref() {
            PARSER_FILE.set(Some(
                OpenOptions::new()
                    .create(true)
                    .truncate(true)
                    .write(true)
                    .read(true)
                    .open(path.with_extension("parser"))?,
            ));
        }
        Ok(())
    })
}

pub fn write_to_parser_file(line: &str) -> Result<(), io::Error> {
    PARSER_FILE.with(|f| {
        if let Some(mut file) = f.borrow_mut().as_ref() {
            writeln!(file, "{line}")?;
        }
        Ok(())
    })
}

pub fn open_symbol_table_file() -> Result<(), io::Error> {
    SOURCE_CODE_PATH.with(|f| -> Result<(), io::Error> {
        if let Some(path) = f.borrow().as_ref() {
            SYMBOL_TABLE_FILE.set(Some(
                OpenOptions::new()
                    .create(true)
                    .truncate(true)
                    .write(true)
                    .open(path.with_extension("symbol_table"))?,
            ));
        }
        Ok(())
    })
}

pub fn write_to_symbol_table_file(line: &str) -> Result<(), io::Error> {
    SYMBOL_TABLE_FILE.with(|f| {
        if let Some(mut file) = f.borrow_mut().as_ref() {
            writeln!(file, "{line}")?;
        }
        Ok(())
    })
}

pub fn read_source_to_string() -> Result<String, CompilerError> {
    SOURCE_CODE_PATH.with(|f| {
        if let Some(path) = f.borrow().as_ref() {
            read_to_string(path)
                .map_err(|e| CompilerError::IO(format!("Failed to read input file: {e}")))
        } else {
            Err(CompilerError::Context(
                "Tried to open source code file without setting the path".into(),
            ))
        }
    })
}

pub fn read_parser_file_to_string() -> Result<String, CompilerError> {
    PARSER_FILE.with(|f| {
        let mut buf = String::new();
        if let Some(mut file) = f.borrow().as_ref() {
            file.rewind()
                .map_err(|e| CompilerError::IO(format!("Failed to rewind parser file: {e:?}")))?;
            file.read_to_string(&mut buf).map_err(|e| {
                CompilerError::IO(format!("Failed to read parser file to string: {e:?}"))
            })?;
            Ok(buf)
        } else {
            Err(CompilerError::Context(
                "Tried to open parser file before creating it".into(),
            ))
        }
    })
}
