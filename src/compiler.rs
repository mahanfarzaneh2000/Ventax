use crate::parser::ast::{Node, Operator};

pub struct Interpreter {
	stack: Vec<i32>,
}

impl Interpreter {
	pub fn new() -> Interpreter {
		Interpreter { stack: Vec::new() }
	}

	pub fn interpret_program(&mut self,node: &Node) -> Vec<i32>{
		self.stack.push(self.eval(node));
		self.stack.clone()
	}

	pub fn eval(&self,node:&Node) -> i32{
		match node {
			Node::Int(n) => *n,
			Node::Print(n) =>{
				println!("{}",self.eval(n));
				0
			},
			Node::UnaryExpr { op, child } => {
				let child = self.eval(child);
				match op {
					Operator::Plus => child,
					Operator::Minus => -child,
        	Operator::Times => panic!("unsupported operation"),
        	Operator::Divide => panic!("unsuported operation"),
				}
			}
			Node::BinaryExpr { op, lhs, rhs } => {
				let lhs_ret = self.eval(lhs);
				let rhs_ret = self.eval(rhs);

				match op {
					Operator::Plus => lhs_ret + rhs_ret,
					Operator::Minus => lhs_ret - rhs_ret,
					Operator::Times => lhs_ret * rhs_ret,
					Operator::Divide => lhs_ret / rhs_ret,
				}
			}
		}
	}
}
