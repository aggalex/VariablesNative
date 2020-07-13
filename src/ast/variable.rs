use std::string::String;
use llvm::*;
use super::Evaluable;
use super::AstData;
use super::LLVMData;

pub struct Variable {
	pub identifier: String,
	pub data: Option<Box::<AstData>>
}

impl Variable {

	pub fn new_boxed (identifier: String, data: Box<AstData>) -> Variable {
		let data_option = Some(data);
		Variable {
			identifier,
			data: data_option
		}
	}

	pub fn new_read (identifier: String) -> Variable {
		Variable {
			identifier,
			data: None
		}
	}

}

impl Evaluable for Variable {
	fn evaluate<'a,'b:'a> (&self, llvm_data: &'a LLVMData<'a,'b>) -> &'a Value {
		match &self.data {
			Some(data) => (*data).evaluate (llvm_data),
			None => match llvm_data.variables.borrow().get (&self.identifier) {
				Some(val) => {
					let value = *val;
					value
				},
				None => panic!("Variable {} not found!", &self.identifier)
			}
		}
	}
}
