// use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;
use std::collections::HashMap;
use std::cell::*;

use llvm::*;

#[macro_use] extern crate lalrpop_util;

mod ast;

mod request;
use request::Request;

lalrpop_mod!(pub parse); // synthesized by LALRPOP

pub struct LLVMData<'a> {
    pub module: &'a Module,
    pub builder: &'a mut Builder,
    pub variables: &'a mut RefCell<HashMap<String, &'a Value>>,
    pub printf: &'a Function
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
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

    if let Ok(lines) = read_lines (args[1].to_string ()) {
        for line in lines {
            if let Ok(line) = line {
                if let Some(value) = parse_line (line, &data) {
                    data.variables.borrow_mut().insert (value.0, value.1);
                }
            }
        }
    }

    data.builder.build_ret(0.compile(&context));

    // The module will never verify because printf is variadic, and this LLVM library has no variadics

    match data.module.verify() {
        Ok(_) => println!("Done!"),
        Err(e) => panic!("Illegal module: {}", e)
    }

    println!("{:?}", data.module);

    let mut out_path = args[1].to_string();
    out_path.push_str(".out");
    module.compile(Path::new(&out_path), 0).unwrap ();
}

fn parse_line<'a> (line: String, llvm_data: &'a LLVMData<'a>) -> Option<(String, &'a Value)> {
    match parse::StatementParser::new().parse(&line[..]) {
        Ok(_res) => match _res {
            Request::PRINT(data) => {
                llvm_data.builder.build_call(
                        llvm_data.printf, 
                        &[data.evaluate(llvm_data)]);
                None
            }
            Request::VARIABLE(var) => match var.data {
                Some(value) => Some((var.identifier, value.evaluate (llvm_data))),
                None => panic!("Recieved variable creation request without value")
            }
        },
        Err(_res) => panic!("Execution failed: {}", _res)
    }
}