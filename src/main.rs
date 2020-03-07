use nand2tetris::assembler::writer::Writer;
extern crate clap;
use std::path::Path;
use clap::{App, Arg};

fn main() {
    let app = App::new("nand2tetris")
        .arg(Arg::with_name("asm_path").help("path to asm file").required(true))
        .arg(Arg::with_name("out").help("output path").short("o").long("out").takes_value(true));
    let matches = app.get_matches();
    let asm_path = Path::new(matches.value_of("asm_path").unwrap()).canonicalize().unwrap();
    let out;
    if let Some(o) = matches.value_of("out") {
        out = o.to_string();
    } else {
        let out_name = asm_path.with_extension("hack").clone();
        out = asm_path.with_file_name(out_name).to_string_lossy().to_string();
    };
    Writer::write(asm_path.to_str().unwrap(), out.as_str());
}
