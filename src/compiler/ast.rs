use crate::grammar::types::ComparisonOp;
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
    pub stack_t: Vec<Rc<Node>>,
    pub stack_e: Vec<Rc<Node>>,
    pub comparision_op_stack: Vec<ComparisonOp>,
    pub comparision_expressions_stack: Vec<Rc<Node>>,
    pub boolean_expression_stack: Vec<Rc<Node>>,
    pub if_body_stack: Vec<Rc<Node>>,
    pub conjunction_stack: Vec<Rc<Node>>,
    pub statement_stack: Vec<Rc<Node>>,
}

impl Debug for Ast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:?}", self.stack_t)?;
        writeln!(f, "{:?}", self.stack_e)?;
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
    left_child: Option<Rc<Node>>,
    right_child: Option<Rc<Node>>,
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Node {
    pub fn new_leaf(value: NodeValue) -> Self {
        Self {
            value,
            parent: Cell::new(None),
            left_child: None,
            right_child: None,
        }
    }
}

#[derive(Clone, Debug)]
pub enum NodeValue {
    Action(AstAction),
    Value(String),
}

impl Display for NodeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Value(value) => write!(f, "{value}"),
            Self::Action(action) => write!(f, "{action}"),
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
    Not,
    IsZero,
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
            Self::Not => write!(f, "NOT"),
            Self::IsZero => write!(f, "ISZERO"),
            Self::While => write!(f, "WHILE"),
            Self::Read => write!(f, "READ"),
            Self::Write => write!(f, "WRITE"),
            Self::S => write!(f, "S"),
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
            tree: array::from_fn(|_| Rc::new(Node::new_leaf(NodeValue::Value("".to_string())))),
            stack_e: Vec::new(),
            stack_t: Vec::new(),
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

    pub fn create_leaf(&mut self, value: String, dest_ptr: AstPtr) -> Rc<Node> {
        let leaf = Rc::new(Node::new_leaf(NodeValue::Value(value)));
        self.tree[dest_ptr as usize] = leaf.clone();
        leaf
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
        writeln!(file, "    {node_name:0>3} [label=\"{:}\"] ;", node.value)?;
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

    pub fn push_t_stack(&mut self, node: AstNodeRef) {
        let node = match node {
            AstNodeRef::Node(node) => node,
            AstNodeRef::Ptr(ptr) => self.tree[ptr as usize].clone(),
        };

        self.stack_t.push(node);
    }

    pub fn pop_t_stack(&mut self) -> Option<Rc<Node>> {
        self.stack_t.pop()
    }

    pub fn push_e_stack(&mut self, node: AstNodeRef) {
        let node = match node {
            AstNodeRef::Node(node) => node,
            AstNodeRef::Ptr(ptr) => self.tree[ptr as usize].clone(),
        };

        self.stack_e.push(node);
    }

    pub fn pop_e_stack(&mut self) -> Option<Rc<Node>> {
        self.stack_e.pop()
    }

    pub fn get_node_from_ptr(&self, from: AstPtr) -> Rc<Node> {
        self.tree[from as usize].clone()
    }
}
