# Variables
A simple example of a compiler with the LLVM compiler infrastructure, Rust and the LALRPOP parser

### What can it do?
It can successfuly parse and execute expression evaluation, variable creation and evaluation and printing. It also has comments.
An example of what the program can do is in the `examples/simple` file

### Syntax
- `variable_name = <expression>`
- `print <expression>`

An expression is a simple series of mathematical of mathematical operations between constants or variables, just like in most languages. They can be evaluated to a single value. For now, all values are integers in this example.

- Operations
  - `val1 + val2`: Addition
  - `val1 - val2`: Subtraction
  - `val1 * val2`: Multiplication
  - `val1 / val2`: Division
  
`val1` and `val2` are either initialized variables or immidiate constants, like `1`, `2`, `3`, `4`...

### Building and running
To build the project you will need the Rust building system `cargo`. Most Linux distributions have `cargo` in their repositories, from which you can install the recommended version of cargo by your distribution maintainer. If however you are not using Linux or wish to install the version recomended by the `cargo` developers, you can follow these instructions: https://doc.rust-lang.org/cargo/getting-started/installation.html

to build the project, go the root folder of the project and run

	cargo build

then again, if you wish to run the interpreter, use

	cargo run <file>

replacing <file> with the file containing the code you want the compiler run.

The compiler will output an object file with the same name as the input file, plus the `.out` extension. You can link this with your favourite linker to output an executable.

### Current problems
The library I am using as an interface for LLVM doesn't support variadic functions. Untill I fork it and add them to the library, as well as fix the issues in the `llvm-rs` repository, this compiler uses the nonstandard function `int print_int(int)` to print the numbers. Use your favourite native language to define the function in a separate `.out` file and link it with the outputed file of this compiler using your favourite linker.
