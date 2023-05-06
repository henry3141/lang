use std::str::Chars;

use crate::compiler::*;

pub fn to_expression(input: String) -> Option<Expression> {
    if smart_contain(input.clone(), '(') {
        let pos = input.find('(').unwrap();
        let name = input[0..pos].to_string();
        let args = smart_split(input[pos + 1..input.len() - 1].to_string(), ',')
            .iter()
            .map(|x| to_expression(x.clone()))
            .collect::<Option<Vec<Expression>>>();
        if let Some(args) = args {
            return Some(Expression::CALL(name, args));
        }
    } else if input.starts_with("+") {
        let split = smart_split(input[1..].to_string(), ',');
        if split.len() == 2 {
            let left = to_expression(split[0].clone());
            let right = to_expression(split[1].clone());
            if let (Some(left), Some(right)) = (left, right) {
                return Some(Expression::Add(Box::new(left), Box::new(right)));
            }
        }
    } else if input.starts_with("-") {
        let split = smart_split(input[1..].to_string(), ',');
        if split.len() == 2 {
            let left = to_expression(split[0].clone());
            let right = to_expression(split[1].clone());
            if let (Some(left), Some(right)) = (left, right) {
                return Some(Expression::Sub(Box::new(left), Box::new(right)));
            }
        }
    } else if input.starts_with("*") {
        let split = smart_split(input[1..].to_string(), ',');
        if split.len() == 2 {
            let left = to_expression(split[0].clone());
            let right = to_expression(split[1].clone());
            if let (Some(left), Some(right)) = (left, right) {
                return Some(Expression::Mul(Box::new(left), Box::new(right)));
            }
        }
    } else if input.starts_with("/") {
        let split = smart_split(input[1..].to_string(), ',');
        if split.len() == 2 {
            let left = to_expression(split[0].clone());
            let right = to_expression(split[1].clone());
            if let (Some(left), Some(right)) = (left, right) {
                return Some(Expression::Div(Box::new(left), Box::new(right)));
            }
        }
    } else if input.starts_with(">") {
        let split = smart_split(input[1..].to_string(), ',');
        if split.len() == 2 {
            let left = to_expression(split[0].clone());
            let right = to_expression(split[1].clone());
            if let (Some(left), Some(right)) = (left, right) {
                return Some(Expression::GREATER(Box::new(left), Box::new(right)));
            }
        }
    } else if input.starts_with("==") {
        let split = smart_split(input[2..].to_string(), ',');
        if split.len() == 2 {
            let left = to_expression(split[0].clone());
            let right = to_expression(split[1].clone());
            if let (Some(left), Some(right)) = (left, right) {
                return Some(Expression::EQ(Box::new(left), Box::new(right)));
            }
        }
    } else if input.starts_with("!") {
        let data = to_expression(input[1..].to_string());
        if let Some(data) = data {
            return Some(Expression::NOT(Box::new(data)));
        }
    } else if let Ok(i) = input.clone().parse::<i32>() {
        return Some(Expression::Number(i));
    } else if input == "false" {
        return Some(Expression::Bool(false));
    } else if input == "true" {
        return Some(Expression::Bool(true));
    } else if input.starts_with("\"") && input.ends_with("\"") {
        return Some(Expression::String(input[1..input.len() - 1].to_string()));
    } else {
        return Some(Expression::Variable(input));
    }
    todo!("Expression:{}", input);
}

fn smart_contain(s: String, c: char) -> bool {
    let mut chars = s.chars();
    let exit = vec!['"'];
    while let Some(n) = chars.next() {
        if n == c {
            return true;
        }
        if exit.contains(&n) {
            return false;
        }
    }
    false
}

fn smart_split(s: String, c: char) -> Vec<String> {
    let mut string = 0;
    let mut chars = s.chars();
    let mut out = vec![];
    let mut args = 0;
    let mut current = String::new();
    while let Some(n) = chars.next() {
        if n == c && string == 0 && args == 0 {
            out.push(current.clone());
            current = String::new();
        } else if n == '"' {
            if string > 0 {
                string -= 1;
            } else {
                string += 1;
            }
        } else if n == '(' {
            args += 1;
        } else if n == ')' {
            args -= 1;
        } else {
            current.push(n);
        }
    }
    out.push(current.clone());
    out
}

