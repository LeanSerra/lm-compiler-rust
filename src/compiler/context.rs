use crate::{
    compiler::{
        ast::{Ast, AstPtr},
        error::CompilerError,
    },
    grammar::{
        rules_builder::Symbol,
        types::{DataType, TokenFloatLiteral, TokenIntLiteral, TokenStringLiteral},
    },
};
use std::{
    cell::RefCell,
    fmt::Display,
    fs::{File, OpenOptions, read_to_string},
    io::{self, Read, Seek, Write},
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

#[derive(Default)]
pub struct SymbolTable {
    table: Vec<SymbolTableElement>,
}

impl SymbolTable {
    pub fn to_data(&self, file: &mut File) -> Result<(), io::Error> {
        writeln!(file, ".DATA")?;
        for symbol in &self.table {
            if let Some(data_type) = &symbol.data_type {
                let var = match data_type {
                    DataType::FloatType(_) => "dd\t?",
                    DataType::IntType(_) => "dd\t?",
                    DataType::StringType(_) => "db\t'?',\t'$'",
                };

                writeln!(file, "\t{}\t{}", symbol.name, var,)?;
            } else {
                writeln!(
                    file,
                    "\t{}\tdd\t{}",
                    symbol.name,
                    symbol.value.as_ref().unwrap()
                )?
            }
        }
        Ok(())
    }

    pub fn symbol_exists(&self, symbol: &SymbolTableElement) -> bool {
        self.table.contains(symbol)
    }

    pub fn insert(&mut self, symbol: SymbolTableElement) {
        if !self.symbol_exists(&symbol) {
            self.table.push(symbol);
        }
    }

    fn iter(&self) -> impl Iterator<Item = &SymbolTableElement> {
        self.table.iter()
    }

    pub fn get_symbol_asm_name(&self, name: &str) -> Option<String> {
        self.table
            .iter()
            .find(|symbol| symbol.original == name)
            .map(|x| x.name.clone())
    }
}

pub struct CompilerContext {
    pub res_stack: Vec<Symbol>,
    source_code_path: PathBuf,
    source_code: String,
    symbol_table: SymbolTable,
    parser_file: File,
    lexer_file: File,
    symbol_table_file: File,
    graph_file: File,
    asm_file: File,
    pub ast: Ast,
}

impl CompilerContext {
    pub fn new(path: PathBuf) -> Result<Self, CompilerError> {
        let source_code = CompilerContext::read_source_to_string(&path)?;
        let parser_file = CompilerContext::open_parser_file(&path)?;
        let lexer_file = CompilerContext::open_lexer_file(&path)?;
        let symbol_table_file = CompilerContext::open_symbol_table_file(&path)?;
        let graph_file = CompilerContext::open_graph_file(&path)?;
        let asm_file = CompilerContext::open_asm_file(&path)?;

        Ok(Self {
            res_stack: Vec::new(),
            source_code_path: path,
            source_code,
            symbol_table: SymbolTable::default(),
            parser_file,
            lexer_file,
            symbol_table_file,
            graph_file,
            asm_file,
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

    fn open_graph_file(path: &Path) -> Result<File, CompilerError> {
        OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(path.with_extension("dot"))
            .map_err(|e| CompilerError::IO(e.to_string()))
    }

    fn open_asm_file(path: &Path) -> Result<File, CompilerError> {
        OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(path.with_extension("asm"))
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
        self.symbol_table.insert(symbol)
    }

    pub fn symbol_exists(&self, symbol: &SymbolTableElement) -> bool {
        self.symbol_table.symbol_exists(symbol)
    }

    pub fn get_symbol_type(&self, symbol_name: &str) -> Option<Option<DataType>> {
        self.symbol_table
            .iter()
            .find(|x| x.original == symbol_name)
            .map(|x| x.data_type.clone())
    }

    pub fn create_ast_graph(&mut self, from: AstPtr) -> Result<(), CompilerError> {
        self.ast
            .graph_ast(
                from,
                &self.source_code_path.to_string_lossy(),
                &mut self.graph_file,
            )
            .map_err(|e| CompilerError::IO(e.to_string()))
    }

    pub fn generate_asm(&mut self) -> Result<(), CompilerError> {
        self.ast
            .generate_asm(&mut self.asm_file, &mut self.symbol_table)
            .map_err(|e| CompilerError::IO(e.to_string()))
    }
}

#[derive(Default)]
pub struct SymbolTableElement {
    pub name: String,
    pub original: String,
    pub data_type: Option<DataType>,
    pub value: Option<String>,
    pub length: Option<usize>,
}

impl Display for SymbolTableElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = &self.name;
        let data_type = self
            .data_type
            .as_ref()
            .map(|r#type| r#type.to_string())
            .unwrap_or_else(|| String::from("-"));
        let value = self
            .value
            .as_ref()
            .cloned()
            .unwrap_or_else(|| String::from("-"));
        let length = self
            .length
            .map(|length| length.to_string())
            .unwrap_or_else(|| String::from("-"));

        write!(f, "{name}|{data_type}|{value}|{length}")
    }
}

impl PartialEq for SymbolTableElement {
    fn eq(&self, other: &Self) -> bool {
        self.original == other.original
    }
}

impl Eq for SymbolTableElement {}

impl From<TokenIntLiteral> for SymbolTableElement {
    fn from(value: TokenIntLiteral) -> Self {
        let mut name = String::with_capacity(value.original.len() + 1);
        name.push('_');
        name.push_str(&value.original);

        Self {
            name,
            original: value.original.clone(),
            data_type: None,
            value: Some(value.original),
            length: None,
        }
    }
}

impl From<TokenFloatLiteral> for SymbolTableElement {
    fn from(value: TokenFloatLiteral) -> Self {
        let mut name = String::with_capacity(value.original.len() + 1);
        name.push('_');
        name.push_str(&value.original);

        Self {
            name,
            original: value.original.clone(),
            data_type: None,
            value: Some(value.original),
            length: None,
        }
    }
}

impl From<TokenStringLiteral> for SymbolTableElement {
    fn from(value: TokenStringLiteral) -> Self {
        let mut name = String::with_capacity(value.len() + 1);
        name.push('_');
        name.push_str(&value);

        Self {
            name,
            original: value.clone(),
            data_type: None,
            length: Some(value.len()),
            value: Some(value),
        }
    }
}
