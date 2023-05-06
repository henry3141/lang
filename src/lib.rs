#![feature(let_chains)]
#![allow(non_camel_case_types)]
use std::io::Read;
pub mod compiler;
pub mod interpreter;
pub mod parser;
pub mod timer;

pub struct Lang {
    pub compiler: compiler::Compiler,
    pub interpreter: interpreter::Interpreter,
    pub debug: bool,
}

impl Lang {
    pub fn new() -> Lang {
        Lang {
            compiler: compiler::Compiler::continues(),
            interpreter: interpreter::Interpreter::new(),
            debug: false,
        }
    }

    pub fn run(file_name: String) {
        let mut f = std::fs::File::open(file_name).unwrap();
        let mut s = String::new();
        f.read_to_string(&mut s).unwrap();
        let mut lang = Lang::new();
        lang.continues(s);
    }

    pub fn continues(&mut self, s: String) {
        let s = parser::parse(s);
        if self.debug {
            println!("====================");
        }
        let s = self.compiler.continues_compile(s);
        if self.debug {
            println!("{}", interpreter::easy::fancy_string(s.clone()));
            println!("====================");
        }
        self.interpreter.continues_run(s);
    }
}
