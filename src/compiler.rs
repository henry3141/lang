use crate::interpreter::Operation;
use crate::interpreter::DATA;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Instruction {
    SET {
        name: String,
        value: Expression,
    },
    ASSIGN {
        name: String,
        value: Expression,
    },
    PRINT {
        value: Expression,
    },
    BLOCK {
        instruction: Vec<Instruction>,
    },
    IF {
        condition: Expression,
        instruction: Vec<Instruction>,
    },
    EXIT,
    FUNCTION {
        name: String,
        args: Vec<String>,
        instruction: Vec<Instruction>,
    },
    RETURN {
        value: Expression,
    },
    LOOP {
        instruction: Vec<Instruction>,
    },
    WHILE {
        condition: Expression,
        instruction: Vec<Instruction>,
    },
    DROP {
        name: String,
    },
    HALT,
}

impl Instruction {
    fn change_name(&self, add: i32, of: &mut Vec<String>) -> Instruction {
        match self.clone() {
            Instruction::SET { mut name, value } => {
                of.push(name.clone());
                name.push_str(&add.to_string());
                Instruction::SET {
                    name,
                    value: value.change_name(add, of),
                }
            }
            Instruction::ASSIGN { mut name, value } => {
                if of.contains(&name) {
                    name.push_str(&add.to_string());
                }
                Instruction::ASSIGN {
                    name,
                    value: value.change_name(add, of),
                }
            }
            Instruction::PRINT { value } => Instruction::PRINT {
                value: value.change_name(add, of),
            },
            Instruction::BLOCK { instruction } => {
                let mut new_instruction = Vec::new();
                for i in instruction {
                    new_instruction.push(i.change_name(add, of));
                }
                Instruction::BLOCK {
                    instruction: new_instruction,
                }
            }
            Instruction::IF {
                condition,
                instruction,
            } => Instruction::IF {
                condition: condition.change_name(add, of),
                instruction: instruction.iter().map(|x| x.change_name(add, of)).collect(),
            },
            Instruction::DROP { mut name } => {
                if of.contains(&name) {
                    name.push_str(&add.to_string());
                    Instruction::DROP { name }
                } else {
                    Instruction::DROP { name }
                }
            }
            Instruction::FUNCTION {
                mut name,
                args,
                instruction,
            } => {
                of.push(name.clone());
                name.push_str(&add.to_string());
                let mut new_instruction = Vec::new();
                for i in instruction {
                    new_instruction.push(i.change_name(add, of));
                }
                Instruction::FUNCTION {
                    name,
                    args,
                    instruction: new_instruction,
                }
            }
            Instruction::RETURN { value } => Instruction::RETURN {
                value: value.change_name(add, of),
            },
            Instruction::LOOP { instruction } => {
                let mut new_instruction = Vec::new();
                for i in instruction {
                    new_instruction.push(i.change_name(add, of));
                }
                Instruction::LOOP {
                    instruction: new_instruction,
                }
            }
            Instruction::WHILE {
                condition,
                instruction,
            } => Instruction::WHILE {
                condition: condition.change_name(add, of),
                instruction: instruction.iter().map(|x| x.change_name(add, of)).collect(),
            },
            _ => self.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Expression {
    Number(i32),
    CALL(String, Vec<Expression>),
    Bool(bool),
    String(String),
    Variable(String),
    Add(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>),
    EQ(Box<Expression>, Box<Expression>),
    NOT(Box<Expression>),
    GREATER(Box<Expression>, Box<Expression>),
}

impl Expression {
    fn change_name(&self, add: i32, of: &mut Vec<String>) -> Self {
        match self.clone() {
            Expression::CALL(mut name, data) => {
                if of.contains(&name) {
                    name.push_str(&add.to_string());
                }
                let mut new_data = Vec::new();
                for i in data {
                    new_data.push(i.change_name(add, of));
                }
                Expression::CALL(name, new_data)
            }
            Expression::Variable(mut name) => {
                if of.contains(&name) {
                    name.push_str(&add.to_string());
                    Expression::Variable(name)
                } else {
                    Expression::Variable(name)
                }
            }
            Expression::Add(a, b) => Expression::Add(
                Box::new(a.change_name(add, of)),
                Box::new(b.change_name(add, of)),
            ),
            Expression::Sub(a, b) => Expression::Sub(
                Box::new(a.change_name(add, of)),
                Box::new(b.change_name(add, of)),
            ),
            Expression::Mul(a, b) => Expression::Mul(
                Box::new(a.change_name(add, of)),
                Box::new(b.change_name(add, of)),
            ),
            Expression::Div(a, b) => Expression::Div(
                Box::new(a.change_name(add, of)),
                Box::new(b.change_name(add, of)),
            ),
            Expression::EQ(a, b) => Expression::EQ(
                Box::new(a.change_name(add, of)),
                Box::new(b.change_name(add, of)),
            ),
            Expression::NOT(a) => Expression::NOT(Box::new(a.change_name(add, of))),
            Expression::GREATER(a, b) => Expression::GREATER(
                Box::new(a.change_name(add, of)),
                Box::new(b.change_name(add, of)),
            ),
            _ => self.clone(),
        }
    }

    pub fn to_addr(&self, addr: i32, compiler: &mut Compiler) -> Vec<Operation> {
        match self {
            Expression::GREATER(a, b) => {
                let addr_a = compiler.new_addr();
                let addr_b = compiler.new_addr();
                let mut ops_a = a.to_addr(addr_a, compiler);
                let ops_b = b.to_addr(addr_b, compiler);
                let ops = vec![Operation::GREATER {
                    name: DATA::POINTER(addr_a),
                    value: DATA::POINTER(addr_b),
                    ret: DATA::Number(addr),
                }];
                ops_a.extend(ops_b);
                ops_a.extend(ops);
                ops_a
            }
            Expression::EQ(a, b) => {
                let addr_a = compiler.new_addr();
                let addr_b = compiler.new_addr();
                let mut ops_a = a.to_addr(addr_a, compiler);
                let ops_b = b.to_addr(addr_b, compiler);
                let ops = vec![Operation::EQ {
                    name: DATA::POINTER(addr_a),
                    value: DATA::POINTER(addr_b),
                    ret: DATA::Number(addr),
                }];
                ops_a.extend(ops_b);
                ops_a.extend(ops);
                ops_a
            }
            Expression::NOT(a) => {
                let addr_a = compiler.new_addr();
                let mut ops_a = a.to_addr(addr_a, compiler);
                let ops = vec![Operation::NOT {
                    name: DATA::POINTER(addr_a),
                    ret: DATA::Number(addr),
                }];
                ops_a.extend(ops);
                ops_a
            }
            Expression::Number(n) => {
                vec![Operation::SET {
                    name: DATA::Number(addr),
                    value: DATA::Number(*n),
                }]
            }
            Expression::Bool(b) => {
                vec![Operation::SET {
                    name: DATA::Number(addr),
                    value: DATA::Bool(*b),
                }]
            }
            Expression::String(s) => {
                vec![Operation::SET {
                    name: DATA::Number(addr),
                    value: DATA::String(s.clone()),
                }]
            }
            Expression::Variable(v) => {
                vec![Operation::SET {
                    name: DATA::Number(addr),
                    value: DATA::POINTER(compiler.vars[v]),
                }]
            }
            Expression::Add(a, b) => {
                let addr_a = compiler.new_addr();
                let addr_b = compiler.new_addr();
                let mut ops_a = a.to_addr(addr_a, compiler);
                let ops_b = b.to_addr(addr_b, compiler);
                let ops = vec![Operation::ADD {
                    name: DATA::POINTER(addr_a),
                    value: DATA::POINTER(addr_b),
                    ret: DATA::Number(addr),
                }];
                ops_a.extend(ops_b);
                ops_a.extend(ops);
                ops_a
            }
            Expression::Sub(a, b) => {
                let addr_a = compiler.new_addr();
                let addr_b = compiler.new_addr();
                let mut ops_a = a.to_addr(addr_a, compiler);
                let ops_b = b.to_addr(addr_b, compiler);
                let ops = vec![Operation::SUB {
                    name: DATA::POINTER(addr_a),
                    value: DATA::POINTER(addr_b),
                    ret: DATA::Number(addr),
                }];
                ops_a.extend(ops_b);
                ops_a.extend(ops);
                ops_a
            }
            Expression::Mul(a, b) => {
                let addr_a = compiler.new_addr();
                let addr_b = compiler.new_addr();
                let mut ops_a = a.to_addr(addr_a, compiler);
                let ops_b = b.to_addr(addr_b, compiler);
                let ops = vec![Operation::MUL {
                    name: DATA::POINTER(addr_a),
                    value: DATA::POINTER(addr_b),
                    ret: DATA::Number(addr),
                }];
                ops_a.extend(ops_b);
                ops_a.extend(ops);
                ops_a
            }
            Expression::Div(a, b) => {
                let addr_a = compiler.new_addr();
                let addr_b = compiler.new_addr();
                let mut ops_a = a.to_addr(addr_a, compiler);
                let ops_b = b.to_addr(addr_b, compiler);
                let ops = vec![Operation::DIV {
                    name: DATA::POINTER(addr_a),
                    value: DATA::POINTER(addr_b),
                    ret: DATA::Number(addr),
                }];
                ops_a.extend(ops_b);
                ops_a.extend(ops);
                ops_a
            }
            Expression::CALL(name, args) => {
                let func = compiler.functions.get(name).unwrap().clone();
                let mut ops = vec![Operation::SET {
                    name: DATA::Number(func.return_addr),
                    value: DATA::Number(addr),
                }];
                for (i, arg) in args.iter().zip(func.args.iter()) {
                    let arg_addr = compiler.new_var(arg.to_string());
                    let mut arg_ops = i.to_addr(arg_addr, compiler);
                    ops.append(&mut arg_ops);
                }
                ops.push(Operation::CALL {
                    name: DATA::Number(func.addr),
                });
                ops
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct FUNCTION {
    pub name: String,
    pub args: Vec<String>,
    pub return_addr: i32,
    pub addr: i32,
}

pub struct Compiler {
    pub instructions: Vec<Instruction>,
    pub program: Vec<Operation>,
    pub fuctions_programms: Vec<Operation>,
    pub functions: HashMap<String, FUNCTION>,
    pub vars: HashMap<String, i32>,
    pub caddr: i32,
}

impl Compiler {
    pub fn compile(p: Vec<Instruction>) -> Vec<Operation> {
        let mut compiler = Compiler {
            instructions: p,
            program: Vec::new(),
            vars: HashMap::new(),
            caddr: 0,
            functions: HashMap::new(),
            fuctions_programms: Vec::new(),
        };
        compiler.compile_instructions();
        compiler.program
    }

    pub fn continues() -> Compiler {
        Compiler {
            instructions: Vec::new(),
            program: Vec::new(),
            vars: HashMap::new(),
            caddr: 0,
            functions: HashMap::new(),
            fuctions_programms: Vec::new(),
        }
    }

    pub fn continues_compile(&mut self, p: Vec<Instruction>) -> Vec<Operation> {
        self.instructions = p;
        self.program = Vec::new();
        self.compile_instructions();
        self.program.clone()
    }

    fn new_addr(&mut self) -> i32 {
        let addr = self.caddr;
        self.caddr += 1;
        addr
    }

    fn new_var(&mut self, name: String) -> i32 {
        let addr = self.new_addr();
        self.vars.insert(name, addr);
        addr
    }

    fn compile_instruction(&mut self, inst: Instruction) -> Vec<Operation> {
        match inst {
            Instruction::HALT => {
                vec![Operation::HALT]
            }
            Instruction::FUNCTION {
                name,
                args,
                mut instruction,
            } => {
                let add = self.new_addr();
                let func = FUNCTION {
                    name: name.clone(),
                    args: args
                        .clone()
                        .iter()
                        .map(|x| {
                            let mut x = x.clone();
                            x.push_str(&add.to_string());
                            x
                        })
                        .collect(),
                    return_addr: self.new_addr(),
                    addr: self.new_addr(),
                };
                self.functions.insert(name.clone(), func.clone());
                let mut ops = vec![Operation::POINT {
                    name: DATA::Number(func.addr),
                }];
                let last = instruction.pop().unwrap();
                let mut of = Vec::new();
                for i in instruction {
                    let mut iops = self.compile_instruction(i.change_name(add, &mut of));
                    ops.append(&mut iops);
                }
                match last {
                    Instruction::RETURN { value } => {
                        let addr = self.new_addr();
                        let mut iops = value.to_addr(addr, self);
                        iops.push(Operation::SET {
                            name: DATA::POINTER(func.return_addr),
                            value: DATA::POINTER(addr),
                        });
                    }
                    _ => ops.extend(self.compile_instruction(last.change_name(add, &mut of))),
                };
                ops.push(Operation::RET);
                ops
            }
            Instruction::DROP { name } => {
                let addr = self.vars[&name];
                self.vars.remove(&name);
                vec![Operation::DROP {
                    name: DATA::Number(addr),
                }]
            }
            Instruction::SET { name, value } => {
                let addr = self.new_var(name.clone());
                let ops = value.to_addr(addr, self);
                ops
            }
            Instruction::ASSIGN { name, value } => {
                let addr = self
                    .vars
                    .get(&name)
                    .unwrap_or_else(|| {
                        println!("var not defined: {}", name);
                        std::process::exit(1);
                    })
                    .clone();
                let ops = value.to_addr(addr, self);
                ops
            }
            Instruction::PRINT { value } => {
                let addr = self.new_addr();
                let mut ops = value.to_addr(addr, self);
                ops.push(Operation::PRINT {
                    value: DATA::POINTER(addr),
                });
                ops
            }
            Instruction::LOOP { instruction } => {
                let addr = self.new_addr();
                let mut ops = vec![Operation::POINT {
                    name: DATA::Number(addr),
                }];
                for i in instruction {
                    let mut iops = self.compile_instruction(i);
                    ops.append(&mut iops);
                }
                ops.push(Operation::JUMP {
                    name: DATA::Number(addr),
                });
                ops
            }
            Instruction::BLOCK { instruction } => {
                let mut ops = Vec::new();
                for i in instruction {
                    let mut iops = self.compile_instruction(i);
                    ops.append(&mut iops);
                }
                ops
            }
            Instruction::IF {
                condition,
                instruction,
            } => {
                let addr = self.new_addr();
                let mut ops = Expression::NOT(Box::new(condition)).to_addr(addr, self);
                let jump_addr = self.new_addr();
                ops.push(Operation::JUMP_IF {
                    name: DATA::Number(jump_addr),
                    condition: DATA::POINTER(addr),
                });
                for i in instruction {
                    let mut iops = self.compile_instruction(i);
                    ops.append(&mut iops);
                }
                ops.push(Operation::POINT {
                    name: DATA::Number(jump_addr),
                });
                ops
            }
            Instruction::WHILE {
                condition,
                instruction,
            } => {
                let jump_start: i32 = self.new_addr();
                let jump_end: i32 = self.new_addr();
                let cond_addr = self.new_addr();
                let mut ops = vec![Operation::POINT {
                    name: DATA::Number(jump_start),
                }];
                ops.extend(Expression::NOT(Box::new(condition)).to_addr(cond_addr, self));
                ops.push(Operation::JUMP_IF {
                    name: DATA::Number(jump_end),
                    condition: DATA::POINTER(cond_addr),
                });
                for i in instruction {
                    let mut iops = self.compile_instruction(i);
                    ops.append(&mut iops);
                }
                ops.push(Operation::JUMP {
                    name: DATA::Number(jump_start),
                });
                ops.push(Operation::POINT {
                    name: DATA::Number(jump_end),
                });
                ops
            }
            Instruction::RETURN { value:_ } => {
                panic!("return outside function last")
            }
            Instruction::EXIT => {
                vec![Operation::RET]
            }
        }
    }

    fn compile_instructions(&mut self) {
        for i in self.instructions.clone() {
            let ops = self.compile_instruction(i);
            self.program.extend(ops);
        }
        self.program.push(Operation::HALT);
    }
}
