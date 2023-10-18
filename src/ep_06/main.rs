use std::io::{self, Write};

pub mod frontend;
pub mod runtime;

use frontend::parser::Parser;
use runtime::interpreter::Interpreter;

use crate::runtime::environment::Environment;

fn repl() -> io::Result<()> {
    let mut interpreter = Interpreter::new();
    let mut environment = Environment::new();
    let global_scope = environment.create_global_scope();

    println!("Repl v0.16");
    //let src_code = "let x = ( 10 + 5 ) * 2".to_string();
    loop {
        print!(">");
        std::io::stdout().flush()?;

        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        if buffer.starts_with("exit") {
            break;
        }

        let program = Parser::produce_ast(&buffer);
        println!("{:?}", program);
        
        let rtval = interpreter.evaluate(&program, &mut environment, global_scope);
        println!("{:?}", rtval);

    }
    Ok(())
}

fn main() -> io::Result<()> {
    return repl();
}
