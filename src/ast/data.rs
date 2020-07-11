use std::any::{Any, TypeId};
use std::collections::HashMap;

use super::variable::Variable;
use super::{Ast, AstData};

enum Data {
	NUM(i32),
	AST(Ast),
	VAR(Variable)
}

fn eval_data (data: Data, var_map: &HashMap<String, i32>) -> i32 {
	match data {
		Data::NUM(num) => num,
		Data::AST(eval) | Data::VAR(eval) => eval.evaluate (var_map)
	}
}

fn to_data<G> (data: G) -> Data
	where G: Evaluable
{
	match TypeId::of::<G>() {
		TypeId::of::<i32>() => NUM(data as i32),
		TypeId::of::<Ast>() => AST(data as Ast),
		TypeId::of::<Variable> () => VAR(data as Variable)
	}
}
