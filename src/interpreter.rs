use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum DATA {
    Number(i32),
    Bool(bool),
    POINTER(i32),
    String(String),
}

impl DATA {
    fn to_string(&self) -> String {
        match self {
            DATA::Number(n) => n.to_string(),
            DATA::Bool(b) => b.to_string(),
            DATA::POINTER(p) => p.to_string(),
            DATA::String(s) => s.clone(),
        }
    }

    fn fancy_string(&self) -> String {
        match self {
            DATA::Number(n) => n.to_string(),
            DATA::Bool(b) => {
                if *b {
                    "true".to_string()
                } else {
                    "false".to_string()
                }
            }
            DATA::POINTER(p) => format!("&{}", p),
            DATA::String(s) => format!("\"{}\"", s),
        }
    }

    fn to_i32(&self) -> i32 {
        match self {
            DATA::Number(n) => *n,
            DATA::Bool(b) => {
                if *b {
                    1
                } else {
                    0
                }
            }
            DATA::POINTER(p) => *p,
            DATA::String(_s) => panic!("Cannot convert String to i32"),
        }
    }

    pub fn from_string(s: String) -> DATA {
        if s.starts_with("\"") && s.ends_with("\"") {
            DATA::String(s[1..s.len() - 1].to_string())
        } else if s.starts_with("&") {
            DATA::POINTER(s[1..].parse::<i32>().unwrap())
        } else if s == "true" {
            DATA::Bool(true)
        } else if s == "false" {
            DATA::Bool(false)
        } else {
            DATA::Number(s.parse::<i32>().unwrap())
        }
    }

