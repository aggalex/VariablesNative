use super::Evaluable;
use std::cell::*;
use super::super::Request;
use super::super::parse_statements;
use super::LLVMData;
use super::AstData;
use llvm::*;
use rand::Rng;

pub struct Branch {
    pub condition: Box<dyn Evaluable>,
    pub on_true: RefCell<Vec<Request>>,
    pub on_false: Option<RefCell<Vec<Request>>>,
    id: String
}

impl Branch {
    pub fn new(condition: Box<AstData>, on_true: RefCell<Vec<Request>>, on_false: Option<RefCell<Vec<Request>>>) -> Branch {
        let mut rng = rand::thread_rng();
        Branch {
            condition,
            on_true,
            on_false,
            id: (0..10).map (|_| {
                (0x65u8 + (rng.gen::<f32>() * 20.0) as u8) as char
            }).collect(),
        }
    }

    fn build_basic_block<'a, 'b:'a> (&self, name_ext: &str, llvm_data: &'a LLVMData<'a,'b>) -> &'a BasicBlock {
        let main = llvm_data.module.get_function("main").unwrap();

        let mut name = self.id.clone();
        name.push_str(name_ext);

        main.append(&name[..])
    }
}

fn build_block<'a, 'b:'a>(request: Ref<Vec<Request>>, block: &'a BasicBlock, after_jump: &BasicBlock, llvm_data: &'a LLVMData<'a,'b>) {
    llvm_data.builder.position_at_end(&block);
    parse_statements(&*request, llvm_data);
    llvm_data.builder.build_br(after_jump);
}

impl Evaluable for Branch {
    fn evaluate<'a,'b:'a> (&self, llvm_data: &'a LLVMData<'a,'b>) -> &'a Value {
        let has_else = self.on_false.is_some();

        let condition = llvm_data.builder.build_cmp(
            0i32.compile(llvm_data.module.get_context()),
            (*self.condition).evaluate(llvm_data),
            Predicate::NotEqual);
        
        let if_block = self.build_basic_block("__if", llvm_data);
        let else_block = if has_else {
            Some(self.build_basic_block("__else", llvm_data))
        } else {
            None
        };
        let endif_block = self.build_basic_block("__endif", llvm_data);

        let output = llvm_data.builder.build_cond_br(
            condition, 
            &if_block,
            Some(else_block.unwrap_or(endif_block))
        );

        build_block(self.on_true.borrow(), if_block, endif_block, llvm_data);
        if has_else {
            build_block(self.on_false.as_ref().unwrap().borrow(), else_block.unwrap(), endif_block, llvm_data)
        }

        llvm_data.builder.position_at_end(endif_block);

        output
    }
}