use crate::{
    compiler::{ast::Ast, error::CompilerError},
    grammar::{
        rules_builder::Symbol,
        types::{DataType, TokenFloatLiteral, TokenIntLiteral, TokenStringLiteral},
    },
};
use std::{
    cell::RefCell,
    fmt::Display,
    fs::{File, OpenOptions, read_to_string},
    io::{Read, Seek, Write},
    path::{Path, PathBuf},
    rc::Rc,
};

#[derive(Clone)]
pub struct Compiler {
    pub inner: Rc<RefCell<CompilerContext>>,
}

impl Compiler {
    pub fn new(path: PathBuf) -> Result<Self, CompilerError> {
        Ok(Self {
            inner: Rc::new(RefCell::new(CompilerContext::new(path)?)),
        })
    }
}

pub struct CompilerContext {
    pub res_stack: Vec<Symbol>,
    source_code_path: PathBuf,
    source_code: String,
    symbol_table: Vec<SymbolTableElement>,
    parser_file: File,
    lexer_file: File,
    symbol_table_file: File,
    pub ast: Ast,
}

impl CompilerContext {
    pub fn new(path: PathBuf) -> Result<Self, CompilerError> {
        let source_code = CompilerContext::read_source_to_string(&path)?;
        let parser_file = CompilerContext::open_parser_file(&path)?;
        let lexer_file = CompilerContext::open_lexer_file(&path)?;
        let symbol_table_file = CompilerContext::open_symbol_table_file(&path)?;

        Ok(Self {
            res_stack: Vec::new(),
            source_code_path: path,
            source_code,
            symbol_table: Vec::new(),
            parser_file,
            lexer_file,
            symbol_table_file,
            ast: Ast::new(),
        })
    }

    fn read_source_to_string(path: &PathBuf) -> Result<String, CompilerError> {
        read_to_string(path)
            .map_err(|e| CompilerError::IO(format!("Failed to read input file: {e}")))
    }

    fn open_lexer_file(path: &Path) -> Result<File, CompilerError> {
        OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(path.with_extension("lexer"))
            .map_err(|e| CompilerError::IO(e.to_string()))
    }

    fn open_parser_file(path: &Path) -> Result<File, CompilerError> {
        OpenOptions::new()
            .create(true)
            .truncate(true)
            .read(true)
            .write(true)
            .open(path.with_extension("parser"))
            .map_err(|e| CompilerError::IO(e.to_string()))
    }

    fn open_symbol_table_file(path: &Path) -> Result<File, CompilerError> {
        OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(path.with_extension("symbol_table"))
            .map_err(|e| CompilerError::IO(e.to_string()))
    }

    pub fn path(&self) -> String {
        self.source_code_path.to_string_lossy().into()
    }

    pub fn source(&self) -> &String {
        &self.source_code
    }

    pub fn dump_symbol_table_to_file(&mut self) -> Result<(), CompilerError> {
        for symbol in self.symbol_table.iter() {
            writeln!(self.symbol_table_file, "{symbol}")
                .map_err(|e| CompilerError::IO(e.to_string()))?;
        }
        Ok(())
    }

    pub fn write_to_lexer_file(&mut self, line: &str) {
        if let Err(e) = writeln!(self.lexer_file, "{line}") {
            eprintln!("IO error: {e}");
            std::process::exit(1)
        };
    }

    pub fn write_to_parser_file(&mut self, line: &str) {
        if let Err(e) = writeln!(self.parser_file, "{line}") {
            eprintln!("IO error: {e}");
            std::process::exit(1)
        };
    }

    pub fn read_parser_file_to_string(&mut self) -> Result<String, CompilerError> {
        let mut buf = String::new();
        self.parser_file
            .rewind()
            .map_err(|e| CompilerError::IO(format!("Failed to rewind parser file: {e:?}")))?;
        self.parser_file.read_to_string(&mut buf).map_err(|e| {
            CompilerError::IO(format!("Failed to read parser file to string: {e:?}"))
        })?;
        Ok(buf)
    }

    pub fn push_to_symbol_table(&mut self, symbol: SymbolTableElement) {
        if !self.symbol_exists(&symbol) {
            self.symbol_table.push(symbol);
        }
    }

    pub fn symbol_exists(&self, symbol: &SymbolTableElement) -> bool {
        self.symbol_table.contains(symbol)
    }
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
