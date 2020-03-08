use std::ffi::OsStr;

use nand2tetris::assembler;
use nand2tetris::vm_translator;

extern crate clap;
use clap::{App, Arg};
use std::path::Path;

fn main() {
    let app = App::new("nand2tetris")
        .arg(
            Arg::with_name("input")
                .help("path to input dir")
                .required(true),
        )
        .arg(
            Arg::with_name("output")
                .help("output path")
                .short("o")
                .long("out")
                .takes_value(true),
        );
    let matches = app.get_matches();
    let input = Path::new(matches.value_of("input").unwrap())
        .canonicalize()
        .unwrap();
    let files = input.read_dir().unwrap().filter(|x| match x.as_ref().ok() {
        Some(dir_entry) => dir_entry.path().extension() == Some(OsStr::new("vm")),
        None => false,
    });
    let asm_path = input.join(format!("{}.asm", input.file_name().unwrap().to_str().unwrap())).to_string_lossy().to_string();
    for file in files {
        let file = file.unwrap().path();
        let vm_path = file.to_str().unwrap();
        println!("{:?}", vm_path);
        println!("{:?}", asm_path);
        vm_translator::code_writer::Writer::new(vm_path, asm_path.as_str()).write();
    }

    let input = Path::new(asm_path.as_str()).canonicalize().unwrap();
    let output;
    if let Some(o) = matches.value_of("output") {
        output = o.to_string();
    } else {
        let out_name = input.with_extension("hack").clone();
        output = input.with_file_name(out_name).to_string_lossy().to_string();
    };
    assembler::writer::Writer::write(input.to_str().unwrap(), output.as_str());
}
