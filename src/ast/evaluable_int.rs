use std::collections::HashMap;
use llvm::*;
use super::Evaluable;
use super::LLVMData;

impl Evaluable for i32 {
	fn evaluate<'a> (&self, llvm_data: &'a LLVMData) -> &'a Value {
		self.compile(llvm_data.module.get_context())
	}
}
