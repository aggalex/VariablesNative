use llvm::*;
use super::Evaluable;
use super::LLVMData;

impl Evaluable for i32 {
	fn evaluate<'a,'b:'a> (&self, llvm_data: &'a LLVMData) -> &'a Value {
		self.compile(llvm_data.module.get_context())
	}
}
