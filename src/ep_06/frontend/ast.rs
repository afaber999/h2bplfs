
#[derive(Debug, Clone)]
pub enum AstStatement {
    Program(AstProgram),
    VarDeclaration(AstVarDeclaration),
    Expression(AstExpression),
}

#[derive(Debug, Clone)]
pub enum AstExpression {
    Binary(AstBinaryExpression),
    Identifier(AstIdentifier),
    NumericLiteral(AstNumericLiteral),
}

#[derive(Debug, Clone)]
pub struct AstBinaryExpression {
    pub left : Box<AstExpression>,
    pub right : Box<AstExpression>,
    pub operator : String,
}

#[derive(Debug, Clone)]
pub struct AstIdentifier {
    pub symbol : String,
}

#[derive(Debug, Clone)]
pub struct AstNumericLiteral {
    pub value : f64,
}

#[derive(Debug, Clone)]
pub struct AstProgram {
    pub body: Vec<AstStatement>,
}
#[derive(Debug, Clone)]
pub struct AstVarDeclaration {
    pub constant : bool,
    pub identifier : String,
    pub value : Option<AstExpression>,
}


pub fn create_null_expression() -> AstExpression {
    AstExpression::Identifier( 
        AstIdentifier { symbol: "null".to_string()  }
    )
}
