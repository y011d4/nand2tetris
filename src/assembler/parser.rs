use std::fs::File;
use std::io;
use std::io::prelude::*;

pub enum Command {
    ACommand,
    CCommand,
    LCommand,
}

pub struct Parser {
    asm: Vec<String>,
    current: usize,
    code: String,
}

impl Parser {
    pub fn new(asm_path: &str) -> io::Result<Parser> {
        let f = File::open(asm_path)?;
        let reader = io::BufReader::new(f);
        let asm: Vec<String> = reader
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
            asm: asm,
            current: 0,
            code: "".to_string(),
        })
    }

    pub fn has_more_commands(&self) -> bool {
        self.current < self.asm.len()
    }

    pub fn advance(&mut self) {
        assert!(self.has_more_commands());
        self.code = self.asm[self.current].clone();
        self.current += 1;
    }

    pub fn command_type(&self) -> Command {
        if self.code.starts_with("@") {
            Command::ACommand
        } else if self.code.starts_with("(") && self.code.ends_with(")") {
            Command::LCommand
        } else {
            Command::CCommand
        }
    }

    pub fn symbol(&self) -> Result<String, String> {
        match self.command_type() {
            Command::ACommand => Ok(self.code[1..].to_string()),
            Command::LCommand => Ok(self.code[1..self.code.len() - 1].to_string()),
            Command::CCommand => Err("Command type should be A_COMMAND or L_COMMAND.".to_string()),
        }
    }

    pub fn dest(&self) -> Option<String> {
        self.decompose_c_command().0
    }

    pub fn comp(&self) -> Option<String> {
        self.decompose_c_command().1
    }

    pub fn jump(&self) -> Option<String> {
        self.decompose_c_command().2
    }

    fn decompose_c_command(&self) -> (Option<String>, Option<String>, Option<String>) {
        let dest: Option<String>;
        let comp: Option<String>;
        let jump: Option<String>;
        let mut tmp_code = self.code.clone();
        match tmp_code.find("=") {
            Some(equal_index) => {
                dest = Some(self.code[0..equal_index].to_string());
                tmp_code = tmp_code[equal_index + 1..].to_string();
            }
            None => dest = None,
        }
        match tmp_code.find(";") {
            Some(semicolon_index) => {
                comp = Some(tmp_code[0..semicolon_index].to_string());
                tmp_code = tmp_code[semicolon_index + 1..].to_string();
                jump = Some(tmp_code);
            }
            None => {
                comp = Some(tmp_code);
                jump = None;
            }
        }
        (dest, comp, jump)
    }
}
