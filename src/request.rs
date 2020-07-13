use crate::ast::variable::Variable;
use crate::ast::branch::Branch;
use crate::ast::AstData;

pub enum Request {
	VARIABLE(Variable),
	PRINT(Box<AstData>),
	IF(Branch)
}
