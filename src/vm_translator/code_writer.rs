use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

use crate::vm_translator::parser::{Command, Parser};

pub struct Writer {
    vm_path: String,
    writer: io::BufWriter<File>,
    n_eq: usize,
    n_gt: usize,
    n_lt: usize,
    n_call_func: HashMap<String, usize>,
}

impl Writer {
    pub fn new(asm_path: &str) -> Self {
        let f = File::create(asm_path).unwrap();
        //let f = OpenOptions::new().write(true).append(true).truncate(false).create(true).open(asm_path).unwrap();
        let writer = io::BufWriter::new(f);
        Self {
            vm_path: "init.vm".to_string(),
            writer: writer,
            n_eq: 0,
            n_gt: 0,
            n_lt: 0,
            n_call_func: HashMap::new(),
        }
    }

    pub fn set_file_name(&self) -> String {
        let path = Path::new(self.vm_path.as_str());
        path.file_stem().unwrap().to_string_lossy().to_string()
    }

    pub fn write_init(&mut self) {
        self.writer
            .write("@256\nD=A\n@SP\nM=D\n".as_bytes())
            .unwrap();
        self.write_call("Sys.init".to_string(), 0);
    }

    pub fn write_arithmetic(&mut self, command: String) -> Result<(), String> {
        let code = match command.as_str() {
            "add" => Some("@SP\nAM=M-1\nD=M\n@SP\nAM=M-1\nM=D+M\n@SP\nM=M+1\n".to_string()),
            "sub" => Some("@SP\nAM=M-1\nD=M\n@SP\nAM=M-1\nM=M-D\n@SP\nM=M+1\n".to_string()),
            "neg" => Some("@SP\nAM=M-1\nM=-M\n@SP\nM=M+1\n".to_string()),
            "eq" => {
                self.n_eq += 1;
                Some(format!("@SP\nAM=M-1\nD=M\n@SP\nAM=M-1\nD=M-D\n@EQ.IF.{}\nD;JEQ\nD=0\n@EQ.ENDIF.{}\n0;JMP\n(EQ.IF.{})\nD=-1\n(EQ.ENDIF.{})\n@SP\nA=M\nM=D\n@SP\nM=M+1\n", self.n_eq, self.n_eq, self.n_eq, self.n_eq))
            }
            "gt" => {
                self.n_gt += 1;
                Some(format!("@SP\nAM=M-1\nD=M\n@SP\nAM=M-1\nD=M-D\n@GT.IF.{}\nD;JGT\nD=0\n@GT.ENDIF.{}\n0;JMP\n(GT.IF.{})\nD=-1\n(GT.ENDIF.{})\n@SP\nA=M\nM=D\n@SP\nM=M+1\n", self.n_gt, self.n_gt, self.n_gt, self.n_gt))
            }
            "lt" => {
                self.n_lt += 1;
                Some(format!("@SP\nAM=M-1\nD=M\n@SP\nAM=M-1\nD=M-D\n@LT.IF.{}\nD;JLT\nD=0\n@LT.ENDIF.{}\n0;JMP\n(LT.IF.{})\nD=-1\n(LT.ENDIF.{})\n@SP\nA=M\nM=D\n@SP\nM=M+1\n", self.n_lt, self.n_lt, self.n_lt, self.n_lt))
            }
            "and" => Some("@SP\nAM=M-1\nD=M\n@SP\nAM=M-1\nM=D&M\n@SP\nM=M+1\n".to_string()),
            "or" => Some("@SP\nAM=M-1\nD=M\n@SP\nAM=M-1\nM=D|M\n@SP\nM=M+1\n".to_string()),
            "not" => Some("@SP\nAM=M-1\nM=!M\n@SP\nM=M+1\n".to_string()),
            _ => None,
        };
        self.writer
            .write(format!("// {}\n", command).as_bytes())
            .unwrap();
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
            "local" => Some(format!("@{}\nD=A\n@LCL\nD=D+M\n@R13\nM=D\n", index)),
            "static" => Some(format!(
                "@{}.{}\nD=A\n@R13\nM=D\n",
                self.set_file_name(),
                index
            )),
            "constant" => Some(format!("@{}\nD=A\n@R14\nM=D\nD=A\n@R13\nM=D\n", index)),
            "this" => Some(format!("@{}\nD=A\n@THIS\nD=D+M\n@R13\nM=D\n", index)),
            "that" => Some(format!("@{}\nD=A\n@THAT\nD=D+M\n@R13\nM=D\n", index)),
            "pointer" => Some(format!("@{}\nD=A\n@3\nD=D+A\n@R13\nM=D\n", index)),
            "temp" => Some(format!("@{}\nD=A\n@5\nD=D+A\n@R13\nM=D\n", index)),
            _ => None,
        };
        let code = match command.as_str() {
            "push" => match address {
                Some(address) => Some(format!(
                    "{}@R13\nA=M\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n",
                    address
                )),
                None => None,
            },
            "pop" => match address {
                Some(address) => Some(format!("{}@SP\nAM=M-1\nD=M\n@R13\nA=M\nM=D\n", address)),
                None => None,
            },
            _ => None,
        };
        self.writer
            .write(format!("// {} {} {}\n", command, segment, index).as_bytes())
            .unwrap();
        match code {
            Some(code) => {
                self.writer.write(code.as_bytes()).unwrap();
                Ok(())
            }
            None => Err(format!(
                "Undefined command `{} {} {}`.",
                command, segment, index
            )),
        }
    }

    pub fn write_label(&mut self, label: String) {
        let label = format!("({})\n", label);
        self.writer
            .write(format!("// label {}", label).as_bytes())
            .unwrap();
        self.writer.write(label.as_bytes()).unwrap();
    }

    pub fn write_goto(&mut self, label: String) {
        self.writer
            .write(format!("// goto {}\n", label).as_bytes())
            .unwrap();

        self.writer
            .write(format!("@{}\n0;JMP\n", label).as_bytes())
            .unwrap();
    }

    pub fn write_if(&mut self, label: String) {
        self.writer
            .write(format!("// if-goto {}\n", label).as_bytes())
            .unwrap();

        self.writer
            .write(format!("@SP\nAM=M-1\nD=M\n@{}\nD;JNE\n", label).as_bytes())
            .unwrap();
    }

    pub fn write_call(&mut self, function_name: String, num_args: usize) {
        self.writer
            .write(format!("// call {} {}\n", function_name, num_args).as_bytes())
            .unwrap();

        // let file_name = self.set_file_name();
        let n_call = self.n_call_func.entry(function_name.clone()).or_insert(0);
        *n_call += 1;
        // let return_address = format!("RETURN.{}.{}_at_{}", function_name, *n_call, file_name);
        let return_address = format!("RETURN.{}.{}", function_name, *n_call);
        self.writer
            .write(
                format!(
                    "{}{}{}{}{}{}{}{}{}",
                    format!("@{}\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n", return_address), // push return-address
                    "@LCL\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n",                         // push LCL
                    "@ARG\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n",                         // push ARG
                    "@THIS\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n",                        // push THIS
                    "@THAT\nD=M\n@SP\nA=M\nM=D\n@SP\nM=M+1\n",                        // push THAT
                    format!("@SP\nD=M\n@{}\nD=D-A\n@5\nD=D-A\n@ARG\nM=D\n", num_args), // ARG = SP-n-5
                    "@SP\nD=M\n@LCL\nM=D\n",                                           // LCL = SP
                    format!("@{}\n0;JMP\n", function_name),                            // goto f
                    format!("({})\n", return_address) // (return-address)
                )
                .as_bytes(),
            )
            .unwrap();
    }

    pub fn write_return(&mut self) {
        self.writer.write("// return\n".as_bytes()).unwrap();

        // R13 is used for temporal variable `FRAME`.
        // R14 is used for return address `RET`
        self.writer
            .write(
                format!(
                    "{}{}{}{}{}{}{}{}{}",
                    "@LCL\nD=M\n@R13\nM=D\n",                  // FRAME = LCL
                    "@5\nA=D-A\nD=M\n@R14\nM=D\n",             // RET = *(FRAME-5)
                    "@SP\nAM=M-1\nD=M\n@ARG\nA=M\nM=D\n",      // *ARG = pop()
                    "@ARG\nD=M+1\n@SP\nM=D\n",                 // SP = ARG+1
                    "@R13\nA=M-1\nD=M\n@THAT\nM=D\n",          // THAT = *(FRAME-1)
                    "@R13\nD=M\n@2\nA=D-A\nD=M\n@THIS\nM=D\n", // THIS = *(FRAME-2)
                    "@R13\nD=M\n@3\nA=D-A\nD=M\n@ARG\nM=D\n",  // ARG = *(FRAME-3)
                    "@R13\nD=M\n@4\nA=D-A\nD=M\n@LCL\nM=D\n",  // LCL = *(FRAME-4)
                    "@R14\nA=M\n0;JMP\n"                       // goto RET
                )
                .as_bytes(),
            )
            .unwrap();
    }

    pub fn write_function(&mut self, function_name: String, num_locals: usize) {
        self.writer
            .write(format!("// function {} {}\n", function_name, num_locals).as_bytes())
            .unwrap();
        let mut repeated_code = "".to_string();
        (0..num_locals).for_each(|_| repeated_code += "@0\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n");
        self.writer
            .write(
                format!(
                    "{}{}",
                    format!("({})\n", function_name), // (f)
                    repeated_code                     // repeat k times: push 0
                )
                .as_bytes(),
            )
            .unwrap();
    }

    pub fn write(&mut self, vm_path: &str) {
        self.vm_path = vm_path.to_string();
        let mut parser = Parser::new(vm_path).unwrap();
        loop {
            if !parser.has_more_commands() {
                break;
            }
            parser.advance();
            match parser.command_type() {
                Command::Push => {
                    self.write_push_pop(
                        "push".to_string(),
                        parser.arg1().unwrap(),
                        parser.arg2().unwrap().parse().unwrap(),
                    )
                    .unwrap();
                }
                Command::Pop => {
                    self.write_push_pop(
                        "pop".to_string(),
                        parser.arg1().unwrap(),
                        parser.arg2().unwrap().parse().unwrap(),
                    )
                    .unwrap();
                }
                Command::Arithmetic => {
                    self.write_arithmetic(parser.command()).unwrap();
                }
                Command::Label => {
                    self.write_label(format!(
                        "{}${}",
                        parser.current_function(),
                        parser.arg1().unwrap()
                    ));
                }
                Command::Goto => {
                    self.write_goto(format!(
                        "{}${}",
                        parser.current_function(),
                        parser.arg1().unwrap()
                    ));
                }
                Command::If => {
                    self.write_if(format!(
                        "{}${}",
                        parser.current_function(),
                        parser.arg1().unwrap()
                    ));
                }
                Command::Call => {
                    self.write_call(
                        parser.arg1().unwrap(),
                        parser.arg2().unwrap().parse().unwrap(),
                    );
                }
                Command::Return => {
                    self.write_return();
                }
                Command::Function => {
                    self.write_function(
                        parser.arg1().unwrap(),
                        parser.arg2().unwrap().parse().unwrap(),
                    );
                }
            }
        }
    }
}
