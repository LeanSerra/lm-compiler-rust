use crate::{
    compiler::{
        asm::TasmGenerator,
        context::{SymbolTable, SymbolTableElementType},
    },
    grammar::types::{ComparisonOp, DataType},
};
use std::{
    array,
    cell::Cell,
    fmt::{Debug, Display},
    fs::File,
    io::{self, Write},
    mem,
    rc::Rc,
};

pub struct Ast {
    tree: [Rc<Node>; mem::variant_count::<AstPtr>()],
    pub term_stack: Vec<Rc<Node>>,
    pub expression_stack: Vec<Rc<Node>>,
    pub comparision_op_stack: Vec<ComparisonOp>,
    pub comparision_expressions_stack: Vec<Rc<Node>>,
    pub boolean_expression_stack: Vec<Rc<Node>>,
    pub if_body_stack: Vec<Rc<Node>>,
    pub conjunction_stack: Vec<Rc<Node>>,
    pub statement_stack: Vec<Rc<Node>>,
}

impl Debug for Ast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:?}", self.term_stack)?;
        writeln!(f, "{:?}", self.expression_stack)?;
        writeln!(f, "{:?}", self.comparision_op_stack)?;
        writeln!(f, "{:?}", self.comparision_expressions_stack)
    }
}

#[derive(Clone, Copy)]
pub enum AstPtr {
    Program = 0,
    Assignment,
    Number,
    Factor,
    Term,
    ArithmeticExpression,
    BooleanExpression,
    Conjunction,
    SimpleExpression,
    Body,
    Statement,
    Expressions,
    If,
    Not,
    IsZero,
    While,
    Read,
    Write,
    ConvDate,
}

pub enum AstNodeRef {
    Ptr(AstPtr),
    Node(Rc<Node>),
}

impl From<AstPtr> for AstNodeRef {
    fn from(value: AstPtr) -> Self {
        Self::Ptr(value)
    }
}

impl From<Rc<Node>> for AstNodeRef {
    fn from(value: Rc<Node>) -> Self {
        Self::Node(value)
    }
}

pub struct Node {
    pub value: NodeValue,
    parent: Cell<Option<Rc<Node>>>,
    pub left_child: Option<Rc<Node>>,
    pub right_child: Option<Rc<Node>>,
    pub r#type: Option<ExpressionType>,
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Node {
    pub fn new_leaf(value: NodeValue, node_type: Option<ExpressionType>) -> Self {
        Self {
            value,
            parent: Cell::new(None),
            left_child: None,
            right_child: None,
            r#type: node_type,
        }
    }
}

#[derive(Clone, Debug)]
pub enum NodeValue {
    Action(AstAction),
    Value(String),
    True,
    False,
}

impl Display for NodeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Value(value) => write!(f, "{value}"),
            Self::Action(action) => write!(f, "{action}"),
            Self::True => write!(f, "True"),
            Self::False => write!(f, "False"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExpressionType {
    Float,
    Int,
    String,
}

impl Display for ExpressionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Float => write!(f, "FLOAT"),
            Self::Int => write!(f, "INT"),
            Self::String => write!(f, "STRING"),
        }
    }
}

impl From<DataType> for ExpressionType {
    fn from(value: DataType) -> Self {
        match value {
            DataType::FloatType(_) => ExpressionType::Float,
            DataType::IntType(_) => ExpressionType::Int,
            DataType::StringType(_) => ExpressionType::String,
        }
    }
}

impl From<SymbolTableElementType> for ExpressionType {
    fn from(value: SymbolTableElementType) -> Self {
        match value {
            SymbolTableElementType::DataType(t) => t.into(),
            SymbolTableElementType::Float => Self::Float,
            SymbolTableElementType::Int => Self::Int,
            SymbolTableElementType::String => Self::String,
        }
    }
}

#[derive(Clone, Debug)]
pub enum AstAction {
    Plus,
    Sub,
    Mult,
    Div,
    Assign,
    If,
    Else,
    And,
    Or,
    GT,
    GTE,
    EQ,
    NE,
    LT,
    LTE,
    While,
    Read,
    Write,
    S,
    Negative,
    Noop,
}

impl Display for AstAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Plus => write!(f, "+"),
            Self::Sub => write!(f, "-"),
            Self::Mult => write!(f, "*"),
            Self::Div => write!(f, "/"),
            Self::Assign => write!(f, ":="),
            Self::GT => write!(f, ">"),
            Self::GTE => write!(f, ">="),
            Self::EQ => write!(f, "=="),
            Self::NE => write!(f, "!="),
            Self::LT => write!(f, "<"),
            Self::LTE => write!(f, "<="),
            Self::If => write!(f, "IF"),
            Self::Else => write!(f, "ELSE"),
            Self::And => write!(f, "AND"),
            Self::Or => write!(f, "OR"),
            Self::While => write!(f, "WHILE"),
            Self::Read => write!(f, "READ"),
            Self::Write => write!(f, "WRITE"),
            Self::S => write!(f, "S"),
            Self::Negative => write!(f, "NEG"),
            Self::Noop => write!(f, "NOOP"),
        }
    }
}

