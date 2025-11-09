use std::{
    fs::File,
    io::{self, Write},
    rc::Rc,
};

use crate::{
    compiler::{
        ast::{AstAction, ExpressionType, Node, NodeValue},
        context::{SymbolTable, SymbolTableElement, SymbolTableElementType},
    },
    grammar::types::DataType,
};

pub struct TasmGenerator<'a> {
    label_if_false_count: usize,
    label_if_body_count: usize,
    label_if_else_body_count: usize,
    label_while_cond_count: usize,
    current_end_label: String,
    current_begin_label: String,
    symbol_table: &'a mut SymbolTable,
    file: &'a mut File,
}

impl<'a> TasmGenerator<'a> {
    pub fn new(symbol_table: &'a mut SymbolTable, file: &'a mut File) -> Self {
        Self {
            file,
            symbol_table,
            current_end_label: String::new(),
            current_begin_label: String::new(),
            label_if_false_count: 0,
            label_if_body_count: 0,
            label_if_else_body_count: 0,
            label_while_cond_count: 0,
        }
    }

    pub fn generate_asm(mut self, root: Rc<Node>) -> Result<(), io::Error> {
        // Header
        self.generate_asm_header()?;
        // Add internal variables to symbol table
        self.add_internal_symbols();
        // .DATA
        self.symbol_table.to_data(self.file)?;
        // .PROGRAM header
        self.generate_code_prologue()?;
        // Program
        self.generate_asm_from_tree(&root)?;
        // END Program
        self.generate_code_epilogue()
    }

