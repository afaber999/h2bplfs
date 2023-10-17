use crate::frontend::ast::AstStatement;
use crate::frontend::ast::*;
use crate::runtime::values::*;

#[derive(Debug)]
pub struct Interpreter {
}

impl Interpreter {

    pub fn new() -> Self {Self {
    }}

    fn evaluate_program(self : &mut Self, program : &AstProgram) -> RtValue {
        let mut last_evaluated = RtValue::NullVal;
        for stmt in &program.body {
            last_evaluated = self.evaluate(stmt);
        }
        last_evaluated
    }

    fn evaluate_binary_expression(self : &mut Self, expression : &AstBinaryExpression) -> RtValue {
        let left_val = self.evaluate_expression(expression.left.as_ref());
        let right_val = self.evaluate_expression(expression.right.as_ref());

        match (&left_val, &right_val) {
                (RtValue::NumberVal(x),RtValue::NumberVal(y)) => {
                    println!("BINARY!!!!!!!!!!!!!!!!!!!!!!!11 {} {}", x, y);
                    match expression.operator.as_str() {
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

    fn evaluate_expression(self : &mut Self, expression : &AstExpression) -> RtValue {

        match expression {
            AstExpression::Binary(expression) => self.evaluate_binary_expression(expression),
            AstExpression::Identifier(_) => todo!(),
            AstExpression::NumericLiteral(x) => RtValue::NumberVal(x.value),
            AstExpression::NullLiteral(_) => RtValue::NullVal,
        }
    }

    pub fn evaluate(self : &mut Self, stmt : &AstStatement) -> RtValue {

        match stmt {
            AstStatement::Program(program) => self.evaluate_program(&program),
            AstStatement::Expression(expression) => self.evaluate_expression(&expression),
        }
    }
}