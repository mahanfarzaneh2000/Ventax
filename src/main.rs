extern crate pest;
extern crate pest_derive;

use std::env;
use std::fs;

mod parser;
mod compiler;

fn main() {
  let args:Vec<_> = env::args().collect();

	let args_len = args.len();
	if args_len > 1 {
		if args_len == 2 {
			let filename = &args[1];
			if filename.starts_with("-") || filename.ends_with("--") {
				set_options(filename);
			}else{
				// Read the code file
				let contents = fs::read_to_string(filename)
					.expect("Something went wrong reading the file");

				// Parse the code and generate the AST
				let ast = parser::parse(contents.as_str());
				//println!("{:?}",&ast);

				let mut compiler = compiler::Interpreter::new();
				for node in ast.unwrap() {
					//println!("{:?}",node);
					compiler.interpret_program(&node);
				}
				//println!("{:?}",compiler.stack);
			}
		}if args_len == 3{
			let option = &args[1];
			set_options(option)
		}
  }else{
    println!("{}",USAGE);
  }
}

fn set_options(option:&String) {
	match option.as_str() {
		"--compile" | "-c" => println!("Not implemented yet!"),
		"--help" | "-h" => println!("{}",USAGE),
		"--version"| "-v" => println!("{}",VERSION),
		_ => println!("{}",USAGE),
	}
}

const VERSION: &'static str = "Version 0.0.1";
const USAGE: &'static str = "
Version 0.0.1, Copyright (c) 2021-2022, MahanFr\n\
All rights reserved under the MIT license.\n\
Visit https://github.com/mahanfarzaneh2000/ventax for more information.\n\
Usage:\n\
\x20  ventax [options] <file>                   Interprets Code to cpu instructions\n\
\x20  Options:\n\
\x20    -h, --help                              Prints this help message\n\
\x20    -v, --version                           Prints the version\n\
\x20    -c, --compile                           Compiles the code to bytecode\n\
";