    fn add_internal_symbols(&mut self) {
        let neg_one_symbol = SymbolTableElement {
            name: String::from("_@1"),
            value: Some(String::from("-1.0")),
            original: String::from("_@1"),
            data_type: SymbolTableElementType::Float,
            length: None,
        };
        let l_comp_symbol = SymbolTableElement {
            name: String::from("_@l_cond"),
            value: None,
            original: String::from("_@l_cond"),
            data_type: DataType::FloatType("".into()).into(),
            length: None,
        };
        let r_comp_symbol = SymbolTableElement {
            name: String::from("_@r_cond"),
            value: None,
            original: String::from("_@r_cond"),
            data_type: DataType::FloatType("".into()).into(),
            length: None,
        };
        let write_number_symbol = SymbolTableElement {
            name: String::from("_@write_number"),
            value: None,
            original: String::from("_@write_number"),
            data_type: DataType::FloatType("".into()).into(),
            length: None,
        };
        let write_string_symbol = SymbolTableElement {
            name: String::from("_@write_string"),
            value: None,
            original: String::from("_@write_string"),
            data_type: DataType::StringType("".into()).into(),
            length: None,
        };
        for symbol in [
            neg_one_symbol,
            l_comp_symbol,
            r_comp_symbol,
            write_number_symbol,
            write_string_symbol,
        ] {
            self.symbol_table.insert(symbol);
        }
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
                    unreachable!("Tried to execute else branch from generate_asm_from_tree")
                }
                AstAction::And => self.generate_action_and(node),
                AstAction::Or => self.generate_action_or(node),
                AstAction::While => self.generate_action_while(node),
                AstAction::Read => self.generate_action_read(node),
                AstAction::Write => self.generate_action_write(node),
                AstAction::Negative => self.generate_action_negative(node),
                AstAction::Noop => Ok(()),
            },
        }
    }

    fn generate_node_value_value(&mut self, val: &str) -> Result<(), io::Error> {
        let val = self.symbol_table.get_symbol_asm_name(val).unwrap();
        writeln!(self.file, "    FLD     {}", val.name)
    }

    fn generate_action_assign(&mut self, node: &Rc<Node>) -> Result<(), io::Error> {
        self.generate_asm_from_tree(node.right_child.as_ref().unwrap())?;
        let NodeValue::Value(lhs) = &node.left_child.as_ref().unwrap().value else {
            panic!("invalid assign")
        };

        let lhs = self.symbol_table.get_symbol_asm_name(lhs).unwrap().name;

        writeln!(self.file, "    FST    {lhs}")?;
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

        let Some(right_child) = &node.right_child else {
            panic!("invalid if")
        };

        if let NodeValue::Action(AstAction::Else) = right_child.value {
            return self.generate_action_else(right_child, left_child);
        }

        let label_if_false = format!("if_false_{}", self.label_if_false_count);
        self.label_if_false_count += 1;
        self.current_end_label = label_if_false.clone();
        // Create jump to label if false depending on operator
        match &left_child.value {
            NodeValue::Value(_val) => panic!("invalid if"),
            NodeValue::True => { /* */ }
            NodeValue::False => {
                writeln!(self.file, "    JMP    {label_if_false}")?;
                writeln!(self.file)?;
            }
            NodeValue::Action(action) => match action {
                AstAction::GT
                | AstAction::GTE
                | AstAction::EQ
                | AstAction::NE
                | AstAction::LT
                | AstAction::LTE => {
                    let jmp = Self::jmp_to_opposite_asm_jmp(action);
                    writeln!(self.file, "    {jmp}    {label_if_false}")?;
                    writeln!(self.file)?;
                }
                AstAction::And => {}
                AstAction::Or => {
                    // TODO should we generate this label every time?
                    let label_if_body = format!("if_body_{}", self.label_if_body_count);
                    self.label_if_body_count += 1;
                    self.current_end_label = label_if_body.clone();
                    // Generate label to jump if any of the OR statements are true
                    writeln!(self.file, "{label_if_body}:")?;
                }
                _ => panic!("invalid"),
            },
        };
        // Generate If body
        self.generate_asm_from_tree(right_child)?;
        // Label to jump if statement is false
        writeln!(self.file, "{label_if_false}:")?;

        Ok(())
    }

    fn generate_action_and(&mut self, node: &Rc<Node>) -> Result<(), io::Error> {
        let label_jmp_to_end = &self.current_end_label.clone();
        let Some(left_child) = node.left_child.as_ref() else {
            panic!("invalid AND");
        };
        // Traverse the left subtree generating the comparison
        self.generate_asm_from_tree(left_child)?;
        // If the generated left side is false then jump to the end of the if
        match &left_child.value {
            NodeValue::Value(_val) => {
                panic!("invalid and")
            }
            NodeValue::True => { /* */ }
            NodeValue::False => {
                writeln!(self.file, "    JMP    {label_jmp_to_end}")?;
                writeln!(self.file)?;
            }
            NodeValue::Action(action) => match action {
                AstAction::GT
                | AstAction::GTE
                | AstAction::EQ
                | AstAction::NE
                | AstAction::LT
                | AstAction::LTE => {
                    let jmp = Self::jmp_to_opposite_asm_jmp(action);
                    writeln!(self.file, "    {jmp}    {label_jmp_to_end}")?;
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
            NodeValue::Value(_val) => {
                panic!("invalid and")
            }
            NodeValue::True => { /* */ }
            NodeValue::False => {
                writeln!(self.file, "    JMP    {label_jmp_to_end}")?;
                writeln!(self.file)?;
            }
            NodeValue::Action(action) => match action {
                AstAction::GT
                | AstAction::GTE
                | AstAction::EQ
                | AstAction::NE
                | AstAction::LT
                | AstAction::LTE => {
                    let jmp = Self::jmp_to_opposite_asm_jmp(action);
                    writeln!(self.file, "    {jmp}    {label_jmp_to_end}")?;
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
        Ok(())
    }

    fn generate_action_or(&mut self, node: &Rc<Node>) -> Result<(), io::Error> {
        let label_begin_body = &self.current_begin_label.clone();
        let label_end_body = &self.current_end_label.clone();
        let Some(left_child) = node.left_child.as_ref() else {
            panic!("invalid OR");
        };
        // Traverse the left subtree generating the comparison
        self.generate_asm_from_tree(left_child)?;
        // If either the left side or the right side are true we jump to the if body
        match &left_child.value {
            NodeValue::Value(_val) => {
                panic!("invalid or")
            }
            NodeValue::True => {
                writeln!(self.file, "    JMP    {label_begin_body}")?;
                writeln!(self.file)?;
            }
            NodeValue::False => { /* */ }
            NodeValue::Action(action) => match action {
                AstAction::GT
                | AstAction::GTE
                | AstAction::EQ
                | AstAction::NE
                | AstAction::LT
                | AstAction::LTE => {
                    let jmp = Self::jmp_to_asm_jmp(action);
                    writeln!(self.file, "    {jmp}    {label_begin_body}")?;
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
            NodeValue::Value(_val) => {
                panic!("invalid or")
            }
            NodeValue::True => {
                writeln!(self.file, "    JMP    {label_begin_body}")?;
                writeln!(self.file)?;
            }
            NodeValue::False => { /* */ }
            NodeValue::Action(action) => match action {
                AstAction::GT
                | AstAction::GTE
                | AstAction::EQ
                | AstAction::NE
                | AstAction::LT
                | AstAction::LTE => {
                    let jmp = Self::jmp_to_asm_jmp(action);
                    writeln!(self.file, "    {jmp}    {label_begin_body}")?;
                    writeln!(self.file)?;
                }
                AstAction::And | AstAction::Or => {}
                _ => panic!("invalid"),
            },
        }
        // None of the conditions are met jump to the end of the statement
        writeln!(self.file, "    JMP    {label_end_body}")
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
        let Some(left_child) = &node.left_child else {
            panic!("invalid write")
        };
        self.generate_asm_from_tree(left_child)?;
        let Some(write_type) = &left_child.r#type else {
            panic!("invalid write")
        };

        match write_type {
            ExpressionType::Float => {
                writeln!(self.file, "    FST    _@write_number")?;
                writeln!(self.file, "    DisplayFloat    _@write_number, 2")?;
            }
            ExpressionType::Int => {
                writeln!(self.file, "    FST    _@write_number")?;
                writeln!(self.file, "    DisplayInteger    _@write_number")?;
            }
            ExpressionType::String => {
                writeln!(self.file, "    FST    _@write_string")?;
                writeln!(self.file, "    DisplayString    _@write_string")?;
            }
        }
        writeln!(self.file, "    newLine")?;
        writeln!(self.file)
    }

    fn generate_action_while(&mut self, node: &Rc<Node>) -> Result<(), io::Error> {
        let while_cond_label = format!("while_cond_{}", self.label_while_cond_count);
        let while_end_label = format!("while_end_{}", self.label_while_cond_count);
        self.label_while_cond_count += 1;
        self.current_begin_label = while_cond_label.clone();
        self.current_end_label = while_end_label.clone();

        // Set the label to the beggining of the loop
        writeln!(self.file, "{while_cond_label}:")?;
        let Some(left_child) = node.left_child.as_ref() else {
            panic!("invalid while");
        };
        self.generate_asm_from_tree(left_child)?;
        // When the condition is false jump to the end of while
        match &left_child.value {
            NodeValue::Value(_val) => panic!("invalid while"),
            NodeValue::False => {
                writeln!(self.file, "    JMP    {while_end_label}")?;
                writeln!(self.file)?;
            }
            NodeValue::True => { /* */ }
            NodeValue::Action(action) => match action {
                AstAction::GT
                | AstAction::GTE
                | AstAction::EQ
                | AstAction::NE
                | AstAction::LT
                | AstAction::LTE => {
                    let jmp = Self::jmp_to_opposite_asm_jmp(action);
                    writeln!(self.file, "    {jmp}    {while_end_label}")?;
                    writeln!(self.file)?;
                }
                AstAction::And | AstAction::Or => {}
                _ => panic!("invalid"),
            },
        };
        // Generate body of the while
        self.generate_asm_from_tree(node.right_child.as_ref().unwrap())?;
        // Jump to begging of while
        writeln!(self.file, "    JMP    {while_cond_label}")?;
        writeln!(self.file)?;
        // End of while label
        writeln!(self.file, "{while_end_label}:")
    }

    fn generate_action_else(
        &mut self,
        node: &Rc<Node>,
        condition_node: &Rc<Node>,
    ) -> Result<(), io::Error> {
        let begin_else_label = format!("else_{}", self.label_if_else_body_count);
        let end_if_else_label = format!("end_if_else{}", self.label_if_else_body_count);
        self.current_begin_label = begin_else_label.clone();

        match &condition_node.value {
            NodeValue::Value(_val) => panic!("invalid if else"),
            NodeValue::False => {
                writeln!(self.file, "    JMP    {begin_else_label}")?;
                writeln!(self.file)?;
            }
            NodeValue::True => { /* */ }
            NodeValue::Action(action) => match action {
                AstAction::GT
                | AstAction::GTE
                | AstAction::EQ
                | AstAction::NE
                | AstAction::LT
                | AstAction::LTE => {
                    let jmp = Self::jmp_to_opposite_asm_jmp(action);
                    writeln!(self.file, "    {jmp}    {begin_else_label}")?;
                    writeln!(self.file)?;
                }
                AstAction::And | AstAction::Or => {}
                _ => panic!("invalid"),
            },
        }
        self.generate_asm_from_tree(node.left_child.as_ref().unwrap())?;
        writeln!(self.file, "    JMP    {end_if_else_label}")?;
        writeln!(self.file)?;
        writeln!(self.file, "{}:", begin_else_label)?;
        self.generate_asm_from_tree(node.right_child.as_ref().unwrap())?;
        writeln!(self.file, "{}:", end_if_else_label)
    }

    fn generate_action_read(&mut self, node: &Rc<Node>) -> Result<(), io::Error> {
        let NodeValue::Value(val) = &node.left_child.as_ref().unwrap().value else {
            panic!("invalid read")
        };
        let Some(symbol) = self.symbol_table.get_symbol_asm_name(val) else {
            panic!("missing symbol")
        };
        let SymbolTableElementType::DataType(symbol_type) = symbol.data_type else {
            panic!("invalid read")
        };
        match symbol_type {
            DataType::FloatType(_) => {
                writeln!(self.file, "    GetFloat    {}", symbol.name)?;
            }
            DataType::IntType(_) => {
                writeln!(self.file, "    GetFloat    {}", symbol.name)?;
            }
            DataType::StringType(_) => {
                writeln!(self.file, "    GetString    {}", symbol.name)?;
            }
        }
        writeln!(self.file, "    newLine")?;
        writeln!(self.file)
    }

    const fn jmp_to_opposite_asm_jmp(jmp: &AstAction) -> &'a str {
        match jmp {
            AstAction::GT => "JNAE",
            AstAction::GTE => "JNA",
            AstAction::EQ => "JNE",
            AstAction::NE => "JE",
            AstAction::LT => "JAE",
            AstAction::LTE => "JA",
            _ => todo!(),
        }
    }

    const fn jmp_to_asm_jmp(jmp: &AstAction) -> &'a str {
        match jmp {
            AstAction::GT => "JA",
            AstAction::GTE => "JAE",
            AstAction::EQ => "EQ",
            AstAction::NE => "JNE",
            AstAction::LT => "JNAE",
            AstAction::LTE => "JNA",
            _ => todo!(),
        }
    }
}
