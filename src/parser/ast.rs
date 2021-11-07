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
}
