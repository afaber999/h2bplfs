use std::io::{self, Write};
use std::fs;

mod frontend;
mod runtime;

use frontend::parser::Parser;
use crate::runtime::{environment::Environment, interpreter::evaluate};

fn repl() -> io::Result<()> {
    let mut environment = Environment::new();
    let global_scope = environment.create_global_scope();

    let mut file_path = std::env::current_dir().unwrap();
    file_path.push("src");
    file_path.push("ep_08");
    file_path.push("test");
    file_path.set_extension("txt");

    println!("Repl v0.18, working dir: {:?}", &file_path);

    let mut buffer = fs::read_to_string(file_path).expect("Can't read file");
    let program = Parser::produce_ast(&buffer);
    println!("{:?}", program);
    
    let rtval = evaluate(&program, &mut environment, global_scope);
    println!("{:?}", rtval);
    
    //let src_code = "let x = ( 10 + 5 ) * 2".to_string();
    loop {
        print!(">");
        std::io::stdout().flush()?;

        buffer.clear();
        io::stdin().read_line(&mut buffer)?;
        if buffer.starts_with("exit") {
            break;
        }

        let program = Parser::produce_ast(&buffer);
        println!("{:?}", program);
        
        let rtval = evaluate(&program, &mut environment, global_scope);
        println!("{:?}", rtval);

    }
    Ok(())
}

fn main() -> io::Result<()> {
    return repl();
}
