use lang::Lang;
use std::env;
use std::io::Read;
use std::{fs, io::Write};

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() == 2 {
        Lang::run(args[1].clone());
        return;
    }
    if args.len() == 3 {
        if args[1] == "-d" {
            let mut lang = Lang::new();
            lang.debug = true;
            let mut f = fs::File::open(args[2].clone()).unwrap();
            let mut s = String::new();
            f.read_to_string(&mut s).unwrap();
            lang.continues(s);
            return;
        }
        if args[1] == "-c" {
            let mut s = String::new();
            let mut f = fs::File::open(args[2].clone()).unwrap();
            f.read_to_string(&mut s).unwrap();
            let s = lang::parser::parse(s);
            println!("====================");
            println!("{:#?}", s);
            let s = lang::compiler::Compiler::compile(s);
            println!("====================");
            println!("{}", lang::interpreter::easy::fancy_string(s.clone()));
            println!("====================");
            lang::interpreter::Interpreter::run(s);
            return;
        }
        return;
    }
    let mut lang = Lang::new();
    lang.debug = true;
    let mut history: Vec<String> = vec![];
    loop {
        let mut s = String::new();
        print!(">>> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut s).unwrap();
        let mut s = s.trim().to_string();
        if s == "exit" {
            break;
        }
        if s == "reset" {
            history = vec![];
            lang = Lang::new();
            lang.debug = true;
            continue;
        }
        if s.starts_with("get") {
            let s2 = s.split(" ").collect::<Vec<&str>>();
            if s2.len() != 2 {
                println!("get <history>");
                continue;
            }
            let index = s2[1].parse::<usize>().unwrap();
            //from back
            let index = history.len() - index;
            if index >= history.len() {
                println!("index out of bounds");
                continue;
            }
            s = history[index].clone();
            println!("|-> {}", s);
        }
        if s == "list" {
            for (i, s) in history.iter().enumerate() {
                println!("|{}|->{}", history.len() - i, s);
            }
            continue;
        }
        if s == "help" {
            println!("exit: exit the program");
            println!("reset: reset the program");
            println!("get <history>: get a program from history");
            println!("list: list history");
            println!("save: save history to hs.txt");
            println!("load: load history from hs.txt");
            println!("execute: execute hs.txt");
            println!("phelp: print program help message");
            println!("help: print this message");
            continue;
        }
        if s == "save" {
            //save history to hs.txt
            let mut f = fs::File::create("hs.lang").unwrap();
            for s in history.iter() {
                f.write_all(s.as_bytes()).unwrap();
                f.write_all(b"\n").unwrap();
            }
            continue;
        }
        if s == "load" {
            //load history from hs.txt
            let s = fs::read_to_string("hs.lang").unwrap();
            let mut s = s
                .split("\n")
                .collect::<Vec<&str>>()
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>();
            s.pop();
            history = s;
            lang = Lang::new();
            lang.debug = true;
            //execute history
            for s in history.iter() {
                println!("|-> {}", s);
                lang.continues(s.clone());
            }
            continue;
        }
        if s == "execute" {
            //execute hs.txt
            let s = fs::read_to_string("hs.lang").unwrap();
            let mut s = s
                .split("\n")
                .collect::<Vec<&str>>()
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>();
            s.pop();
            history.extend(s.clone());
            for s in s.iter() {
                println!("|-> {}", s);
                lang.continues(s.clone());
            }
            continue;
        }
        if s == "phelp" {
            println!("print <value>: print a value");
            println!("let <name> = <value>: create a variable");
            println!("<name> = <value>: set var to values");
            println!("if <condition> {{<code>}}: if condition is true, execute code");
            println!("while <condition> {{<code>}}: while condition is true, execute code");
            println!("def <name> (<args>) {{<code>}}: create a function");
            continue;
        }
        if !s.ends_with(";") {
            s.push(';');
        }
        history.push(s.clone());
        lang.continues(s);
    }
}
