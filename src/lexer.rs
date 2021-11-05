/*
* This file is a part of the compiler
* scan_code_file function will return a list of tokens
* the next step is to parse the tokens
*/
use std::io::{ BufRead, BufReader};
use std::fs::File;
use regex::Regex;

#[derive(Debug, PartialEq)]
enum TokenType {
	NL,WS,
	STRING,CHAR,NONE,
	LPAREN,RPAREN,LBRACKET,RBRACKET,
	LBRACE,RBRACE,
	COMMA,SEMICOLON,
	POW,MOD,
	ASSIGN,
	ADDRESS,
	MINUS,
	PLUS,
	DIVIDE,
	MULTIPLY,
	BIGGER,
	SMALLER,
	COLON,
	PIPE,
	QMARK,

	DOT,
	NOT,
	EQ,
	NEQ,
	GEQ,
	LEQ,
	DEC,
	INC,
	AND,
	OR,
	SHL,
	SHR,
	ARROW,
	FATARROW,
	SCOPE,
	UNSAFE,
	COMMENT,

	IDENTIFIER,
	NUMBER,
}

#[derive(Debug)]
struct Token{
	token_type: TokenType,
	literal: String,
	line: u32,
	column: u32,
}

// Read text file and return a vector of tokens (PRINT for debugging)
pub fn scan_code_file(file_path:&str) {
	// Reading file as buffer
	let mut f = BufReader::new(File::open(file_path).expect("ERROR: File not Found"));

	// stores lines of code in bits
	let mut buf = Vec::<u8>::new();
	// Collects column number
	let mut col_number : u32 = 1;
	// Collects line number
	let mut line_number : u32 = 1;

	let mut token_stack = Vec::<Token>::new();

	// Loops through file line by line
	while f.read_until(b'\n', &mut buf).expect("read_until failed") != 0 {
		// Transforms bites into string
		let s = String::from_utf8(buf).expect("from_utf8 failed");

		// Collects multi character lexemes
		let mut buffer = String::new();
		// Checks for literals
		let mut literal_token_buffer = TokenType::NONE;

		for cursor in s.chars() {
			// New line
			if cursor == '\r'{
				// Ignore
			}
			else if cursor == '\n'{
				if literal_token_buffer == TokenType::STRING || literal_token_buffer == TokenType::CHAR {
					panic!("ERROR: Unclosed literal {}:{}",col_number,line_number);
				}else if literal_token_buffer == TokenType::COMMENT {
					literal_token_buffer = TokenType::NONE;
				}
				if buffer.len() > 0{
					validate_buffer(&buffer,&mut token_stack,line_number,col_number-buffer.len() as u32);
					buffer.clear();
				}
				token_stack.push(Token{token_type:TokenType::NL, literal:"".to_string(), line:line_number, column:col_number});
			}

			// Whitespace
			else if cursor == ' ' || cursor == '\t' {
				if literal_token_buffer != TokenType::NONE{
					buffer.push(cursor);
				}else{
					if buffer.len() > 0{
						validate_buffer(&buffer,&mut token_stack,line_number,col_number-buffer.len() as u32);
						buffer.clear();
					}
					token_stack.push(Token{token_type:TokenType::WS, literal:cursor.to_string(), line:line_number, column:col_number});
				}
			}

			// Literal
			else if cursor == '"' || cursor == '\'' {
				if literal_token_buffer == TokenType::NONE{
					if buffer.len() > 0{
						validate_buffer(&buffer,&mut token_stack,line_number,col_number-buffer.len() as u32);
						buffer.clear();
					}
					buffer.push(cursor);
					match cursor{
						'"' => literal_token_buffer = TokenType::STRING,
						'\'' => literal_token_buffer = TokenType::CHAR,
						_ => (),
					}
				}else if literal_token_buffer == TokenType::STRING && cursor == '"'{
          literal_token_buffer = TokenType::NONE;
					buffer.push(cursor);
					validate_buffer(&buffer,&mut token_stack,line_number,col_number+1 -buffer.len() as u32);
					buffer.clear();
				}else if literal_token_buffer == TokenType::CHAR && cursor == '\''{
					literal_token_buffer = TokenType::NONE;
					buffer.push(cursor);
					validate_buffer(&buffer,&mut token_stack,line_number,col_number+1 -buffer.len() as u32);
					buffer.clear();
				}else {
					buffer.push(cursor);
				}
			}

			// Single meaning characters
			else if cursor == '%' || cursor == '^' || cursor == '(' ||
				cursor == ')' || cursor == '[' || cursor == ']' || cursor == '{' || cursor == '}' ||
				cursor == ',' || cursor == ';'
			{
				if literal_token_buffer != TokenType::NONE{
					buffer.push(cursor);
				}else{
					if buffer.len() > 0{
						validate_buffer(&buffer,&mut token_stack,line_number,col_number-buffer.len() as u32);
						buffer.clear();
					}
					let t = get_single_char_token_type(cursor);
					token_stack.push(Token { token_type: t,
						literal: cursor.to_string(),
						line: line_number,
						column: col_number
					});
				}
			}

			// Double meaning characters
			else if cursor == '=' || cursor == '!' || cursor == '&' || cursor == '-' || cursor == '+' ||
				cursor == '/' || cursor == '*' || cursor == '<' || cursor == '>' || cursor == ':' ||
				cursor == '?' || cursor == '|' || cursor == '~' || cursor == '.'
			{
				if literal_token_buffer != TokenType::NONE{
					buffer.push(cursor);
				}else{
					if buffer.len() > 0{
						validate_buffer(&buffer,&mut token_stack,line_number,col_number-buffer.len() as u32);
						buffer.clear();
					}
					let last_token : Token;
					if token_stack.len() > 0{
						last_token = token_stack.pop().unwrap();
						if is_part_of_double_char_tokens(&last_token.token_type){
							let u_token = update_token_type(last_token,cursor);
							if u_token.token_type == TokenType::COMMENT{
								literal_token_buffer = TokenType::COMMENT;
							}
							token_stack.push(u_token);
						}else{
							token_stack.push(last_token);
							let t = get_single_char_token_type(cursor);
							token_stack.push(Token { token_type: t,
								literal: cursor.to_string(),
								line: line_number,
								column: col_number
							});
						}
					}else{
						let t = get_single_char_token_type(cursor);
						token_stack.push(Token { token_type: t,
							literal: cursor.to_string(),
							line: line_number,
							column: col_number
						});
					}

				}
			}

			// just Push to Buffer
			else{
				buffer.push(cursor);
			}
			col_number +=1;
		}
		// For the last line that has no new line at the end will print reminder of buffer

		if buffer.len() > 0 {
			validate_buffer(&buffer,&mut token_stack,line_number,col_number-buffer.len() as u32);
			buffer.clear();
		}
		buffer.clear();
		// increments line number
		line_number += 1;
		// reset column number
		col_number = 1;
		// this returns the ownership of the read data to buf
		buf = s.into_bytes();
		buf.clear();
	}
	println!("{:?}",token_stack);
}

