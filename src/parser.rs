
use pest::{self,Parser};

pub mod ast;

#[derive(pest_derive::Parser)]
#[grammar = "parser/grammar.pest"]
struct ProgramParser;

// Pars program using pest
pub fn parse(source: &str) -> std::result::Result<Vec<ast::Node>, pest::error::Error<Rule>> {
	let mut ast = vec![];
	let pairs = ProgramParser::parse(Rule::Program, source)?;
	//println!("{:#?}", pairs);
	for pair in pairs {
		if let Rule::Instruction = pair.as_rule() {
			//ast.push(build_ast_from_expr(pair));
			ast.push(build_ast_from_instruction(pair));
		}
	}
	// Return the abstract syntax tree
	Ok(ast)
}

fn build_ast_from_instruction(pair: pest::iterators::Pair<Rule>) -> ast::Node {
	match pair.as_rule() {
		Rule::Instruction => build_ast_from_instruction(pair.into_inner().next().unwrap()),
		Rule::Expr => build_ast_from_expr(pair.into_inner().next().unwrap()),
		Rule::UnaryExpr => {
			let mut pair = pair.into_inner();
			let op = pair.next().unwrap();
			let child = pair.next().unwrap();
			let child = build_ast_from_term(child);
			parse_unary_expr(op, child)
		}
		Rule::BinaryExpr => {
			let mut pair = pair.into_inner();
			let lhspair = pair.next().unwrap();
			let lhs = build_ast_from_term(lhspair);
			let op = pair.next().unwrap();
			let rhspair = pair.next().unwrap();
			let rhs = build_ast_from_term(rhspair);
			parse_binary_expr(op, lhs, rhs)
		},
		Rule::Print =>{
			let mut pair = pair.into_inner();
			let child = pair.next().unwrap();
			let child = build_ast_from_term(child);
			ast::Node::Print(Box::new(child))
		},
		Rule::VariableDeclaration => {
			let mut pair = pair.into_inner();
			let var = pair.next().unwrap();
			let var = var.as_str();
			let child = pair.next().unwrap();
			let child = build_ast_from_expr(child);
			ast::Node::Declaration{ name: var.to_string(), value: Box::new(child) }
		},
		Rule::VariableAssignment => {
			let mut pair = pair.into_inner();
			let var = pair.next().unwrap();
			let var = var.as_str();
			let child = pair.next().unwrap();
			let child = build_ast_from_expr(child);
			ast::Node::Assignment{ name: var.to_string(), value: Box::new(child) }
		},
		unknown => panic!("Unknown expr: {:?}", unknown),
	}
}


// Creates ast node form pest pair
fn build_ast_from_expr(pair: pest::iterators::Pair<Rule>) -> ast::Node {
	match pair.as_rule() {
		Rule::Expr => build_ast_from_expr(pair.into_inner().next().unwrap()),
		Rule::UnaryExpr => {
			let mut pair = pair.into_inner();
			let op = pair.next().unwrap();
			let child = pair.next().unwrap();
			let child = build_ast_from_term(child);
			parse_unary_expr(op, child)
		}
		Rule::BinaryExpr => {
			let mut pair = pair.into_inner();
			let lhspair = pair.next().unwrap();
			let lhs = build_ast_from_term(lhspair);
			let op = pair.next().unwrap();
			let rhspair = pair.next().unwrap();
			let rhs = build_ast_from_term(rhspair);
			parse_binary_expr(op, lhs, rhs)
		},
		Rule::Print =>{
			let mut pair = pair.into_inner();
			let child = pair.next().unwrap();
			let child = build_ast_from_expr(child);
			ast::Node::Print(Box::new(child))
		},
		Rule::Identifier => {
			let var = pair.as_str();
			ast::Node::Identifier(var.to_string())
		}
		unknown => panic!("Unknown expr: {:?}", unknown),
	}
}

fn build_ast_from_term(pair: pest::iterators::Pair<Rule>) -> ast::Node {
	match pair.as_rule() {
		Rule::Int => {
			let istr = pair.as_str();
			let (sign, istr) = match &istr[..1] {
					"-" => (-1, &istr[1..]),
					_ => (1, istr),
			};
			let int: i32 = istr.parse().unwrap();
			ast::Node::Int(sign * int)
		}
		Rule::Expr => build_ast_from_expr(pair),
		Rule::Identifier => {
			let var = pair.as_str();
			ast::Node::Identifier(var.to_string())
		}
		unknown => panic!("Unknown term: {:?}", unknown),
	}
}

fn parse_unary_expr(pair: pest::iterators::Pair<Rule>, child: ast::Node) -> ast::Node {
	ast::Node::UnaryExpr {
		op: match pair.as_str() {
			"+" => ast::Operator::Plus,
			"-" => ast::Operator::Minus,
			_ => unreachable!(),
		},
			child: Box::new(child),
	}
}

fn parse_binary_expr(pair: pest::iterators::Pair<Rule>, lhs: ast::Node, rhs: ast::Node) -> ast::Node {
	ast::Node::BinaryExpr {
			op: match pair.as_str() {
				"+" => ast::Operator::Plus,
				"-" => ast::Operator::Minus,
				"*" => ast::Operator::Times,
				"/" => ast::Operator::Divide,
				_ => unreachable!(),
			},
			lhs: Box::new(lhs),
			rhs: Box::new(rhs),
	}
}
