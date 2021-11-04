/*
* This file is a part of the compiler
* scan_code_file function will return a list of tokens
* the next step is to parse the tokens
*/

use std::io::{ BufRead, BufReader};
use std::fs::File;

#[derive(Debug, PartialEq)]
enum TokenType {
	// Literal Tokens
	STRING,CHAR,NONE,INTEGER,
	// Keyword Tokens
	PRINT,
	// Operator Tokens
	PLUS
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
	// Loops through file line by line
	while f.read_until(b'\n', &mut buf).expect("read_until failed") != 0 {
		// Transforms bites into string
		let s = String::from_utf8(buf).expect("from_utf8 failed");

		// Collects multi character lexemes
		let mut buffer = String::new();
		// Checks for literals
		let mut literal_token_buffer = TokenType::NONE;

		for c in s.chars() {
			if c == '\n'{
				// panics if literal_token_buffer is not NONE
				// or like string literal includes 2 lines
				if literal_token_buffer != TokenType::NONE {
					panic!("ERROR: Unterminated literal token");
				}
				// prints every thing that is in the buffer before the new line
				if buffer.len() > 0 {
					create_token(&buffer,line_number,col_number-buffer.len() as u32,None);
				}
				buffer.clear();
			} else if  c == ' ' || c == '\t' || c == '\n' || c == '\r' {
				// checks if literal_token_buffer is not NONE include space inside the buffer
				if literal_token_buffer != TokenType::NONE {
					buffer.push(c);
				}else{
					// prints every thing that is in the buffer before space or (tab or new line or carriage return)
					if buffer.len() > 0 {
						create_token(&buffer,line_number,col_number-buffer.len() as u32,None);
					}
					buffer.clear();
				}
			} else if c == '"'  {
				// if there is no opened double quote
				if literal_token_buffer == TokenType::NONE {
					// checks if literal_token_buffer is not NONE tokenize space inside the buffer
					if buffer.len() > 0 {
						create_token(&buffer,line_number,col_number-buffer.len() as u32,None);
					}
					buffer.clear();
					// sets literal_token_buffer to STRING so that it can be closed later
					literal_token_buffer = TokenType::STRING;
					// include double quote in the buffer
					buffer.push(c);
				}else if literal_token_buffer == TokenType::STRING {
					// if double quote is closed make literal_token_buffer NONE
					literal_token_buffer = TokenType::NONE;
					// include double quote in the buffer
					buffer.push(c);
					// col number is not incremented after the closing quote pushed to buffer
					if buffer.len() > 0 {
						create_token(&buffer,line_number,col_number+1-buffer.len() as u32,Some(TokenType::STRING));
					}
					buffer.clear();
				}else{
					buffer.push(c);
				}
			} else if c == '\''{
				// if there is no opened quote
				if literal_token_buffer == TokenType::NONE {
					// checks if literal_token_buffer is not NONE tokenize space inside the buffer
					if buffer.len() > 0 {
						create_token(&buffer,line_number,col_number-buffer.len() as u32,None);
					}
					buffer.clear();
					literal_token_buffer = TokenType::CHAR;
					// starting ['] is also included in the literal token
					buffer.push(c);
				}else if literal_token_buffer == TokenType::CHAR {
					// if quote is closed make literal_token_buffer NONE
					literal_token_buffer = TokenType::NONE;
					// ending ['] is also included in the literal token
					buffer.push(c);
					// col number is not incremented after the closing quote pushed to buffer
					if buffer.len() > 0 {
						create_token(&buffer,line_number,col_number+1-buffer.len() as u32,Some(TokenType::CHAR));
					}
					buffer.clear();
				}else{
					buffer.push(c);
				}
			}else if c == '+'{
				// checks if literal_token_buffer is not NONE tokenize space inside the buffer
				if literal_token_buffer != TokenType::NONE {
					buffer.push(c);
				}else{
					// prints every thing that is in the buffer before +
					if buffer.len() > 0 {
						create_token(&buffer,line_number,col_number-buffer.len() as u32,None);
					}
					buffer.clear();
					create_token(&("+".to_string()), line_number, col_number, Some(TokenType::PLUS));
				}
			}else{
				buffer.push(c);
			}
			col_number +=1;
		}
		// For the last line that has no new line at the end will print reminder of buffer
		if buffer.len() > 0 {
			create_token(&buffer,line_number,col_number-buffer.len() as u32,None);
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
}

// Create tokens based on collected information when reading the text file
fn create_token(phrase: &String, line_number: u32, col_number: u32,known_token:Option<TokenType>) -> Token {

	let mut token = Token{
		token_type: TokenType::NONE,
		literal: phrase.to_string(),
		line: line_number,
		column: col_number,
	};

	// if token is provided by scanner
	if known_token != None {
		token.token_type = known_token.unwrap();
	} else{
		if phrase.len() > 0 {
			if phrase.starts_with("print") {
				token.token_type = TokenType::PRINT;
			}else if phrase.to_string().chars().all(char::is_numeric){
				token.token_type = TokenType::INTEGER;
			}
		}
	}

	println!("{:?}",token);
	return token;
}
