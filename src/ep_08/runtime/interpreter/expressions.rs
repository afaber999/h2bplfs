use std::collections::HashMap;
use std::ops::Deref;

use crate::frontend::ast::*;
use crate::runtime::values::*;
use super::super::environment::Environment;

fn evaluate_binary_expression(expression : &AstBinaryExpression, environment: &mut Environment, scope : usize) -> RtValue {
    let left_val = evaluate_expression(expression.left.as_ref(), environment, scope);
    let right_val = evaluate_expression(expression.right.as_ref(), environment, scope);

    match (&left_val, &right_val) {
            (RtValue::NumberVal(x),RtValue::NumberVal(y)) => {
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

fn evaluate_assignment_expression(assignment : &AstAssignmentExpression, environment: &mut Environment, scope : usize) -> RtValue {

    let name;
    //let assignee = &assignment.assignee; 
    match &assignment.assignee.deref() {
        AstExpression::Identifier(id) => name = id.symbol.clone(),
        _ => panic!("Invalid assignee in assignment expression, got {:?}", 1)
    }

    let value = evaluate_expression( &assignment.value, environment, scope );

    environment.assign_value(scope, &name, value)
}

fn evaluate_object_expression(object : &AstObjectLiteral, environment: &mut Environment, scope : usize) -> RtValue {

    let mut properties = HashMap::new();

    for ast_prop in &object.properties {
        let k = ast_prop.key.clone(); 
        let rt_val = match &ast_prop.value {
            Some(expr) => {
                evaluate_expression( &expr, environment, scope )
            },
            None =>  {
                environment.get_value(scope, &k)
            },
        };
        properties.insert(k, rt_val);
    }

    RtValue::Object(ObjectRtVal{ properties })
}


pub fn evaluate_expression(expression : &AstExpression, environment: &mut Environment, scope : usize) -> RtValue {

    match expression {
        AstExpression::Binary(expression) => evaluate_binary_expression(expression, environment, scope),
        AstExpression::Identifier(name) => environment.get_value(scope, &name.symbol),
        AstExpression::NumericLiteral(x) => RtValue::NumberVal(x.value),
        AstExpression::Assignment(assignment) => evaluate_assignment_expression(assignment, environment, scope),
        AstExpression::ObjectLiteral(object) => evaluate_object_expression(object, environment, scope),
        AstExpression::Property(_) => todo!(),
    }
}
