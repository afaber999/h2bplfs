use std::io::{self, Write};

pub mod frontend;
pub mod runtime;

use frontend::parser::Parser;
use runtime::interpreter::Interpreter;

fn repl() -> io::Result<()> {
    let mut interpreter = Interpreter::new();

    println!("Repl v0.14");
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
        
        let rtval = interpreter.evaluate(&program);
        println!("{:?}", rtval);

    }
    Ok(())
}

fn main() -> io::Result<()> {
    return repl();
}
