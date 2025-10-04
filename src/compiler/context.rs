use crate::{
    compiler::error::CompilerError,
    grammar::types::{DataType, TokenFloatLiteral, TokenIntLiteral, TokenStringLiteral},
};
use std::{
    cell::RefCell,
    fmt::Display,
    fs::{File, OpenOptions, read_to_string},
    io::{self, Read, Seek, Write},
    path::PathBuf,
};

thread_local! {
    pub static SOURCE_CODE_PATH: RefCell<Option<PathBuf>> = const { RefCell::new(None) };
    pub static LEXER_FILE: RefCell<Option<File>> = const { RefCell::new(None) };
    pub static PARSER_FILE: RefCell<Option<File>> = const { RefCell::new(None) };
    pub static SYMBOL_TABLE_FILE: RefCell<Option<File>> = const { RefCell::new(None) };
    pub static SYMBOL_TABLE: RefCell<Vec<SymbolTableElement>> = const { RefCell::new(Vec::new())}
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

pub fn write_to_lexer_file(line: &str) {
    LEXER_FILE.with(|f| {
        if let Some(mut file) = f.borrow_mut().as_ref()
            && let Err(e) = writeln!(file, "{line}")
        {
            eprintln!("Error while writing to lexer file: {e}");
            std::process::exit(1)
        }
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

pub fn write_to_parser_file(line: &str) {
    PARSER_FILE.with(|f| {
        if let Some(mut file) = f.borrow_mut().as_ref()
            && let Err(e) = writeln!(file, "{line}")
        {
            eprintln!("Error while writing to parser file: {e}");
            std::process::exit(1)
        }
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

pub fn dump_symbol_table_to_file() {
    SYMBOL_TABLE_FILE.with(|f| {
        if let Some(mut file) = f.borrow_mut().as_ref() {
            for symbol in SYMBOL_TABLE.take() {
                if let Err(e) = writeln!(file, "{symbol}") {
                    eprintln!("Error while dumping symbol table {e}");
                    std::process::exit(1)
                }
            }
        }
    })
}

pub fn read_source_to_string() -> String {
    SOURCE_CODE_PATH.with(|f| {
        if let Some(path) = f.borrow().as_ref() {
            match read_to_string(path)
                .map_err(|e| CompilerError::IO(format!("Failed to read input file: {e}")))
            {
                Err(e) => {
                    eprintln!("Error: {e}");
                    std::process::exit(1)
                }
                Ok(source) => source,
            }
        } else {
            eprintln!("Error: Tried to open source code file without setting the path");
            std::process::exit(1)
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

#[derive(Clone, Debug)]
pub enum SymbolTableElement {
    VarDeclaration(String, DataType, usize),
    IntLiteral(TokenIntLiteral),
    FloatLiteral(TokenFloatLiteral),
    StringLiteral(TokenStringLiteral),
}

impl Display for SymbolTableElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FloatLiteral(float) => write!(
                f,
                "{}|CONST_FLOAT|{}|{}",
                float.original,
                float.original,
                float.original.len()
            )?,
            Self::IntLiteral(int) => {
                write!(f, "{}|CONST_INT|{}|{}", int, int, int.to_string().len())?
            }
            Self::StringLiteral(string) => {
                write!(f, "{}|CONST_STRING|{}|{}", string, string, string.len())?
            }
            Self::VarDeclaration(token, r#type, length) => {
                write!(f, "{}|{}|-|{}", token, r#type, length)?
            }
        };
        Ok(())
    }
}

impl PartialEq for SymbolTableElement {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::FloatLiteral(token0), Self::FloatLiteral(token1)) => token0 == token1,
            (Self::IntLiteral(token0), Self::IntLiteral(token1)) => token0 == token1,
            (Self::StringLiteral(token0), Self::StringLiteral(token1)) => token0 == token1,
            (Self::VarDeclaration(token0, _, _), Self::VarDeclaration(token1, _, _)) => {
                token0 == token1
            }
            _ => false,
        }
    }
}

impl From<TokenIntLiteral> for SymbolTableElement {
    fn from(value: TokenIntLiteral) -> Self {
        Self::IntLiteral(value)
    }
}

impl From<TokenFloatLiteral> for SymbolTableElement {
    fn from(value: TokenFloatLiteral) -> Self {
        Self::FloatLiteral(value)
    }
}

impl From<TokenStringLiteral> for SymbolTableElement {
    fn from(value: TokenStringLiteral) -> Self {
        Self::StringLiteral(value)
    }
}

pub fn push_to_symbol_table(item: SymbolTableElement) {
    SYMBOL_TABLE.with(|table| {
        // Avoid pushing duplicate symbols to the symbol table
        if !symbol_exists(&item) {
            table.borrow_mut().push(item);
        }
    })
}

pub fn symbol_exists(item: &SymbolTableElement) -> bool {
    SYMBOL_TABLE.with(|table| table.borrow().contains(item))
}
