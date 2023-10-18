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

    // let scope2 = environment.add_scope(global_scope);
    // let scope3 = environment.add_scope(scope2);
    // let scope4 = environment.add_scope(global_scope);


    // environment.declare_variable(global_scope, "four", runtime::values::RtValue::NumberVal(4.0));
    // environment.declare_variable(global_scope, "pi", runtime::values::RtValue::NumberVal(3.14159267));
    // environment.declare_variable(global_scope, "alsofour", runtime::values::RtValue::NumberVal(4.0));
    // environment.declare_variable(scope4, "fourscopefour", runtime::values::RtValue::NumberVal(4.0));


    // environment.list_scope(global_scope);
    // environment.list_scope(scope2);
    // environment.list_scope(scope3);
    // environment.list_scope(scope4);


    println!("Get value 1 {:?}", environment.get_value(global_scope, "pi"));
    println!("Get value 2 {:?}", environment.get_value(global_scope, "two_pi"));
    println!("Get value 3 {:?}", environment.get_value(global_scope, "poop"));

    environment.assign_value(scope3, "poop", runtime::values::RtValue::NumberVal(12.0));
    println!("Get value 3 {:?}", environment.get_value(scope3, "poop"));

    environment.declare_variable(scope3, "poop", runtime::values::RtValue::NumberVal(13.0));

    println!("Get value 4 {:?}", environment.get_value(global_scope, "poop"));
    println!("Get value 5 {:?}", environment.get_value(scope3, "poop"));

    println!("Repl v0.15");
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
