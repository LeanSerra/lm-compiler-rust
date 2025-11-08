use std::{
    fs::File,
    io::{self, Write},
    rc::Rc,
};

use crate::compiler::{
    ast::{AstAction, Node, NodeValue},
    context::SymbolTable,
};

pub struct TasmGenerator<'a> {
    label_if_false_count: usize,
    label_if_body_count: usize,
    symbol_table: &'a mut SymbolTable,
    file: &'a mut File,
}

impl<'a> TasmGenerator<'a> {
    pub fn new(symbol_table: &'a mut SymbolTable, file: &'a mut File) -> Self {
        Self {
            file,
            symbol_table,
            label_if_false_count: 0,
            label_if_body_count: 0,
        }
    }

    pub fn generate_asm(mut self, root: Rc<Node>) -> Result<(), io::Error> {
        // Header
        self.generate_asm_header()?;
        // .DATA
        self.symbol_table.to_data(self.file)?;
        // .PROGRAM header
        self.generate_code_prologue()?;
        // Program
        self.generate_asm_from_tree(&root)?;
        // END Program
        self.generate_code_epilogue()
    }

    fn generate_asm_header(&mut self) -> Result<(), io::Error> {
        let file = &mut self.file;
        writeln!(file, ".MODEL LARGE")?;
        writeln!(file, ".386")?;
        writeln!(file, ".STACK 200h")
    }

    fn generate_code_prologue(&mut self) -> Result<(), io::Error> {
        let file = &mut self.file;
        writeln!(file)?;
        writeln!(file, ".CODE")?;
        writeln!(file)?;
        writeln!(file, "Program:")?;

        writeln!(file, "    MOV AX, @DATA")?;
        writeln!(file, "    MOV DS, AX")?;
        writeln!(file, "    MOV ES, AX")?;
        writeln!(file)
    }

    fn generate_code_epilogue(&mut self) -> Result<(), io::Error> {
        let file = &mut self.file;
        writeln!(file, "    MOV AX, 4C00H")?;
        writeln!(file, "    INT 21H")?;
        writeln!(file, "    END Program")
    }

    fn generate_asm_from_tree(&mut self, node: &Rc<Node>) -> Result<(), io::Error> {
        match &node.value {
            NodeValue::Value(val) => self.generate_node_value_value(val),
            NodeValue::True => Ok(()),
            NodeValue::False => Ok(()),
            NodeValue::Action(action) => match action {
                AstAction::S => {
                    self.generate_asm_from_tree(node.left_child.as_ref().unwrap())?;
                    self.generate_asm_from_tree(node.right_child.as_ref().unwrap())
                }
                AstAction::Assign => self.generate_action_assign(node),
                AstAction::GT
                | AstAction::GTE
                | AstAction::EQ
                | AstAction::NE
                | AstAction::LT
                | AstAction::LTE => self.generate_action_cmp(node),
                AstAction::Plus => {
                    self.generate_asm_from_tree(node.left_child.as_ref().unwrap())?;
                    self.generate_asm_from_tree(node.right_child.as_ref().unwrap())?;
                    writeln!(self.file, "    FADD")
                }
                AstAction::Div => {
                    self.generate_asm_from_tree(node.left_child.as_ref().unwrap())?;
                    self.generate_asm_from_tree(node.right_child.as_ref().unwrap())?;
                    writeln!(self.file, "    FDIV")
                }
                AstAction::Sub => {
                    self.generate_asm_from_tree(node.left_child.as_ref().unwrap())?;
                    self.generate_asm_from_tree(node.right_child.as_ref().unwrap())?;
                    writeln!(self.file, "    FSUB")
                }
                AstAction::Mult => {
                    self.generate_asm_from_tree(node.left_child.as_ref().unwrap())?;
                    self.generate_asm_from_tree(node.right_child.as_ref().unwrap())?;
                    writeln!(self.file, "    FMUL")
                }

                AstAction::If => self.generate_action_if(node),
                AstAction::Else => {
                    todo!()
                }
                AstAction::And => self.generate_action_and(node),
                AstAction::Or => self.generate_action_or(node),
                AstAction::Not => {
                    todo!()
                }
                AstAction::While => {
                    todo!()
                }
                AstAction::Read => {
                    todo!()
                }
                AstAction::Write => self.generate_action_write(node),
                AstAction::Negative => self.generate_action_negative(node),
                AstAction::Noop => {
                    panic!("tried to execute noop")
                }
            },
        }
    }

    fn generate_node_value_value(&mut self, val: &str) -> Result<(), io::Error> {
        let val = self.symbol_table.get_symbol_asm_name(val).unwrap();
        writeln!(self.file, "    FLD     {val}")
    }

    fn generate_action_assign(&mut self, node: &Rc<Node>) -> Result<(), io::Error> {
        self.generate_asm_from_tree(node.right_child.as_ref().unwrap())?;
        let NodeValue::Value(lhs) = &node.left_child.as_ref().unwrap().value else {
            panic!("invalid assign")
        };

        let lhs = self.symbol_table.get_symbol_asm_name(lhs).unwrap();

        writeln!(self.file, "    FST   {lhs}")?;
        writeln!(self.file, "    FFREE")?;
        writeln!(self.file)
    }

