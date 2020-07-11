use crate::ast::variable::Variable;
use crate::ast::AstData;

pub enum Request {
	VARIABLE(Variable),
	PRINT(Box<AstData>),
}
