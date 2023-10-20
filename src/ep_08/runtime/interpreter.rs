use crate::frontend::ast::AstStatement;
use crate::runtime::values::*;
use super::environment::Environment;

pub mod statements;
pub mod expressions;

use statements::*;
use expressions::*;

pub fn evaluate(stmt : &AstStatement, environment: &mut Environment, scope : usize) -> RtValue {

    match stmt {
        AstStatement::Program(program) => evaluate_program(&program, environment, scope),
        AstStatement::Expression(expression) => evaluate_expression(&expression, environment, scope),
        AstStatement::VarDeclaration(declaration) => evaluate_var_declaration(&declaration, environment, scope),
    }
}