    fn generate_action_cmp(&mut self, node: &Rc<Node>) -> Result<(), io::Error> {
        self.generate_asm_from_tree(node.left_child.as_ref().unwrap())?;
        writeln!(self.file, "    FST     _@l_cond")?;
        writeln!(self.file)?;
        self.generate_asm_from_tree(node.right_child.as_ref().unwrap())?;
        writeln!(self.file, "    FST     _@r_cond")?;
        writeln!(self.file)?;
        writeln!(self.file, "    FLD     _@l_cond")?;
        writeln!(self.file, "    FCOMP   _@r_cond")?;
        writeln!(self.file, "    FSTSW   AX")?;
        writeln!(self.file, "    SAHF")?;
        writeln!(self.file, "    FFREE")?;
        writeln!(self.file)
    }

    fn generate_action_if(&mut self, node: &Rc<Node>) -> Result<(), io::Error> {
        let Some(left_child) = &node.left_child else {
            panic!("invalid if")
        };
        self.generate_asm_from_tree(left_child)?;

        let label_if_false = format!("if_false_{}", self.label_if_false_count);
        self.label_if_false_count += 1;
        // Create jump to label if false depending on operator
        match &left_child.value {
            NodeValue::Value(_val) => todo!("handle if(a)"),
            NodeValue::True => todo!("handle if(True)"),
            NodeValue::False => todo!("handle if(False)"),
            NodeValue::Action(action) => match action {
                AstAction::GT => {
                    writeln!(self.file, "    JNAE    {label_if_false}")?;
                    writeln!(self.file)?;
                }
                AstAction::GTE => {
                    writeln!(self.file, "    JNA    {label_if_false}")?;
                    writeln!(self.file)?;
                }
                AstAction::EQ => {
                    writeln!(self.file, "    JNE    {label_if_false}")?;
                    writeln!(self.file)?;
                }
                AstAction::NE => {
                    writeln!(self.file, "    JE    {label_if_false}")?;
                    writeln!(self.file)?;
                }
                AstAction::LT => {
                    writeln!(self.file, "    JAE    {label_if_false}")?;
                    writeln!(self.file)?;
                }
                AstAction::LTE => {
                    writeln!(self.file, "    JA    {label_if_false}")?;
                    writeln!(self.file)?;
                }
                AstAction::And => {}
                AstAction::Or => {
                    // TODO should we generate this label every time?
                    let label_if_body = format!("if_body_{}", self.label_if_body_count);
                    self.label_if_body_count += 1;
                    // Generate label to jump if any of the OR statements are true
                    writeln!(self.file, "{label_if_body}:")?;
                }
                _ => panic!("invalid"),
            },
        };
        // Generate If body
        self.generate_asm_from_tree(node.right_child.as_ref().unwrap())?;
        // Label to jump if statement is false
        writeln!(self.file, "{label_if_false}:")?;

        Ok(())
    }

    fn generate_action_and(&mut self, node: &Rc<Node>) -> Result<(), io::Error> {
        let label_if_false = format!("if_false_{}", self.label_if_false_count);
        let Some(left_child) = node.left_child.as_ref() else {
            panic!("invalid AND");
        };
        // Traverse the left subtree generating the comparison
        self.generate_asm_from_tree(left_child)?;
        // If the generated left side is false then jump to the end of the if
        match &left_child.value {
            NodeValue::True => {
                todo!("handle AND True")
            }
            NodeValue::False => {
                todo!("handle AND False")
            }
            NodeValue::Value(_val) => {
                todo!("handle AND (a)")
            }
            NodeValue::Action(action) => match action {
                AstAction::GT => {
                    writeln!(self.file, "    JNAE    {label_if_false}")?;
                    writeln!(self.file)?;
                }
                AstAction::GTE => {
                    writeln!(self.file, "    JNA    {label_if_false}")?;
                    writeln!(self.file)?;
                }
                AstAction::EQ => {
                    writeln!(self.file, "    JNE    {label_if_false}")?;
                    writeln!(self.file)?;
                }
                AstAction::NE => {
                    writeln!(self.file, "    JE    {label_if_false}")?;
                    writeln!(self.file)?;
                }
                AstAction::LT => {
                    writeln!(self.file, "    JAE    {label_if_false}")?;
                    writeln!(self.file)?;
                }
                AstAction::LTE => {
                    writeln!(self.file, "    JA    {label_if_false}")?;
                    writeln!(self.file)?;
                }
                AstAction::Or => {
                    // TODO maybe we need this
                    // writeln!(self.file, "    JMP    {label_if_false}")?;
                }
                AstAction::And => {}
                _ => panic!("invalid"),
            },
        }
        let Some(right_child) = node.right_child.as_ref() else {
            panic!("invalid and");
        };
        self.generate_asm_from_tree(right_child)?;
        // If the generated right side is false then jump to the end of the if
        match &right_child.value {
            NodeValue::True => {
                todo!("handle AND True")
            }
            NodeValue::False => {
                todo!("handle AND False")
            }
            NodeValue::Value(_val) => {
                todo!("handle AND (a)")
            }
            NodeValue::Action(action) => match action {
                AstAction::GT => {
                    writeln!(self.file, "    JNAE    {label_if_false}")?;
                    writeln!(self.file)?;
                }
                AstAction::GTE => {
                    writeln!(self.file, "    JNA    {label_if_false}")?;
                    writeln!(self.file)?;
                }
                AstAction::EQ => {
                    writeln!(self.file, "    JNE    {label_if_false}")?;
                    writeln!(self.file)?;
                }
                AstAction::NE => {
                    writeln!(self.file, "    JE    {label_if_false}")?;
                    writeln!(self.file)?;
                }
                AstAction::LT => {
                    writeln!(self.file, "    JAE    {label_if_false}")?;
                    writeln!(self.file)?;
                }
                AstAction::LTE => {
                    writeln!(self.file, "    JA    {label_if_false}")?;
                    writeln!(self.file)?;
                }
                AstAction::And | AstAction::Or => {}
                _ => panic!("invalid"),
            },
        }
        Ok(())
    }

