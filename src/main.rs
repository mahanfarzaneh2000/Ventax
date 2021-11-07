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
			// Read the code file
			let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
			// Parse the code and generate the AST
			let ast = parser::parse(contents.as_str());
			//println!("{:?}",ast.unwrap());

			// TODO: traverse the AST tree
			let stack = compiler::Interpreter::new().interpret_program(ast.unwrap().last().unwrap());
			println!("{:?}",stack);
		}else{
			panic!("TODO: read args from user and interpret");
		}
  }else{
    panic!("TODO: set up a shell");
  }
}
