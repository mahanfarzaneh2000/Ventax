use std::env;

mod lexer;

fn main() {
  let args:Vec<_> = env::args().collect();
  // TODO: let mut has_errors = false;

	let args_len = args.len();
	if args_len > 1 {
		if args_len == 2 {
			let file_path = &args[1];
			lexer::scan_code_file(file_path);
		}else{
			panic!("TODO: read args from user and interpret");
		}
  }else{
    panic!("TODO: set up a shell");
  }
}
