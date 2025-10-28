use std::{array, cell::Cell, mem, rc::Rc};

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

#[derive(Clone, Debug)]
pub enum AstAction {
    Plus,
    Sub,
    Mult,
    Div,
    Assign,
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

    pub fn traverse_from(&self, from: AstPtr) {
        Ast::recursive_traverse(&self.tree[from as usize], 0);
    }

    fn recursive_traverse(node: &Rc<Node>, depth: usize) {
        if let Some(left_child) = &node.left_child {
            Ast::recursive_traverse(left_child, depth + 1);
        }

        println!("DEPTH: {depth}|{:?}", node.value);

        if let Some(right_child) = &node.right_child {
            Ast::recursive_traverse(right_child, depth + 1);
        }
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
