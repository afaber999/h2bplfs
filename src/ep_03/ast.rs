
#[derive(Debug, Clone)]
pub enum AstStatement {
    Program(AstProgram),
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
