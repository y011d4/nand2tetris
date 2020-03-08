use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

use crate::vm_translator::parser::{Command, Parser};

pub struct Writer {
    vm_path: String,
    parser: Parser,
    writer: io::BufWriter<File>,
    n_eq: usize,
    n_gt: usize,
    n_lt: usize,
}

impl Writer {
    pub fn new(vm_path: &str, asm_path: &str) -> Self {
        let parser = Parser::new(vm_path).unwrap();
        let f = File::create(asm_path).unwrap();
        let writer = io::BufWriter::new(f);
        Self {
            vm_path: vm_path.to_string(),
            parser: parser,
            writer: writer,
            n_eq: 0,
            n_gt: 0,
            n_lt: 0,
        }
    }

    pub fn set_file_name(&self) -> String {
        let path = Path::new(self.vm_path.as_str());
        path.file_stem().unwrap().to_string_lossy().to_string()
    }

    pub fn write_arithmetic(&mut self, command: String) -> Result<(), String> {
        let code = match command.as_str() {
            "add" => Some("@SP\nAM=M-1\nD=M\n@SP\nAM=M-1\nM=D+M\n@SP\nM=M+1\n".to_string()),
            "sub" => Some("@SP\nAM=M-1\nD=M\n@SP\nAM=M-1\nM=M-D\n@SP\nM=M+1\n".to_string()),
            "neg" => Some("@SP\nAM=M-1\nM=-M\n@SP\nM=M+1\n".to_string()),
            "eq"  => {
                self.n_eq += 1;
                Some(format!("@SP\nAM=M-1\nD=M\n@SP\nAM=M-1\nD=M-D\n@EQ.IF.{}\nD;JEQ\nD=0\n@EQ.ENDIF.{}\n0;JMP\n(EQ.IF.{})\nD=-1\n(EQ.ENDIF.{})\n@SP\nA=M\nM=D\n@SP\nM=M+1\n", self.n_eq, self.n_eq, self.n_eq, self.n_eq))
            },
            "gt"  => {
                self.n_gt += 1;
                Some(format!("@SP\nAM=M-1\nD=M\n@SP\nAM=M-1\nD=M-D\n@GT.IF.{}\nD;JGT\nD=0\n@GT.ENDIF.{}\n0;JMP\n(GT.IF.{})\nD=-1\n(GT.ENDIF.{})\n@SP\nA=M\nM=D\n@SP\nM=M+1\n", self.n_gt, self.n_gt, self.n_gt, self.n_gt))
            },
            "lt"  => {
                self.n_lt += 1;
                Some(format!("@SP\nAM=M-1\nD=M\n@SP\nAM=M-1\nD=M-D\n@LT.IF.{}\nD;JLT\nD=0\n@LT.ENDIF.{}\n0;JMP\n(LT.IF.{})\nD=-1\n(LT.ENDIF.{})\n@SP\nA=M\nM=D\n@SP\nM=M+1\n", self.n_lt, self.n_lt, self.n_lt, self.n_lt))
            },
            "and" => Some("@SP\nAM=M-1\nD=M\n@SP\nAM=M-1\nM=D&M\n@SP\nM=M+1\n".to_string()),
            "or" => Some("@SP\nAM=M-1\nD=M\n@SP\nAM=M-1\nM=D|M\n@SP\nM=M+1\n".to_string()),
            "not" => Some("@SP\nAM=M-1\nM=!M\n@SP\nM=M+1\n".to_string()),
            _ => None,
        };
        self.writer.write(format!("// {}\n", command).as_bytes()).unwrap();
        match code {
            Some(code) => {
                self.writer.write(code.as_bytes()).unwrap();
                Ok(())
            }
            None => Err(format!("Undefined command `{}`.", command)),
        }
    }

