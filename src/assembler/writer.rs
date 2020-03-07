use std::fs::File;
use std::io;
use std::io::prelude::*;

use crate::assembler::code::Code;
use crate::assembler::parser::{Command, Parser};
use crate::assembler::symbol_table::make_symbol_table;

pub struct Writer {}

impl Writer {
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
}
