use llvm::*;
use super::LLVMData;

pub mod branch;
pub mod variable;
mod evaluable_int;

pub type AstData = dyn Evaluable;

pub enum Operation {
	ADD,
	SUB,
	MUL,
	DIV,
}

pub struct Ast {
	pub left: Box::<AstData>,
	pub op: Operation,
	pub right: Box::<AstData>
}

pub trait Evaluable {
	fn evaluate<'llvm_data, 'scope_data:'llvm_data> (&self, llvm_data: &'llvm_data LLVMData<'llvm_data,'scope_data>) -> &'llvm_data Value;
}

impl Ast {

	pub fn new (left: Box<AstData>, op: Operation, right: Box<AstData>) -> Ast {
		Ast {
			left, op, right
		}
	}

}

impl Evaluable for Ast {

	fn evaluate<'a,'b:'a> (&self, llvm_data: &'a LLVMData<'a,'b>) -> &'a Value {
		let left = (*self.left).evaluate(llvm_data);
		let right = (*self.right).evaluate(llvm_data);
		
		match self.op {
			Operation::ADD => llvm_data.builder.build_add(left, right),
			Operation::SUB => llvm_data.builder.build_sub(left, right),
			Operation::MUL => llvm_data.builder.build_mul(left, right),
			Operation::DIV => llvm_data.builder.build_div(left, right)
		}
	}
}