    pub fn write_push_pop(
        &mut self,
        command: String,
        segment: String,
        index: usize,
    ) -> Result<(), String> {
        // R13 is used for temporal register containing address to be referenced.
        // R14 is used for `static` or `const` variable.
        let address = match segment.as_str() {
            "argument" => Some(format!("@{}\nD=A\n@ARG\nD=D+M\n@R13\nM=D\n", index)),
            "local"    => Some(format!("@{}\nD=A\n@LCL\nD=D+M\n@R13\nM=D\n", index)),
            "static"   => Some(format!("@{}.{}\nD=A\n@R13\nM=D\n", self.set_file_name(), index)),
            "constant" => Some(format!("@{}\nD=A\n@R14\nM=D\nD=A\n@R13\nM=D\n", index)),
            "this"     => Some(format!("@{}\nD=A\n@THIS\nD=D+M\n@R13\nM=D\n", index)),
            "that"     => Some(format!("@{}\nD=A\n@THAT\nD=D+M\n@R13\nM=D\n", index)),
            "pointer"  => Some(format!("@{}\nD=A\n@3\nD=D+A\n@R13\nM=D\n", index)),
            "temp"     => Some(format!("@{}\nD=A\n@5\nD=D+A\n@R13\nM=D\n", index)),
            _ => None
        };
        let code = match command.as_str() {
            "push" => match address {
                Some(address) => Some(format!("{}@R13\nA=M\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n", address)),
                None => None,
            },
            "pop"  => match address {
                Some(address) => Some(format!("{}@SP\nAM=M-1\nD=M\n@R13\nA=M\nM=D\n", address)),
                None => None,
            }
            _ => None
        };
        self.writer.write(format!("// {} {} {}\n", command, segment, index).as_bytes()).unwrap();
        match code {
            Some(code) => {
                self.writer.write(code.as_bytes()).unwrap();
                Ok(())
            }
            None => Err(format!("Undefined command `{} {} {}`.", command, segment, index)),
        }
    }

    pub fn write(&mut self) {
        loop {
            if !self.parser.has_more_commands() {
                break;
            }
            self.parser.advance();
            match self.parser.command_type() {
                Command::Push => { self.write_push_pop("push".to_string(), self.parser.arg1().unwrap(), self.parser.arg2().unwrap().parse().unwrap()).unwrap(); },
                Command::Pop => { self.write_push_pop("pop".to_string(), self.parser.arg1().unwrap(), self.parser.arg2().unwrap().parse().unwrap()).unwrap(); },
                Command::Arithmetic => { self.write_arithmetic(self.parser.command()).unwrap(); },
                _ => {},
            }
        }
    }

    /*
    pub fn write(asm_path: &str, hack_path: &str) {
        let mut parser = Parser::new(asm_path).unwrap();
        let mut symbol_table = make_symbol_table(&mut parser);

        let mut parser = Parser::new(asm_path).unwrap();
        let f = File::create(hack_path).unwrap();
        let mut writer = io::BufWriter::new(f);
        let mut n_var = 0;
        loop {
            if !parser.has_more_commands() {
                break;
            }
            parser.advance();
            let byte = match parser.command_type() {
                Command::CCommand => Some(format!(
                    "111{}{}{}\n",
                    Code::new(parser.comp()).comp().unwrap(),
                    Code::new(parser.dest()).dest().unwrap(),
                    Code::new(parser.jump()).jump().unwrap()
                )),
                Command::ACommand => {
                    let symbol = parser.symbol().unwrap();
                    if symbol.chars().nth(0).unwrap().is_digit(10) {
                        Some(format!("{:0>16b}\n", symbol.parse::<i64>().unwrap()))
                    } else {
                        let address = if symbol_table.contains(&symbol) {
                            symbol_table.get_address(&symbol)
                        } else {
                            symbol_table.add_entry(symbol.clone(), n_var + 16);
                            n_var += 1;
                            symbol_table.get_address(&symbol)
                        };
                        Some(format!("{:0>16b}\n", address))
                    }
                }
                Command::LCommand => None,
            };
            match byte {
                Some(byte) => {
                    writer.write(byte.as_bytes()).unwrap();
                }
                None => {}
            }
        }
    }
        */
}
