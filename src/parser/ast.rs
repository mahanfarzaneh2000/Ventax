#[derive(Debug)]
pub enum Operator {
	Plus,
	Minus,
	Times,
	Divide
}

#[derive(Debug)]
pub enum Node {
	Int(i32),
	UnaryExpr {
		op: Operator,
		child: Box<Node>,
	},
	BinaryExpr {
		op: Operator,
		lhs: Box<Node>,
		rhs: Box<Node>,
	},
	Print(Box<Node>),
	Declaration {
		name: String,
		value: Box<Node>,
	},
	Assignment {
		name: String,
		value: Box<Node>,
	},
	Identifier(String),
}
