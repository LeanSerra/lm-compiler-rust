use std::{
    array,
    cell::Cell,
    fmt::Display,
    fs::File,
    io::{self, Write},
    mem,
    rc::Rc,
};

pub struct Ast {
    tree: [Rc<Node>; mem::variant_count::<AstPtr>()],
    stack_t: Vec<Rc<Node>>,
    stack_e: Vec<Rc<Node>>,
}

#[derive(Clone, Copy)]
pub enum AstPtr {
    Program = 0,
    Assignment,
    Number,
    Factor,
    Term,
    ArithmeticExpression,
    SimpleExpression,
}

pub enum AstNodeRef {
    Ptr(AstPtr),
    Node(Rc<Node>),
}

pub struct Node {
    pub value: NodeValue,
    parent: Cell<Option<Rc<Node>>>,
    left_child: Option<Rc<Node>>,
    right_child: Option<Rc<Node>>,
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
}

impl Display for AstAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Plus => write!(f, "PLUS"),
            Self::Sub => write!(f, "SUB"),
            Self::Mult => write!(f, "MUL"),
            Self::Div => write!(f, "DIV"),
            Self::Assign => write!(f, "ASSIGN"),
        }
    }
}

impl Default for Ast {
    fn default() -> Self {
        Self {
            tree: array::from_fn(|_| Rc::new(Node::new_leaf(NodeValue::Value("".to_string())))),
            stack_e: Vec::new(),
            stack_t: Vec::new(),
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
}