#[derive(Debug, Clone)]
pub enum Block {
    Block(Vec<Block>),
    String(String),
}

impl Block {
    pub fn into_vec(&self) -> Option<Vec<Block>> {
        if let Block::Block(b) = self {
            return Some(b.clone());
        }
        None
    }
}

pub fn to_block(s: &mut Chars) -> Block {
    let mut current = vec![Block::String(String::new())];
    while let Some(n) = s.next() {
        match n {
            '{' => {
                current.push(to_block(s));
            }
            '}' => {
                let current = current
                    .into_iter()
                    .filter(|x| {
                        if let Block::String(s) = x {
                            if s.len() == 0 {
                                return false;
                            }
                        }
                        true
                    })
                    .collect::<Vec<Block>>();
                return Block::Block(current);
            }
            ';' => {
                current.push(Block::String(String::new()));
            }
            _ => {
                if let Block::String(s) = current.last_mut().unwrap() {
                    s.push(n);
                } else {
                    current.push(Block::String(n.to_string()));
                }
            }
        }
    }
    //remove all Block::String(String::new())
    let current = current
        .into_iter()
        .filter(|x| {
            if let Block::String(s) = x {
                if s.len() == 0 {
                    return false;
                }
            }
            true
        })
        .collect::<Vec<Block>>();
    Block::Block(current)
}

pub fn smart_trim(s: String) -> String {
    let mut in_string = false;
    let mut out = String::new();
    for n in s.chars() {
        if n == '"' {
            in_string = !in_string;
        }
        if n == ' ' && !in_string {
            continue;
        }
        if n == '\n' && !in_string {
            continue;
        }
        out.push(n);
    }
    out
}

pub fn parse_block(b: Vec<Block>) -> Vec<Instruction> {
    let mut b = b.into_iter();
    let mut instructions = vec![];
    while let Some(block) = b.next() {
        match block {
            Block::Block(b) => {
                instructions.extend(parse_block(b));
            }
            Block::String(s) => {
                if s.starts_with("print") {
                    let s = s[5..].to_string();
                    instructions.push(Instruction::PRINT {
                        value: to_expression(s).unwrap(),
                    });
                } else if s.starts_with("let") {
                    let data = smart_split(s[3..].to_string(), '=');
                    if data.len() == 2 {
                        let name = data[0].clone();
                        let value = to_expression(data[1].clone());
                        if let Some(value) = value {
                            instructions.push(Instruction::SET { name, value });
                        }
                    }
                } else if s.starts_with("while") {
                    let cond = s[5..].to_string();
                    let cond = to_expression(cond).unwrap();
                    let instructions2 = parse_block(b.next().unwrap().into_vec().unwrap());
                    instructions.push(Instruction::WHILE {
                        condition: cond,
                        instruction: instructions2,
                    })
                } else if s.starts_with("if") {
                    let cond = s[2..].to_string();
                    let cond = to_expression(cond).unwrap();
                    let instructions2 = parse_block(b.next().unwrap().into_vec().unwrap());
                    instructions.push(Instruction::IF {
                        condition: cond,
                        instruction: instructions2,
                    })
                } else if s.contains("=") {
                    let data = smart_split(s, '=');
                    if data.len() == 2 {
                        let name = data[0].clone();
                        let value = to_expression(data[1].clone());
                        if let Some(value) = value {
                            instructions.push(Instruction::ASSIGN { name, value });
                        }
                    }
                } else {
                    todo!("Block::String:{}", s)
                }
            }
        }
    }
    instructions
}

pub fn parse(s: String) -> Vec<Instruction> {
    let s = smart_trim(s);
    let mut chars = s.chars();
    let block = vec![to_block(&mut chars)];
    parse_block(block)
}
