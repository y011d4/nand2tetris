use std::fs::File;
use std::io;
use std::io::prelude::*;

#[derive(Debug)]
pub enum Command {
    Arithmetic,
    Push,
    Pop,
    Label,
    Goto,
    If,
    Function,
    Return,
    Call,
}

pub struct Parser {
    vm: Vec<String>,
    current: usize,
    code: String,
}

impl Parser {
    pub fn new(vm_path: &str) -> io::Result<Parser> {
        let f = File::open(vm_path)?;
        let reader = io::BufReader::new(f);
        let vm: Vec<String> = reader
            .lines()
            .filter_map(|l| {
                let mut line = l.unwrap();
                line = line.trim().to_string();
                line = match line.find("//") {
                    Some(index) => (&line[..index]).trim().to_string(),
                    None => line,
                };
                if line.len() > 0 {
                    Some(line)
                } else {
                    None
                }
            })
            .collect();
        Ok(Parser {
            vm: vm,
            current: 0,
            code: "".to_string(),
        })
    }

    pub fn has_more_commands(&self) -> bool {
        self.current < self.vm.len()
    }

    pub fn advance(&mut self) {
        assert!(self.has_more_commands());
        self.code = self.vm[self.current].clone();
        self.current += 1;
    }

    pub fn command_type(&self) -> Command {
        let command = self.code.split_whitespace().nth(0).unwrap();
        match command {
            "push" => Command::Push,
            "pop" => Command::Pop,
            "label" => Command::Label,
            "goto" => Command::Goto,
            "if-goto" => Command::If,
            "function" => Command::Function,
            "call" => Command::Call,
            "return" => Command::Return,
            _ => Command::Arithmetic,
        }
    }

    pub fn arg1(&self) -> Result<String, String> {
        match self.command_type() {
            Command::Return => Err("Command type should not be C_RETURN.".to_string()),
            _ => Ok(self.code.split_whitespace().nth(1).unwrap().to_string()),
        }
    }

    pub fn arg2(&self) -> Result<String, String> {
        match self.command_type() {
            Command::Push | Command::Pop | Command::Function | Command::Call => Ok(self.code.split_whitespace().nth(2).unwrap().to_string()),
            _ => Err(format!("command type {:?} doesn't have two arguments.", self.command_type())),
        }
    }

    pub fn command(&self) -> String {
        self.code.split_whitespace().nth(0).unwrap().to_string()
    }
}
