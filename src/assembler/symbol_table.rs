use std::collections::HashMap;

use crate::assembler::parser::{Command, Parser};

pub struct SymbolTable {
    table: HashMap<String, usize>,
}

impl SymbolTable {
    pub fn new() -> Self {
        let mut table = HashMap::new();
        table.insert("SP".to_string(), 0);
        table.insert("LCL".to_string(), 1);
        table.insert("ARG".to_string(), 2);
        table.insert("THIS".to_string(), 3);
        table.insert("THAT".to_string(), 4);
        table.insert("R0".to_string(), 0);
        table.insert("R1".to_string(), 1);
        table.insert("R2".to_string(), 2);
        table.insert("R3".to_string(), 3);
        table.insert("R4".to_string(), 4);
        table.insert("R5".to_string(), 5);
        table.insert("R6".to_string(), 6);
        table.insert("R7".to_string(), 7);
        table.insert("R8".to_string(), 8);
        table.insert("R9".to_string(), 9);
        table.insert("R10".to_string(), 10);
        table.insert("R11".to_string(), 11);
        table.insert("R12".to_string(), 12);
        table.insert("R13".to_string(), 13);
        table.insert("R14".to_string(), 14);
        table.insert("R15".to_string(), 15);
        table.insert("SCREEN".to_string(), 16384);
        table.insert("KBD".to_string(), 24576);
        Self { table: table }
    }

    pub fn add_entry(&mut self, symbol: String, address: usize) {
        self.table.insert(symbol, address);
    }

    pub fn contains(&self, symbol: &String) -> bool {
        self.table.contains_key(symbol)
    }

    pub fn get_address(&self, symbol: &String) -> usize {
        *self.table.get(symbol).unwrap()
    }
}

pub fn make_symbol_table(parser: &mut Parser) -> SymbolTable {
    let mut symbol_table = SymbolTable::new();
    let mut i = 0;
    loop {
        if !parser.has_more_commands() {
            break;
        }
        parser.advance();
        match parser.command_type() {
            Command::LCommand => {
                let symbol = parser.symbol().unwrap();
                if !symbol.chars().nth(0).unwrap().is_digit(10) {
                    symbol_table.add_entry(symbol, i);
                }
            }
            _ => {
                i += 1;
            }
        };
    }
    symbol_table
}
