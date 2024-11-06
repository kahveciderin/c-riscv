use std::env;
use std::fs::File;
use std::io::{Read, Write};

mod parser;
mod riscv;
mod types;
mod utils;

use parser::parse_program;
use riscv::{compile_program, optimize_program};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Usage: {} <filename>", args[0]);
    }

    let filename = &args[1];

    let input_file_path = format!("tests/{}.c", filename);
    let output_file_path = format!("output/{}.s", filename);

    let mut file = File::open(input_file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let input = contents.as_str();

    let ast = parse_program(input).unwrap();
    let compiled_output = compile_program(ast);
    let compiled_output = optimize_program(compiled_output);

    println!("Compiled Output:");
    compiled_output.iter().for_each(|x| {
        println!("{x:?}");
    });

    let mut file = File::create(output_file_path).unwrap();
    compiled_output.iter().for_each(|x| {
        file.write_all(x.to_string().as_bytes()).unwrap();
        file.write_all("\n".as_bytes()).unwrap();
    });
}
