use crate::{
    compiler::error::CompilerError,
    grammar::{
        rules_builder::Symbol,
        types::{DataType, TokenFloatLiteral, TokenIntLiteral, TokenStringLiteral},
    },
};
use std::{
    cell::{OnceCell, RefCell},
    fmt::Display,
    fs::{File, OpenOptions, read_to_string},
    io::{Read, Seek, Write},
    path::PathBuf,
    rc::Rc,
};

#[derive(Clone, Default)]
pub struct Compiler {
    pub inner: Rc<RefCell<CompilerContext>>,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            inner: Rc::new(RefCell::new(CompilerContext::new())),
        }
    }

    pub fn source(&self) -> String {
        self.inner.borrow().source().clone()
    }
}

#[derive(Default)]
pub struct CompilerContext {
    pub res_stack: Vec<Symbol>,
    source_code_path: Option<PathBuf>,
    source_code: OnceCell<String>,
    symbol_table: Vec<SymbolTableElement>,
    parser_file: Option<File>,
    lexer_file: Option<File>,
    symbol_table_file: Option<File>,
}
impl CompilerContext {
    pub const fn new() -> Self {
        Self {
            res_stack: Vec::new(),
            source_code_path: None,
            source_code: OnceCell::new(),
            symbol_table: Vec::new(),
            parser_file: None,
            lexer_file: None,
            symbol_table_file: None,
        }
    }
}

impl CompilerContext {
    pub fn init_compiler_context(&mut self, path: PathBuf) -> Result<(), CompilerError> {
        self.source_code_path = Some(path);
        self.source_code.set(self.read_source_to_string()?).unwrap();
        self.open_parser_file()?;
        self.open_lexer_file()?;
        self.open_symbol_table_file()?;

        Ok(())
    }

    fn read_source_to_string(&self) -> Result<String, CompilerError> {
        read_to_string(self.source_code_path.as_ref().unwrap())
            .map_err(|e| CompilerError::IO(format!("Failed to read input file: {e}")))
    }

    fn open_lexer_file(&mut self) -> Result<(), CompilerError> {
        self.lexer_file = Some(
            OpenOptions::new()
                .create(true)
                .truncate(true)
                .write(true)
                .open(
                    self.source_code_path
                        .as_ref()
                        .unwrap()
                        .with_extension("lexer"),
                )
                .map_err(|e| CompilerError::IO(e.to_string()))?,
        );
        Ok(())
    }

    fn open_parser_file(&mut self) -> Result<(), CompilerError> {
        self.parser_file = Some(
            OpenOptions::new()
                .create(true)
                .truncate(true)
                .read(true)
                .write(true)
                .open(
                    self.source_code_path
                        .as_ref()
                        .unwrap()
                        .with_extension("parser"),
                )
                .map_err(|e| CompilerError::IO(e.to_string()))?,
        );
        Ok(())
    }

    fn open_symbol_table_file(&mut self) -> Result<(), CompilerError> {
        self.symbol_table_file = Some(
            OpenOptions::new()
                .create(true)
                .truncate(true)
                .write(true)
                .open(
                    self.source_code_path
                        .as_ref()
                        .unwrap()
                        .with_extension("symbol_table"),
                )
                .map_err(|e| CompilerError::IO(e.to_string()))?,
        );
        Ok(())
    }

    pub fn path(&self) -> String {
        self.source_code_path
            .as_ref()
            .unwrap()
            .to_string_lossy()
            .into()
    }

    pub fn source(&self) -> &String {
        self.source_code
            .get_or_init(|| self.read_source_to_string().unwrap())
    }

    pub fn dump_symbol_table_to_file(&mut self) -> Result<(), CompilerError> {
        let Some(symbol_table_file) = self.symbol_table_file.as_mut() else {
            return Err(CompilerError::Context(
                "Tried to dump symbol table to file before opening it".into(),
            ));
        };
        for symbol in self.symbol_table.iter() {
            writeln!(symbol_table_file, "{symbol}")
                .map_err(|e| CompilerError::IO(e.to_string()))?;
        }
        Ok(())
    }

    pub fn write_to_lexer_file(&mut self, line: &str) {
        let Some(lexer_file) = self.lexer_file.as_mut() else {
            eprintln!("Error: tried to write to lexer file before opening it");
            std::process::exit(1)
        };
        if let Err(e) = writeln!(lexer_file, "{line}") {
            eprintln!("IO error: {e}");
            std::process::exit(1)
        };
    }

    pub fn write_to_parser_file(&mut self, line: &str) {
        let Some(parser_file) = self.parser_file.as_mut() else {
            eprintln!("Error: tried to write to lexer file before opening it");
            std::process::exit(1)
        };
        if let Err(e) = writeln!(parser_file, "{line}") {
            eprintln!("IO error: {e}");
            std::process::exit(1)
        };
    }

    pub fn read_parser_file_to_string(&mut self) -> Result<String, CompilerError> {
        let mut buf = String::new();
        let file = self
            .parser_file
            .as_mut()
            .ok_or(CompilerError::IO("Parser file is not set".into()))?;
        file.rewind()
            .map_err(|e| CompilerError::IO(format!("Failed to rewind parser file: {e:?}")))?;
        file.read_to_string(&mut buf).map_err(|e| {
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
