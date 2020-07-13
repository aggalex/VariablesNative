// use std::error::Error;
use std::fs::*;
use std::path::Path;
use std::env;
use std::collections::HashMap;
use std::cell::*;

use llvm::*;

#[macro_use] extern crate lalrpop_util;
extern crate rand;

mod ast;
use ast::Evaluable;

mod request;
use request::Request;

lalrpop_mod!(pub parse); // synthesized by LALRPOP

pub struct LLVMData<'a, 'b:'a> {
    pub module: &'a Module,
    pub builder: &'b mut Builder,
    pub variables: &'b mut RefCell<HashMap<String, &'a Value>>,
    pub printf: &'a Function
}

fn read_file<P>(filename: &P) -> String
where P: AsRef<Path>, {
    read_to_string(filename).expect("Failed to open file")
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Wrong number of arguments");
    }

    let context: &Context;
    unsafe {
        context = Context::get_global();
    }

    let module = Module::new("main", &context);
    
    let main_type = FunctionType::new (
        IntegerType::new(context, 32),
        &[]
    );

    let printf_type = FunctionType::new(
            IntegerType::new(context, 32),
            &[IntegerType::new(context, 32)]
        );

    // Can't use printf because it's variadic
    let printf = module.add_function("print_int", printf_type);
    (*printf).set_linkage(Linkage::External);

    let main = module.add_function("main", main_type);
    let main_entry = main.append ("entry");
    let mut builder = Builder::new(&context);
    builder.position_at_end(&main_entry);

    let mut var_map: RefCell<HashMap<String, &Value>> = RefCell::new(HashMap::new());

    let data = LLVMData {
        module: &module,
        builder: &mut builder,
        variables: &mut var_map,
        printf: &printf
    };

    
    parse_ast(read_file(&args[1]), &data);

    data.builder.build_ret(0.compile(&context));

    match data.module.verify() {
        Ok(_) => println!("Done!"),
        Err(e) => panic!("Illegal module: {}\nGenerated IR: {:?}", e, data.module)
    }

    println!("{:?}", data.module);

    let mut out_path = args[1].to_string();
    out_path.push_str(".out");
    module.compile(Path::new(&out_path), 0).unwrap ();
}

fn parse_ast<'a, 'b:'a> (data: String, llvm_data: &'a LLVMData<'a, 'b>) {
    parse_statements(&(*parse::StatementsParser::new().parse(&data[..])
                                                .unwrap_or_else(|error| { panic!("Execution Failed: {}", error) })
                                                .borrow())
                    , llvm_data
    );
}

pub fn parse_statements<'a, 'b:'a> (statements: &Vec<Request>, llvm_data: &'a LLVMData<'a, 'b>) {
    for statement in statements {
        match statement {
            Request::PRINT(data) => {
                llvm_data.builder.build_call(
                        llvm_data.printf, 
                        &[data.evaluate(llvm_data)]);
            }
            Request::VARIABLE(var) => match &var.data {
                Some(value) => {
                    let data = value.evaluate(llvm_data);
                    llvm_data.variables.borrow_mut().insert (var.identifier.clone(), data);
                },
                None => panic!("Recieved variable creation request without value")
            }
            Request::IF(branch) => {
                branch.evaluate(llvm_data);
            }
        }
    }
}