fn update_token_type(last_token: Token, cursor: char) -> Token {
	let mut new_token_string = last_token.literal;
	new_token_string.push(cursor);
	let mut new_token = Token{
		token_type: TokenType::NONE,
		literal: new_token_string,
		line: last_token.line,
		column: last_token.column,
	};

	if new_token.literal == "=="{
		new_token.token_type = TokenType::EQ;
	}else if new_token.literal == "!="{
		new_token.token_type = TokenType::NEQ;
	}else if new_token.literal == ">="{
		new_token.token_type = TokenType::GEQ;
	}else if new_token.literal == "<="{
		new_token.token_type = TokenType::LEQ;
	}else if new_token.literal == "--"{
		new_token.token_type = TokenType::DEC;
	}else if new_token.literal == "++"{
		new_token.token_type = TokenType::INC;
	}else if new_token.literal == "&&"{
		new_token.token_type = TokenType::AND;
	}else if new_token.literal == "||"{
		new_token.token_type = TokenType::OR;
	}else if new_token.literal == "<<"{
		new_token.token_type = TokenType::SHL;
	}else if new_token.literal == ">>"{
		new_token.token_type = TokenType::SHR;
	}else if new_token.literal == "->"{
		new_token.token_type = TokenType::ARROW;
	}else if new_token.literal == "=>"{
		new_token.token_type = TokenType::FATARROW;
	}else if new_token.literal == "::"{
		new_token.token_type = TokenType::SCOPE;
	}else if new_token.literal == ":?"{
		new_token.token_type = TokenType::UNSAFE;
	}else if new_token.literal == "//"{
		new_token.token_type = TokenType::COMMENT;
	}else{
		panic!("invalid Token {}:{}",last_token.line,last_token.column)
	}
	new_token
}

