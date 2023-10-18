use std::{iter::Peekable, slice::Iter};
use crate::frontend::lexer::{Lexer, Token, TokenKind};
use crate::frontend::ast::*;

#[derive(Debug)]
pub struct Parser {}

type TokenIter<'a> = Peekable<Iter<'a,Token>>;


impl Parser {
    pub fn new() -> Self {Self {}}

    fn at<'a>(tokens : &'a mut TokenIter) -> &'a Token {
        let nt = tokens.peek();
        nt.unwrap()
    }

    fn eat<'a>(tokens : &'a mut TokenIter) -> &'a Token {
        let nt = tokens.next();
        nt.unwrap()
    }

    fn expect<'a>(tokens : &'a mut TokenIter, kind: TokenKind, msg: &str ) -> &'a Token {
        let nt = tokens.next();
        let nt = nt.unwrap();
        if nt.kind != kind {
            panic!( "{} got {:?}", msg, nt.kind);
        }
        nt
    }

    fn parse_primary_expression(tokens : &mut TokenIter) -> Option<AstExpression> {
        
        let next_token = Self::at(tokens);
        match next_token.kind {
            TokenKind::Null => { 
                let token = Self::eat(tokens);
                Some(AstExpression::NullLiteral(AstNullLiteral{}))
            }, 
            TokenKind::Number => { 
                let token = Self::eat(tokens);
                if let Ok(value) = token.value.parse::<f64>() {
                    let id = AstNumericLiteral{ value,};
                    let e = AstExpression::NumericLiteral(id);
                    Some(e)
                } else {
                    None
                }
            }, 
            TokenKind::Identifier => { 
                let token = Self::eat(tokens);
                let id = AstIdentifier{ symbol : token.value.clone(),};
                let e = AstExpression::Identifier(id);
                Some(e)
            } ,
            TokenKind::OpenParen => { 
                Self::expect(tokens, TokenKind::OpenParen, "Expected open paren in parenterized expression");
                let e = Self::parse_expression(tokens).unwrap();
                Self::expect(tokens, TokenKind::CloseParen, "Expected close paren in parenterized expression");
                Some(e)
            } ,
            _ => {
                panic!("Unknown token in primary expression: {:?}", next_token);
            },
        }
    }

    fn parse_multiplicitave_expression(tokens : &mut TokenIter) -> Option<AstExpression> {

        let mut left = Self::parse_primary_expression(tokens).unwrap();
        let next_token = Self::at(tokens);

        if next_token.value == "*" || next_token.value == "/" || next_token.value == "%" {
            let operator = Self::eat(tokens).value.clone();
            let right = Self::parse_primary_expression(tokens).unwrap();
            let bin_expr = AstBinaryExpression{ 
                    operator : operator,
                    left : Box::new(left),
                    right: Box::new(right),
            };
            left = AstExpression::Binary(bin_expr);
        } 
        Some(left)
    }

    fn parse_additive_expression(tokens : &mut TokenIter) -> Option<AstExpression> {

        let mut left = Self::parse_multiplicitave_expression(tokens).unwrap();
        let next_token = Self::at(tokens);

        if next_token.value == "+" || next_token.value == "-" {
            let operator = Self::eat(tokens).value.clone();
            let right = Self::parse_multiplicitave_expression(tokens).unwrap();
            let bin_expr = AstBinaryExpression{ 
                    operator : operator,
                    left : Box::new(left),
                    right: Box::new(right),
            };
            left = AstExpression::Binary(bin_expr);
        } 
        Some(left)
    }

    fn parse_expression(tokens : &mut TokenIter) -> Option<AstExpression> {
        
        let expr_option = Self::parse_additive_expression(tokens);
        if expr_option.is_some() {
            return expr_option;
        } else {
            None
        }
    }

    fn parse_statement(tokens : &mut TokenIter) -> Option<AstStatement> {
        // parse when implementing future statements=
        match Self::parse_expression(tokens) {
            Some(expr) => Some(AstStatement::Expression(expr)),
            None => None,
        }
    }

    pub fn produce_ast(src_code : &String) -> AstStatement {
        
        let tokens = Lexer::tokenize(src_code);
        let mut tokens : TokenIter = tokens.iter().peekable();

        let mut program = AstProgram{ body : Vec::new()};

        loop{
            match Self::parse_statement(&mut tokens) {
                Some(body) => program.body.push(body),
                None => {}  
            }

            if let Some( nt ) = tokens.peek() {
                println!("CHECK TOKEN: {:?}", nt.kind);
                if nt.kind == TokenKind::Eof {
                    break;
                }
            } else {
                break;
            }
        }
        AstStatement::Program(program)
    }
}

