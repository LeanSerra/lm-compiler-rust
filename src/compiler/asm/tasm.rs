use std::{
    fs::File,
    io::{self, Write},
    rc::Rc,
};

use crate::{
    compiler::{
        ast::{AstAction, ExpressionType, Node, NodeValue},
        context::{SymbolTable, SymbolTableElement, SymbolTableElementType},
        error::CompilerError,
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

    pub fn generate_asm(mut self, root: Rc<Node>) -> Result<(), CompilerError> {
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
        self.generate_code_epilogue()?;
        Ok(())
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
        for symbol in [
            neg_one_symbol,
            l_comp_symbol,
            r_comp_symbol,
            write_number_symbol,
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

    fn generate_asm_from_tree(&mut self, node: &Rc<Node>) -> Result<(), CompilerError> {
        match &node.value {
            NodeValue::Value(val) => self.generate_node_value_value(val)?,
            NodeValue::True => {}
            NodeValue::False => {}
            NodeValue::Action(action) => match action {
                AstAction::S => {
                    self.generate_asm_from_tree(node.left_child.as_ref().unwrap())?;
                    self.generate_asm_from_tree(node.right_child.as_ref().unwrap())?;
                }
                AstAction::Assign => self.generate_action_assign(node)?,
                AstAction::GT
                | AstAction::GTE
                | AstAction::EQ
                | AstAction::NE
                | AstAction::LT
                | AstAction::LTE => self.generate_action_cmp(node)?,
                AstAction::Plus => {
                    self.generate_asm_from_tree(node.left_child.as_ref().unwrap())?;
                    self.generate_asm_from_tree(node.right_child.as_ref().unwrap())?;
                    writeln!(self.file, "    FADD")?;
                }
                AstAction::Div => {
                    self.generate_asm_from_tree(node.left_child.as_ref().unwrap())?;
                    self.generate_asm_from_tree(node.right_child.as_ref().unwrap())?;
                    writeln!(self.file, "    FDIV")?;
                }
                AstAction::Sub => {
                    self.generate_asm_from_tree(node.left_child.as_ref().unwrap())?;
                    self.generate_asm_from_tree(node.right_child.as_ref().unwrap())?;
                    writeln!(self.file, "    FSUB")?;
                }
                AstAction::Mult => {
                    self.generate_asm_from_tree(node.left_child.as_ref().unwrap())?;
                    self.generate_asm_from_tree(node.right_child.as_ref().unwrap())?;
                    writeln!(self.file, "    FMUL")?;
                }

                AstAction::If => self.generate_action_if(node)?,
                AstAction::Else => {
                    return Err(CompilerError::Internal(
                        "Tried to generate code for else node from generate_asm_from_tree".into(),
                    ));
                }
                AstAction::And => self.generate_action_and(node)?,
                AstAction::Or => self.generate_action_or(node)?,
                AstAction::While => self.generate_action_while(node)?,
                AstAction::Read => self.generate_action_read(node)?,
                AstAction::Write => self.generate_action_write(node)?,
                AstAction::Negative => self.generate_action_negative(node)?,
                AstAction::Noop => {}
            },
        }
        Ok(())
    }

    fn generate_node_value_value(&mut self, val: &str) -> Result<(), CompilerError> {
        let val = self
            .symbol_table
            .get_symbol_from_name(val)
            .ok_or(CompilerError::Internal(
                "Symbol not found when generating ASM for NodeValue::Value".into(),
            ))?;
        if let SymbolTableElementType::String = val.data_type {
            return Ok(());
        }
        if let SymbolTableElementType::DataType(DataType::StringType(_)) = val.data_type {
            return Ok(());
        }
        writeln!(self.file, "    FLD     {}", val.name)?;
        Ok(())
    }

    fn generate_action_assign(&mut self, node: &Rc<Node>) -> Result<(), CompilerError> {
        let (left_child, right_child) =
            Self::get_left_and_right_child_or_error(node, "Invalid assignment node")?;

        self.generate_asm_from_tree(&right_child)?;
        let NodeValue::Value(lhs) = &left_child.value else {
            return Err(CompilerError::Internal(
                "Left hand side of an assignment is not an id".into(),
            ));
        };

        let lhs_symbol =
            self.symbol_table
                .get_symbol_from_name(lhs)
                .ok_or(CompilerError::Internal(
                    "Left hand side of an assignment is not in the symbol table".into(),
                ))?;

        writeln!(self.file, "    FST    {}", lhs_symbol.name)?;
        writeln!(self.file, "    FFREE")?;
        writeln!(self.file)?;
        Ok(())
    }

    fn generate_action_cmp(&mut self, node: &Rc<Node>) -> Result<(), CompilerError> {
        let (left_child, right_child) =
            Self::get_left_and_right_child_or_error(node, "Invalid comparison node")?;

        self.generate_asm_from_tree(&left_child)?;
        writeln!(self.file, "    FST     _@l_cond")?;
        writeln!(self.file)?;
        self.generate_asm_from_tree(&right_child)?;
        writeln!(self.file, "    FST     _@r_cond")?;
        writeln!(self.file)?;
        writeln!(self.file, "    FLD     _@l_cond")?;
        writeln!(self.file, "    FCOMP   _@r_cond")?;
        writeln!(self.file, "    FSTSW   AX")?;
        writeln!(self.file, "    SAHF")?;
        writeln!(self.file, "    FFREE")?;
        writeln!(self.file)?;
        Ok(())
    }

    fn generate_action_if(&mut self, node: &Rc<Node>) -> Result<(), CompilerError> {
        let (left_child, right_child) =
            Self::get_left_and_right_child_or_error(node, "Invalid If node")?;

        if let NodeValue::Action(AstAction::Or) = left_child.value {
            let label_if_body = format!("if_body_{}", self.label_if_body_count);
            self.label_if_body_count += 1;
            self.current_begin_label = label_if_body.clone();
        }

        let label_if_false = format!("if_false_{}", self.label_if_false_count);
        self.current_end_label = label_if_false.clone();
        self.generate_asm_from_tree(&left_child)?;

        if let NodeValue::Action(AstAction::Else) = right_child.value {
            return self.generate_action_else(&right_child, &left_child);
        }

        // Create jump to label if false depending on operator
        match &left_child.value {
            NodeValue::Value(_val) => {
                return Err(CompilerError::Internal(
                    "Invalid If node lhs is a value".into(),
                ));
            }
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
                    let jmp = Self::jmp_to_opposite_asm_jmp(action).ok_or(
                        CompilerError::Internal("Tried to jump to invalid action".into()),
                    )?;
                    writeln!(self.file, "    {jmp}    {label_if_false}")?;
                    writeln!(self.file)?;
                }
                AstAction::And => {}
                AstAction::Or => {
                    // TODO should we generate this label every time?
                    let label_if_body = format!("if_body_{}", self.label_if_body_count);
                    self.label_if_false_count += 1;
                    // Generate label to jump if any of the OR statements are true
                    writeln!(self.file, "{label_if_body}:")?;
                }
                action => {
                    return Err(CompilerError::Internal(format!(
                        "Invalid action: {action} in If node"
                    )));
                }
            },
        };
        // Generate If body
        self.generate_asm_from_tree(&right_child)?;
        // Label to jump if statement is false
        writeln!(self.file, "{label_if_false}:")?;
        Ok(())
    }

    fn generate_action_and(&mut self, node: &Rc<Node>) -> Result<(), CompilerError> {
        let (left_child, right_child) =
            Self::get_left_and_right_child_or_error(node, "Invalid AND node")?;

        let label_jmp_to_end = &self.current_end_label.clone();
        // Traverse the left subtree generating the comparison
        self.generate_asm_from_tree(&left_child)?;
        // If the generated left side is false then jump to the end of the if
        match &left_child.value {
            NodeValue::Value(_val) => {
                return Err(CompilerError::Internal(
                    "Invalid And node lhs is a value".into(),
                ));
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
                    let jmp = Self::jmp_to_opposite_asm_jmp(action).ok_or(
                        CompilerError::Internal("Tried to jump to invalid action".into()),
                    )?;
                    writeln!(self.file, "    {jmp}    {label_jmp_to_end}")?;
                    writeln!(self.file)?;
                }
                AstAction::Or => {
                    // TODO maybe we need this
                    // writeln!(self.file, "    JMP    {label_if_false}")?;
                }
                AstAction::And => {}
                action => {
                    return Err(CompilerError::Internal(format!(
                        "Invalid action: {action} in And node"
                    )));
                }
            },
        }

        self.generate_asm_from_tree(&right_child)?;
        // If the generated right side is false then jump to the end of the if
        match &right_child.value {
            NodeValue::Value(_val) => {
                return Err(CompilerError::Internal(
                    "Invalid And node lhs is a value".into(),
                ));
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
                    let jmp = Self::jmp_to_opposite_asm_jmp(action).ok_or(
                        CompilerError::Internal("Tried to jump to invalid action".into()),
                    )?;
                    writeln!(self.file, "    {jmp}    {label_jmp_to_end}")?;
                    writeln!(self.file)?;
                }
                AstAction::Or => {
                    // TODO maybe we need this
                    // writeln!(self.file, "    JMP    {label_if_false}")?;
                }
                AstAction::And => {}
                action => {
                    return Err(CompilerError::Internal(format!(
                        "Invalid action: {action} in And node"
                    )));
                }
            },
        }
        Ok(())
    }

    fn generate_action_or(&mut self, node: &Rc<Node>) -> Result<(), CompilerError> {
        let (left_child, right_child) =
            Self::get_left_and_right_child_or_error(node, "Invalid AND node")?;

        let label_begin_body = &self.current_begin_label.clone();
        let label_end_body = &self.current_end_label.clone();
        // Traverse the left subtree generating the comparison
        self.generate_asm_from_tree(&left_child)?;
        // If either the left side or the right side are true we jump to the if body
        match &left_child.value {
            NodeValue::Value(_val) => {
                return Err(CompilerError::Internal(
                    "Invalid Or node lhs is a value".into(),
                ));
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
                    let jmp = Self::jmp_to_asm_jmp(action).ok_or(CompilerError::Internal(
                        "Tried to jump to invalid action".into(),
                    ))?;
                    writeln!(self.file, "    {jmp}    {label_begin_body}")?;
                    writeln!(self.file)?;
                }
                AstAction::And | AstAction::Or => {}
                action => {
                    return Err(CompilerError::Internal(format!(
                        "Invalid action: {action} in Or node"
                    )));
                }
            },
        }

        self.generate_asm_from_tree(&right_child)?;
        match &right_child.value {
            NodeValue::Value(_val) => {
                return Err(CompilerError::Internal(
                    "Invalid Or node lhs is a value".into(),
                ));
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
                    let jmp = Self::jmp_to_opposite_asm_jmp(action).ok_or(
                        CompilerError::Internal("Tried to jump to invalid action".into()),
                    )?;
                    writeln!(self.file, "    {jmp}    {label_begin_body}")?;
                    writeln!(self.file)?;
                }
                AstAction::And | AstAction::Or => {}
                action => {
                    return Err(CompilerError::Internal(format!(
                        "Invalid action: {action} in Or node"
                    )));
                }
            },
        }
        // None of the conditions are met jump to the end of the statement
        writeln!(self.file, "    JMP    {label_end_body}")?;
        Ok(())
    }

    fn generate_action_negative(&mut self, node: &Rc<Node>) -> Result<(), CompilerError> {
        let left_child = Self::get_left_child_or_error(node, "No left child on Negative node")?;
        let NodeValue::Value(lhs) = &left_child.value else {
            return Err(CompilerError::Internal(
                "Left child of Negative node is not a value ".into(),
            ));
        };
        let symbol = self
            .symbol_table
            .get_symbol_from_name(lhs)
            .ok_or(CompilerError::Internal(
                "Left side of Negative node is not in the symbol table".into(),
            ))?;

        writeln!(self.file, "    FLD    _@1")?;
        writeln!(self.file, "    FMUL   {}", symbol.name)?;
        writeln!(self.file)?;
        Ok(())
    }

    fn generate_action_write(&mut self, node: &Rc<Node>) -> Result<(), CompilerError> {
        let left_child = Self::get_left_child_or_error(node, "No left child on Write node")?;
        self.generate_asm_from_tree(&left_child)?;
        let write_type = left_child.r#type.as_ref().ok_or(CompilerError::Internal(
            "Left child of write expression has no type".into(),
        ))?;

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
                let NodeValue::Value(val) = &left_child.value else {
                    return Err(CompilerError::Internal(
                        "Left child of Write node of String type is not a Value".into(),
                    ));
                };
                let name =
                    self.symbol_table
                        .get_symbol_from_name(val)
                        .ok_or(CompilerError::Internal(
                            "Left child symbol is not in the symbol table in write node".into(),
                        ))?;
                writeln!(self.file, "    DisplayString    {}", name.name)?;
            }
        }
        writeln!(self.file, "    newLine")?;
        writeln!(self.file)?;
        Ok(())
    }

    fn generate_action_while(&mut self, node: &Rc<Node>) -> Result<(), CompilerError> {
        let (left_child, right_child) =
            Self::get_left_and_right_child_or_error(node, "Invalid While node")?;
        let while_cond_label = format!("while_cond_{}", self.label_while_cond_count);
        let while_end_label = format!("while_end_{}", self.label_while_cond_count);
        self.label_while_cond_count += 1;
        self.current_begin_label = while_cond_label.clone();
        self.current_end_label = while_end_label.clone();

        // Set the label to the beggining of the loop
        writeln!(self.file, "{while_cond_label}:")?;
        self.generate_asm_from_tree(&left_child)?;
        // When the condition is false jump to the end of while
        match &left_child.value {
            NodeValue::Value(_val) => {
                return Err(CompilerError::Internal(
                    "Invalid While node lhs is a value".into(),
                ));
            }
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
                    let jmp = Self::jmp_to_opposite_asm_jmp(action).ok_or(
                        CompilerError::Internal("Tried to jump to invalid action".into()),
                    )?;
                    writeln!(self.file, "    {jmp}    {while_end_label}")?;
                    writeln!(self.file)?;
                }
                AstAction::And | AstAction::Or => {}
                action => {
                    return Err(CompilerError::Internal(format!(
                        "Invalid action: {action} in While node"
                    )));
                }
            },
        };
        // Generate body of the while
        self.generate_asm_from_tree(&right_child)?;
        // Jump to begging of while
        writeln!(self.file, "    JMP    {while_cond_label}")?;
        writeln!(self.file)?;
        // End of while label
        writeln!(self.file, "{while_end_label}:")?;
        Ok(())
    }

    fn generate_action_else(
        &mut self,
        node: &Rc<Node>,
        condition_node: &Rc<Node>,
    ) -> Result<(), CompilerError> {
        let begin_else_label = format!("else_{}", self.label_if_else_body_count);
        let end_if_else_label = format!("end_if_else{}", self.label_if_else_body_count);
        self.current_begin_label = begin_else_label.clone();

        match &condition_node.value {
            NodeValue::Value(_val) => {
                return Err(CompilerError::Internal(
                    "Invalid If Else node lhs is a value".into(),
                ));
            }
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
                    let jmp = Self::jmp_to_opposite_asm_jmp(action).ok_or(
                        CompilerError::Internal("Tried to jump to invalid action".into()),
                    )?;
                    writeln!(self.file, "    {jmp}    {begin_else_label}")?;
                    writeln!(self.file)?;
                }
                AstAction::And | AstAction::Or => {}
                action => {
                    return Err(CompilerError::Internal(format!(
                        "Invalid action: {action} in If Else node"
                    )));
                }
            },
        }
        self.generate_asm_from_tree(node.left_child.as_ref().unwrap())?;
        writeln!(self.file, "    JMP    {end_if_else_label}")?;
        writeln!(self.file)?;
        writeln!(self.file, "{}:", begin_else_label)?;
        self.generate_asm_from_tree(node.right_child.as_ref().unwrap())?;
        writeln!(self.file, "{}:", end_if_else_label)?;
        Ok(())
    }

    fn generate_action_read(&mut self, node: &Rc<Node>) -> Result<(), CompilerError> {
        let left_child = Self::get_left_child_or_error(node, "No left child on Read node")?;
        let NodeValue::Value(val) = &left_child.value else {
            return Err(CompilerError::Internal(
                "Invalid Read node left child is not a value".into(),
            ));
        };
        let symbol = self
            .symbol_table
            .get_symbol_from_name(val)
            .ok_or(CompilerError::Internal(
                "Left child symbol is not in the symbol table in read node".into(),
            ))?;

        let SymbolTableElementType::DataType(symbol_type) = symbol.data_type else {
            return Err(CompilerError::Internal(
                "Left child symbol is not a variable".into(),
            ));
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
        writeln!(self.file)?;
        Ok(())
    }

    const fn jmp_to_opposite_asm_jmp(jmp: &AstAction) -> Option<&'a str> {
        Some(match jmp {
            AstAction::GT => "JNAE",
            AstAction::GTE => "JNA",
            AstAction::EQ => "JNE",
            AstAction::NE => "JE",
            AstAction::LT => "JAE",
            AstAction::LTE => "JA",
            _ => return None,
        })
    }

    const fn jmp_to_asm_jmp(jmp: &AstAction) -> Option<&'a str> {
        Some(match jmp {
            AstAction::GT => "JA",
            AstAction::GTE => "JAE",
            AstAction::EQ => "EQ",
            AstAction::NE => "JNE",
            AstAction::LT => "JNAE",
            AstAction::LTE => "JNA",
            _ => return None,
        })
    }

    fn get_left_and_right_child_or_error(
        node: &Rc<Node>,
        err: &str,
    ) -> Result<(Rc<Node>, Rc<Node>), CompilerError> {
        let left_child = Self::get_left_child_or_error(node, err)?;
        let right_child = Self::get_right_child_or_error(node, err)?;
        Ok((left_child, right_child))
    }

    fn get_left_child_or_error(node: &Rc<Node>, err: &str) -> Result<Rc<Node>, CompilerError> {
        node.left_child
            .as_ref()
            .cloned()
            .ok_or(CompilerError::Internal(err.into()))
    }

    fn get_right_child_or_error(node: &Rc<Node>, err: &str) -> Result<Rc<Node>, CompilerError> {
        node.right_child
            .as_ref()
            .cloned()
            .ok_or(CompilerError::Internal(err.into()))
    }
}
