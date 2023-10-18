use crate::frontend::ast::*;
use crate::runtime::values::*;
use super::super::environment::Environment;

pub fn evaluate_program(program : &AstProgram, environment: &mut Environment, scope : usize) -> RtValue {
    let mut last_evaluated = RtValue::NullVal;
    for stmt in &program.body {
        last_evaluated = super::evaluate(stmt, environment, scope);
    }
    last_evaluated
}


pub fn evaluate_var_declaration(declaration : &AstVarDeclaration, environment: &mut Environment, scope : usize) -> RtValue {

    // check clone
    let value = match declaration.value.clone() {
        Some(expr) => super::evaluate_expression(&expr, environment, scope),
        None => RtValue::NullVal,
    };

    environment.declare_variable(
        scope, 
        declaration.identifier.as_str(), 
        value, 
        declaration.constant,
     )
}