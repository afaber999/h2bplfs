use std::io::{self, Write};
pub mod lexer;
pub mod ast;
pub mod parser;

use parser::Parser;

fn repl() -> io::Result<()> {
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
    }
    Ok(())
}

fn main() -> io::Result<()> {
    return repl();
}