impl From<ComparisonOp> for AstAction {
    fn from(value: ComparisonOp) -> Self {
        match value {
            ComparisonOp::ComparisonOpEqual(_) => Self::EQ,
            ComparisonOp::ComparisonOpNotEqual(_) => Self::NE,
            ComparisonOp::ComparisonOpLess(_) => Self::LT,
            ComparisonOp::ComparisonOpLessEqual(_) => Self::LTE,
            ComparisonOp::ComparisonOpGreater(_) => Self::GT,
            ComparisonOp::ComparisonOpGreaterEqual(_) => Self::GTE,
        }
    }
}

impl Default for Ast {
    fn default() -> Self {
        Self {
            tree: array::from_fn(|_| {
                Rc::new(Node::new_leaf(NodeValue::Value("".to_string()), None))
            }),
            expression_stack: Vec::new(),
            term_stack: Vec::new(),
            comparision_op_stack: Vec::new(),
            comparision_expressions_stack: Vec::new(),
            boolean_expression_stack: Vec::new(),
            if_body_stack: Vec::new(),
            conjunction_stack: Vec::new(),
            statement_stack: Vec::new(),
        }
    }
}

impl Ast {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn assign_node_to_ptr(&mut self, source: AstNodeRef, dest_ptr: AstPtr) {
        let node = match source {
            AstNodeRef::Node(node) => node,
            AstNodeRef::Ptr(ptr) => self.tree[ptr as usize].clone(),
        };

        self.tree[dest_ptr as usize] = node;
    }

    pub fn create_node(
        &mut self,
        action: AstAction,
        left_child_ptr: AstNodeRef,
        right_child_ptr: AstNodeRef,
        dest_ptr: AstPtr,
        r#type: Option<ExpressionType>,
    ) -> Rc<Node> {
        let left_child = match left_child_ptr {
            AstNodeRef::Ptr(ptr) => self.tree.get(ptr as usize).cloned(),
            AstNodeRef::Node(node) => Some(node),
        };
        let right_child = match right_child_ptr {
            AstNodeRef::Ptr(ptr) => self.tree.get(ptr as usize).cloned(),
            AstNodeRef::Node(node) => Some(node),
        };

        let node = Rc::new(Node {
            value: NodeValue::Action(action),
            parent: Cell::new(None),
            left_child: left_child.clone(),
            right_child: right_child.clone(),
            r#type,
        });

        if let Some(left) = left_child {
            left.parent.replace(Some(node.clone()));
        }
        if let Some(right) = right_child {
            right.parent.replace(Some(node.clone()));
        }

        self.tree[dest_ptr as usize] = node.clone();
        node
    }

    pub fn create_leaf(
        &mut self,
        value: String,
        dest_ptr: AstPtr,
        node_type: Option<ExpressionType>,
    ) -> Rc<Node> {
        let leaf = Rc::new(Node::new_leaf(NodeValue::Value(value), node_type));
        self.tree[dest_ptr as usize] = leaf.clone();
        leaf
    }

    pub fn get_node_from_ptr(&self, from: AstPtr) -> Rc<Node> {
        self.tree[from as usize].clone()
    }

    pub fn graph_ast(
        &self,
        from: AstPtr,
        graph_label: &str,
        file: &mut File,
    ) -> Result<(), io::Error> {
        writeln!(file, "graph \"\"")?;
        writeln!(file, "{{")?;
        writeln!(file, "    fontname=\"Arial\"")?;
        writeln!(file, "    node [fontname=\"Arial\"]")?;
        writeln!(file, "    edge [fontname=\"Arial\"]")?;
        writeln!(file, "    label=\"{}\"", graph_label.trim())?;
        writeln!(file)?;
        Ast::graph_recursive_traverse(&self.tree[from as usize], 0, file)?;
        writeln!(file, "}}")
    }

    fn graph_recursive_traverse(
        node: &Rc<Node>,
        mut node_count: usize,
        file: &mut File,
    ) -> Result<usize, io::Error> {
        let node_name = format!("n{node_count:0>3}");
        writeln!(file, "    {node_name:0>3} ;")?;
        writeln!(
            file,
            "    {node_name:0>3} [label=\"{}{}\"] ;",
            node.value,
            node.r#type
                .as_ref()
                .map(|t| format!(" | {t}"))
                .unwrap_or_default()
        )?;
        if let Some(left_child) = &node.left_child {
            node_count += 1;
            writeln!(file, "    {node_name} -- n{node_count:0>3} ;")?;
            node_count = Ast::graph_recursive_traverse(left_child, node_count, file)?;
        }

        if let Some(right_child) = &node.right_child {
            node_count += 1;
            writeln!(file, "    {node_name} -- n{node_count:0>3} ;")?;
            node_count = Ast::graph_recursive_traverse(right_child, node_count, file)?;
        }
        Ok(node_count)
    }

    pub fn generate_asm(
        &self,
        file: &mut File,
        symbol_table: &mut SymbolTable,
    ) -> Result<(), io::Error> {
        let node = self.get_node_from_ptr(AstPtr::Program);
        TasmGenerator::new(symbol_table, file).generate_asm(node)
    }
}
