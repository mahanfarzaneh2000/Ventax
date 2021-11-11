use crate::parser::ast::{Node, Operator};
use std::collections::HashMap;

pub struct Interpreter {
	pub stack: Vec<i32>,
	pub variables: HashMap<String, i32>,
}

impl Interpreter {
	pub fn new() -> Interpreter {
		Interpreter { stack: Vec::new() , variables: HashMap::new() }
	}

	pub fn interpret_program(&mut self,node: &Node){
		let evaled_node = self.eval(node);
		self.stack.push(evaled_node);
		// self.stack.clone()
	}

	pub fn eval(&mut self,node:&Node) -> i32{
		match node {
			Node::Declaration{name,value} => {
				let liteal_value = self.eval(value);
				self.variables.insert(name.clone(),liteal_value);
				// println!("Define {} = {:?}",name,self.eval(value));
				0
			},
			Node::Identifier(name) => {
				if self.variables.contains_key(name) {
					let value = self.variables.get(name).unwrap().clone();
					return value
				}else{
					println!("Variable {} not found in this scope",name);
				}
				//println!("Assign {} = {:?}",name,self.eval(value));
				0
			},
			Node::Assignment{name,value} => {
				let liteal_value = self.eval(value);
				if self.variables.contains_key(name) {
					self.variables.insert(name.clone(),liteal_value);
				}else{
					println!("Variable {} not found in this scope",name);
				}
				//println!("Assign {} = {:?}",name,self.eval(value));
				0
			},
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
        	Operator::Divide => panic!("unsupported operation"),
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