fn is_part_of_double_char_tokens(token_type: &TokenType) -> bool {
	match token_type{
		TokenType::ASSIGN | TokenType::ADDRESS | TokenType::MINUS | TokenType::PLUS |
		TokenType::DIVIDE | TokenType::MULTIPLY | TokenType::BIGGER | TokenType::SMALLER |
		TokenType::COLON | TokenType::NOT | TokenType::PIPE | TokenType::QMARK => true,
		_ => false,
	}
}

fn get_single_char_token_type(cursor: char) -> TokenType{
   match cursor{
		 // always single character tokens
		'%' => TokenType::MOD,
		'^' => TokenType::POW,
		'(' => TokenType::LPAREN,
		')'	=> TokenType::RPAREN,
		'['	=> TokenType::LBRACKET,
		']'	=> TokenType::RBRACKET,
		'{'	=> TokenType::LBRACE,
		'}'	=> TokenType::RBRACE,
		','	=> TokenType::COMMA,
		';'	=> TokenType::SEMICOLON,
		'.' => TokenType::DOT,
		// Can be multiple character tokens
		'!' => TokenType::NOT,
		'&' => TokenType::ADDRESS,
		'=' => TokenType::ASSIGN,
		'+' => TokenType::PLUS,
		'-' => TokenType::MINUS,
		'*' => TokenType::MULTIPLY,
		'/' => TokenType::DIVIDE,
		'<' => TokenType::SMALLER,
		'>' => TokenType::BIGGER,
		'?' => TokenType::QMARK,
		'|' => TokenType::PIPE,
		':' => TokenType::COLON,
		_ => {
			panic!("ERROR: Invalid character");
		},
	 }
}

// TODO: ADD KEYWORDS
fn validate_buffer(buffer: &String,token_vec : &mut Vec<Token>,line_number: u32,col_number: u32) {
	// Regex for identifiers
	let identifier_regex = Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*$").unwrap();
	// Regex for numbers
	let number_regex = Regex::new(r"^[0-9]+$").unwrap();
	// Regex for strings
	let string_regex = Regex::new(r#"^".*""#).unwrap();

	let char_regex = Regex::new(r"^'.'$").unwrap();

	let mut token = Token {
		token_type:TokenType::NONE,
		literal:buffer.clone(),
		line:line_number,
		column:col_number
	};

	if identifier_regex.is_match(buffer) {
		token.token_type = TokenType::IDENTIFIER;
	}else if number_regex.is_match(buffer) {
		token.token_type = TokenType::NUMBER;
	}else if string_regex.is_match(buffer) {
		token.token_type = TokenType::STRING;
	}else if char_regex.is_match(buffer) {
		token.token_type = TokenType::CHAR;
	}else{
		panic!("ERROR: Invalid token {}:{}",line_number,col_number);
	}

	if token_vec.len() > 0{
		let last_token = token_vec.last().unwrap();
		let last_token_type = &last_token.token_type;
		if *last_token_type == TokenType::COMMENT {
			token_vec.pop();
			return;
		}else if token.token_type == TokenType::NUMBER {
			match last_token_type{
				TokenType::MINUS => {
					token_vec.pop();
					token.token_type = TokenType::NUMBER;
					token.literal = format!("-{}",buffer);
				},
				_ => (),
			}

		}
	}
	token_vec.push(token);
}
