use crate::frontend::ast::AstStatement;
use crate::frontend::ast::*;
use crate::runtime::values::*;
use super::environment::Environment;

#[derive(Debug)]
pub struct Interpreter {
}

impl Interpreter {

    pub fn new() -> Self {Self {
    }}

    fn evaluate_program(self : &mut Self, program : &AstProgram, environment: &mut Environment, scope : usize) -> RtValue {
        let mut last_evaluated = RtValue::NullVal;
        for stmt in &program.body {
            last_evaluated = self.evaluate(stmt, environment, scope);
        }
        last_evaluated
    }

    fn evaluate_binary_expression(self : &mut Self, expression : &AstBinaryExpression, environment: &mut Environment, scope : usize) -> RtValue {
        let left_val = self.evaluate_expression(expression.left.as_ref(), environment, scope);
        let right_val = self.evaluate_expression(expression.right.as_ref(), environment, scope);

        match (&left_val, &right_val) {
                (RtValue::NumberVal(x),RtValue::NumberVal(y)) => {
\                    match expression.operator.as_str() {
                        "+" => RtValue::NumberVal(x + y), 
                        "-" => RtValue::NumberVal(x - y), 
                        "*" => RtValue::NumberVal(x * y), 
                        "/" => RtValue::NumberVal(x / y),
                        "%" => RtValue::NumberVal(x % y), 
                        _ => panic!("Invalid operator {}", expression.operator),
                    }
                    
                },
                _ => panic!("Invalid type for binary: left {:?} right {:?}", left_val, right_val), 
        }
    }

    fn evaluate_expression(self : &mut Self, expression : &AstExpression, environment: &mut Environment, scope : usize) -> RtValue {

        match expression {
            AstExpression::Binary(expression) => self.evaluate_binary_expression(expression, environment, scope),
            AstExpression::Identifier(name) => environment.get_value(scope, &name.symbol),
            AstExpression::NumericLiteral(x) => RtValue::NumberVal(x.value),
        }
    }

    pub fn evaluate(self : &mut Self, stmt : &AstStatement, environment: &mut Environment, scope : usize) -> RtValue {

        match stmt {
            AstStatement::Program(program) => self.evaluate_program(&program, environment, scope),
            AstStatement::Expression(expression) => self.evaluate_expression(&expression, environment, scope),
        }
    }
}