    fn get(&self, int: &mut Interpreter) -> DATA {
        match self {
            DATA::POINTER(p) => int
                .data
                .get(p)
                .expect(&format!("NULLPOINTER:{}", p))
                .clone()
                .get(int),
            _ => self.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Operation {
    POINT { name: DATA },
    SET { name: DATA, value: DATA },
    JUMP { name: DATA },
    PRINT { value: DATA },
    CALL { name: DATA },
    RET,
    HALT,
    JUMP_IF { name: DATA, condition: DATA },
    EQ { name: DATA, value: DATA, ret: DATA },
    GREATER { name: DATA, value: DATA, ret: DATA },
    NOT { name: DATA, ret: DATA },
    ADD { name: DATA, value: DATA, ret: DATA },
    SUB { name: DATA, value: DATA, ret: DATA },
    MUL { name: DATA, value: DATA, ret: DATA },
    DIV { name: DATA, value: DATA, ret: DATA },
    NOP,
    DROP { name: DATA },
}

pub struct Interpreter {
    pub data: HashMap<i32, DATA>,
    pub points: HashMap<i32, usize>,
    pub programms: Vec<Operation>,
    pub call_stack: Vec<usize>,
}

impl Interpreter {
    pub fn run(p: Vec<Operation>) {
        let mut interpreter = Interpreter::new();
        interpreter.programms = p;
        interpreter.call_stack.push(0);
        let mut pos = 0;
        interpreter.programms = interpreter
            .programms
            .iter()
            .filter(|x| match x {
                Operation::POINT { name } => {
                    interpreter.points.insert(name.to_i32(), pos);
                    pos += 1;
                    false
                }
                _ => {
                    pos += 1;
                    true
                }
            })
            .map(|x| x.clone())
            .collect();
        while interpreter.tick() {}
    }

    pub fn new() -> Interpreter {
        Interpreter {
            data: HashMap::new(),
            points: HashMap::new(),
            programms: Vec::new(),
            call_stack: Vec::new(),
        }
    }

    pub fn continues_run(&mut self, p: Vec<Operation>) {
        self.programms = p;
        self.call_stack.push(0);
        let mut pos = 0;
        self.programms = self
            .programms
            .iter()
            .filter(|x| match x {
                Operation::POINT { name } => {
                    self.points.insert(name.to_i32(), pos);
                    pos += 1;
                    false
                }
                _ => {
                    pos += 1;
                    true
                }
            })
            .map(|x| x.clone())
            .collect();
        while self.tick() {}
    }

    pub fn tick(&mut self) -> bool {
        let index = self.call_stack.pop().unwrap();
        if index > self.programms.len() - 1 {
            return false;
        }
        let operation = self.programms[index].clone();
        self.call_stack.push(index + 1);
        match operation {
            Operation::DROP { name } => {
                let pos = name.get(self).to_i32();
                self.data.remove(&pos);
                true
            }
            Operation::POINT { name } => {
                let pos = name.get(self).to_i32();
                self.points.insert(pos, index);
                true
            }
            Operation::SET { name, value } => {
                let value = value.get(self);
                let pos = name.get(self).to_i32();
                self.data.insert(pos, value);
                true
            }
            Operation::JUMP { name } => {
                let pos = name.get(self).to_i32();
                self.call_stack.pop();
                self.call_stack.push(self.points.get(&pos).unwrap().clone());
                true
            }
            Operation::PRINT { value } => {
                println!("{}", value.get(self).to_string());
                true
            }
            Operation::GREATER { name, value, ret } => {
                let value = value.get(self).to_i32();
                let name = name.get(self).to_i32();
                let ret = ret.get(self).to_i32();
                self.data.insert(ret, DATA::Bool(name > value));
                true
            }
            Operation::CALL { name } => {
                let pos = name.get(self).to_i32();
                self.call_stack.push(self.points.get(&pos).unwrap().clone());
                true
            }
            Operation::JUMP_IF { name, condition } => {
                if condition.get(self).to_i32() == 1 {
                    let pos = name.get(self).to_i32();
                    self.call_stack.pop();
                    self.call_stack.push(self.points.get(&pos).unwrap().clone());
                }
                true
            }
            Operation::EQ { name, value, ret } => {
                let value = value.get(self).to_i32();
                let name = name.get(self).to_i32();
                let ret = ret.get(self).to_i32();
                self.data.insert(ret, DATA::Bool(name == value));
                true
            }
            Operation::NOT { name, ret } => {
                let name = name.get(self).to_i32();
                let ret = ret.get(self).to_i32();
                self.data.insert(ret, DATA::Bool(name == 0));
                true
            }
            Operation::ADD { name, value, ret } => {
                let value = value.get(self).to_i32();
                let name = name.get(self).to_i32();
                let ret = ret.get(self).to_i32();
                self.data.insert(ret, DATA::Number(name + value));
                true
            }
            Operation::SUB { name, value, ret } => {
                let value = value.get(self).to_i32();
                let name = name.get(self).to_i32();
                let ret = ret.get(self).to_i32();
                self.data.insert(ret, DATA::Number(name - value));
                true
            }
            Operation::MUL { name, value, ret } => {
                let value = value.get(self).to_i32();
                let name = name.get(self).to_i32();
                let ret = ret.get(self).to_i32();
                self.data.insert(ret, DATA::Number(name * value));
                true
            }
            Operation::DIV { name, value, ret } => {
                let value = value.get(self).to_i32();
                let name = name.get(self).to_i32();
                let ret = ret.get(self).to_i32();
                self.data.insert(ret, DATA::Number(name / value));
                true
            }
            Operation::RET => {
                self.call_stack.pop();
                true
            }
            Operation::HALT => false,
            Operation::NOP => true,
        }
    }
}

pub mod easy {
    use super::Operation;
    use super::DATA;
    pub fn set(value: String, name: String) -> Operation {
        Operation::SET {
            name: DATA::from_string(name.to_string()),
            value: DATA::from_string(value.to_string()),
        }
    }
    pub fn point(name: String) -> Operation {
        Operation::POINT {
            name: DATA::from_string(name.to_string()),
        }
    }
    pub fn jump(name: String) -> Operation {
        Operation::JUMP {
            name: DATA::from_string(name.to_string()),
        }
    }
    pub fn print(name: String) -> Operation {
        Operation::PRINT {
            value: DATA::from_string(name.to_string()),
        }
    }
    pub fn call(name: String) -> Operation {
        Operation::CALL {
            name: DATA::from_string(name.to_string()),
        }
    }
    pub fn ret() -> Operation {
        Operation::RET
    }
    pub fn halt() -> Operation {
        Operation::HALT
    }
    pub fn jump_if(name: String, condition: String) -> Operation {
        Operation::JUMP_IF {
            name: DATA::from_string(name.to_string()),
            condition: DATA::from_string(condition.to_string()),
        }
    }
    pub fn eq(name: String, value: String, ret: String) -> Operation {
        Operation::EQ {
            name: DATA::from_string(name.to_string()),
            value: DATA::from_string(value.to_string()),
            ret: DATA::from_string(ret.to_string()),
        }
    }
    pub fn not(name: String, ret: String) -> Operation {
        Operation::NOT {
            name: DATA::from_string(name.to_string()),
            ret: DATA::from_string(ret.to_string()),
        }
    }
    pub fn add(name: String, value: String, ret: String) -> Operation {
        Operation::ADD {
            name: DATA::from_string(name.to_string()),
            value: DATA::from_string(value.to_string()),
            ret: DATA::from_string(ret.to_string()),
        }
    }
    pub fn sub(name: String, value: String, ret: String) -> Operation {
        Operation::SUB {
            name: DATA::from_string(name.to_string()),
            value: DATA::from_string(value.to_string()),
            ret: DATA::from_string(ret.to_string()),
        }
    }
    pub fn mul(name: String, value: String, ret: String) -> Operation {
        Operation::MUL {
            name: DATA::from_string(name.to_string()),
            value: DATA::from_string(value.to_string()),
            ret: DATA::from_string(ret.to_string()),
        }
    }
    pub fn div(name: String, value: String, ret: String) -> Operation {
        Operation::DIV {
            name: DATA::from_string(name.to_string()),
            value: DATA::from_string(value.to_string()),
            ret: DATA::from_string(ret.to_string()),
        }
    }
    pub fn from_string(s: String) -> Vec<Operation> {
        s.split(";")
            .map(|s| {
                //remove trainling and starting whitespaces
                let mut name: Vec<String> = s.split(" ").map(|s| s.to_string()).collect();
                while name.get(0).unwrap_or(&"".to_string()) == &"".to_string()
                    || name.get(0).unwrap_or(&"".to_string()) == &"\n".to_string()
                {
                    if name.len() <= 1 {
                        break;
                    }
                    name.remove(0);
                }
                if name[0].starts_with("//") {
                    return Operation::NOP;
                }
                match name[0].as_str() {
                    "set" => set(name[2].clone(), name[1].clone()),
                    "point" => point(name[1].clone()),
                    "jump" => jump(name[1].clone()),
                    "print" => print(name[1].clone()),
                    "call" => call(name[1].clone()),
                    "ret" => ret(),
                    "halt" => halt(),
                    "jump_if" => jump_if(name[1].clone(), name[2].clone()),
                    "eq" => eq(name[1].clone(), name[2].clone(), name[3].clone()),
                    "not" => not(name[1].clone(), name[2].clone()),
                    "add" => add(name[1].clone(), name[2].clone(), name[3].clone()),
                    "sub" => sub(name[1].clone(), name[2].clone(), name[3].clone()),
                    "mul" => mul(name[1].clone(), name[2].clone(), name[3].clone()),
                    "div" => div(name[1].clone(), name[2].clone(), name[3].clone()),
                    "" => Operation::NOP,
                    _ => panic!("Unknown Operation: {}", name[0]),
                }
            })
            .collect()
    }
    pub fn run(s: String) {
        let operations = from_string(s);
        super::Interpreter::run(operations);
    }
    pub fn fancy_string(operations: Vec<Operation>) -> String {
        operations
            .iter()
            .map(|x| match x {
                Operation::DROP { name } => {
                    format!("drop {};", name.fancy_string())
                }
                Operation::GREATER { name, value, ret } => {
                    format!(
                        "greater {} {} {};",
                        name.fancy_string(),
                        value.fancy_string(),
                        ret.fancy_string()
                    )
                }
                Operation::HALT => "halt;".to_string(),
                Operation::ADD { name, value, ret } => {
                    format!(
                        "add {} {} {};",
                        name.fancy_string(),
                        value.fancy_string(),
                        ret.fancy_string()
                    )
                }
                Operation::SUB { name, value, ret } => {
                    format!(
                        "sub {} {} {};",
                        name.fancy_string(),
                        value.fancy_string(),
                        ret.fancy_string()
                    )
                }
                Operation::MUL { name, value, ret } => {
                    format!(
                        "mul {} {} {};",
                        name.fancy_string(),
                        value.fancy_string(),
                        ret.fancy_string()
                    )
                }
                Operation::DIV { name, value, ret } => {
                    format!(
                        "div {} {} {};",
                        name.fancy_string(),
                        value.fancy_string(),
                        ret.fancy_string()
                    )
                }
                Operation::NOT { name, ret } => {
                    format!("not {} {};", name.fancy_string(), ret.fancy_string())
                }
                Operation::EQ { name, value, ret } => {
                    format!(
                        "eq {} {} {};",
                        name.fancy_string(),
                        value.fancy_string(),
                        ret.fancy_string()
                    )
                }
                Operation::JUMP_IF { name, condition } => {
                    format!(
                        "jump_if {} {};",
                        name.fancy_string(),
                        condition.fancy_string()
                    )
                }
                Operation::SET { name, value } => {
                    format!("set {} {};", name.fancy_string(), value.fancy_string())
                }
                Operation::POINT { name } => {
                    format!("point {};", name.fancy_string())
                }
                Operation::JUMP { name } => {
                    format!("jump {};", name.fancy_string())
                }
                Operation::PRINT { value } => {
                    format!("print {};", value.fancy_string())
                }
                Operation::CALL { name } => {
                    format!("call {};", name.fancy_string())
                }
                Operation::RET => "ret;".to_string(),
                Operation::NOP => "NOP;".to_string(),
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
}
