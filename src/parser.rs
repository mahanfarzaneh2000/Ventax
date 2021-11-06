
#[derive(Debug)]
pub struct Node{
	pub kind: NodeType,
}

#[derive(Debug)]
pub enum NodeType{
	IDENTIFIER,
	NUMERICLITERAL(i64),
}

pub fn parse(string:String) -> Vec<Node>{
	let mut vec = Vec::<Node>::new();
	vec.push(_program(string));
	vec
}

fn _program(string:String) -> Node{
	return _numeric_literal(string);
}

fn _numeric_literal(string:String) -> Node{
	return Node{
		kind: NodeType::NUMERICLITERAL(string.parse::<i64>().unwrap()),
	}
}
