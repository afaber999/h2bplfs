pub mod lexer;
use lexer::Lexer;

#[derive(Debug)]
pub enum NodeType {
    Program,
    NumericLiteral,
    Identifier,
    BinaryExpression,
}

// struct AstNode {
//     left : Option<AstNode>
// }


// trait AstNode {
//     fn node_type() -> NodeType;
// }

#[derive(Debug)]
struct Program {

}

impl Program {
    pub fn new() -> Self {
        Self {

        }
    }
}


#[derive(Debug)]
struct Parser {

}

impl Parser {
    pub fn new() -> Self {
        Self {

        }
    }

    pub fn produce_ast(src_code : &String) -> Program {
        
        let tokens = Lexer::tokenize(src_code);

        println!("lexer, EP_01!");
        for token in tokens {
            println!("Token: {:#?}", token);
        }    
            
        Program::new()
    }
}


fn main() {
    println!("how to build a programming language from scratch, EP_02!");
    let src_code = "let x = ( 10 + 5 ) * 2".to_string();
    Parser::produce_ast(&src_code);
}