    fn generate_action_or(&mut self, node: &Rc<Node>) -> Result<(), io::Error> {
        let label_if_body = format!("if_body_{}", self.label_if_body_count);
        let label_if_false = format!("if_false_{}", self.label_if_false_count);
        let Some(left_child) = node.left_child.as_ref() else {
            panic!("invalid OR");
        };
        // Traverse the left subtree generating the comparison
        self.generate_asm_from_tree(left_child)?;
        // If either the left side or the right side are true we jump to the if body
        match &left_child.value {
            NodeValue::True => {
                todo!("handle OR True")
            }
            NodeValue::False => {
                todo!("handle OR False")
            }
            NodeValue::Value(_val) => {
                todo!("handle OR (a)")
            }
            NodeValue::Action(action) => match action {
                AstAction::GT => {
                    writeln!(self.file, "    JA    {label_if_body}")?;
                    writeln!(self.file)?;
                }
                AstAction::GTE => {
                    writeln!(self.file, "    JAE    {label_if_body}")?;
                    writeln!(self.file)?;
                }
                AstAction::EQ => {
                    writeln!(self.file, "    JE    {label_if_body}")?;
                    writeln!(self.file)?;
                }
                AstAction::NE => {
                    writeln!(self.file, "    JNE    {label_if_body}")?;
                    writeln!(self.file)?;
                }
                AstAction::LT => {
                    writeln!(self.file, "    JNA    {label_if_body}")?;
                    writeln!(self.file)?;
                }
                AstAction::LTE => {
                    writeln!(self.file, "    JNAE    {label_if_body}")?;
                    writeln!(self.file)?;
                }
                AstAction::And | AstAction::Or => {}
                _ => panic!("invalid"),
            },
        }

        let Some(right_child) = node.right_child.as_ref() else {
            panic!("invalid and");
        };
        self.generate_asm_from_tree(right_child)?;
        match &right_child.value {
            NodeValue::True => {
                todo!("handle AND True")
            }
            NodeValue::False => {
                todo!("handle AND False")
            }
            NodeValue::Value(_val) => {
                todo!("handle AND (a)")
            }
            NodeValue::Action(action) => match action {
                AstAction::GT => {
                    writeln!(self.file, "    JA    {label_if_body}")?;
                    writeln!(self.file)?;
                }
                AstAction::GTE => {
                    writeln!(self.file, "    JAE    {label_if_body}")?;
                    writeln!(self.file)?;
                }
                AstAction::EQ => {
                    writeln!(self.file, "    JE    {label_if_body}")?;
                    writeln!(self.file)?;
                }
                AstAction::NE => {
                    writeln!(self.file, "    JNE    {label_if_body}")?;
                    writeln!(self.file)?;
                }
                AstAction::LT => {
                    writeln!(self.file, "    JNA    {label_if_body}")?;
                    writeln!(self.file)?;
                }
                AstAction::LTE => {
                    writeln!(self.file, "    JNAE    {label_if_body}")?;
                    writeln!(self.file)?;
                }
                AstAction::And | AstAction::Or => {}
                _ => panic!("invalid"),
            },
        }
        writeln!(self.file, "    JMP    {label_if_false}")
    }

    fn generate_action_negative(&mut self, node: &Rc<Node>) -> Result<(), io::Error> {
        let NodeValue::Value(lhs) = &node.left_child.as_ref().unwrap().value else {
            panic!("invalid negative")
        };
        writeln!(self.file, "    FLD    _@1")?;
        writeln!(self.file, "    FMUL   {lhs}")?;
        writeln!(self.file)
    }

    fn generate_action_write(&mut self, node: &Rc<Node>) -> Result<(), io::Error> {
        self.generate_asm_from_tree(node.left_child.as_ref().unwrap())?;

        writeln!(self.file, "    FST _@write")?;
        writeln!(self.file, "    DisplayFloat _@write, 2")?;
        writeln!(self.file)
    }
}
