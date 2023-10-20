// hkindps://github.com/tlaceby/guide-to-interpreters-series
// -----------------------------------------------------------
// ---------------          LEXER          -------------------
// ---  Responsible for producing tokens from the source   ---
// -----------------------------------------------------------

// Represents tokens that our language understands in parsing.


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Location {
    pub line : usize,
    pub col : usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenKind {
	// Literal Types
	Number,
	Identifier,

	// Keywords
	Let,
	Const,

	// Grouping * Operators
	BinaryOperator,
	Equals,
	OpenParen,
	CloseParen,    
    OpenBrace,
    CloseBrace,

    // Other
    SemiColon,
    Comma,
    Colon,
    Eof,
}


#[derive(Debug)]
pub struct Token {
    pub value : String,
    pub kind  : TokenKind,
    pub location  : Location,
}

impl Token {
    pub fn new(value : String, kind : TokenKind, line : usize, col : usize) -> Self {
        Self {
            value,
            kind,
            location : Location {line,col}
        }
    }
}

#[derive(Debug)]
pub struct Keyword {
    text : &'static str,
    kind   : TokenKind,
}

impl Keyword {
    pub const fn new(text: &'static str, kind : TokenKind) -> Self {
        Self {
            text,
            kind,
        }
    } 
}

static KEYWORDS : [Keyword; 2] = [ Keyword::new( "let", TokenKind::Let),Keyword::new( "const", TokenKind::Const)];

fn is_integer(x: &char) -> bool {
    x.is_numeric()
}

fn is_alphanumeric(x: &char) -> bool {
    let extra_chars = "_.";
    x.is_alphanumeric() || extra_chars.contains(*x)
}


#[derive(Debug)]
pub struct Lexer {}

impl Lexer {

    pub fn new() -> Self {Self {}}

    pub fn tokenize(src_code : &String) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut src = src_code.chars().peekable();

        let mut line = 1;
        let mut col = 0;

        // build each token until end of file
        while let Some(c) = src.next() {

            col += 1;
            let mut value = c.to_string();

            match c {
                ',' => tokens.push(Token::new(value,TokenKind::Comma, line, col)),
                ':' => tokens.push(Token::new(value,TokenKind::Colon, line, col)),
                ';' => tokens.push(Token::new(value,TokenKind::SemiColon, line, col)),
                '{' => tokens.push(Token::new(value,TokenKind::OpenBrace, line, col)),
                '}' => tokens.push(Token::new(value,TokenKind::CloseBrace, line, col)),
                '(' => tokens.push(Token::new(value,TokenKind::OpenParen, line, col)),
                ')' => tokens.push(Token::new(value,TokenKind::CloseParen, line, col)),
                '+' | '-' | '*' | '/' | '%'  => tokens.push(Token::new(value,TokenKind::BinaryOperator, line, col)),
                '=' => tokens.push(Token::new(value,TokenKind::Equals, line, col)),

                '\n'=> {
                    line += 1;
                    col = 0;
                }
                ' ' | '\t' | '\r' => {
                    // ignore whitespace
                }
                _ => {
                    if c.is_numeric() {
                        while let Some(c) = src.next_if(is_integer) {
                            col += 1;
                            value.push(c);
                        }
                        tokens.push(Token::new(value,TokenKind::Number, line, col))
                    } else  if c.is_alphabetic() {
                        while let Some(c) = src.next_if(is_alphanumeric) {
                            col += 1;
                            value.push(c);
                        }

                        // check for reserve keywords
                        KEYWORDS.iter().for_each(|k| {
                            if k.text == &value {
                                println!("Special keywoard: {:?}", k.kind);
                                tokens.push(Token::new(k.text.to_string(),k.kind, line, col));
                                value.clear();                                
                            }
                        });

                        if value.len() > 0 {
                            tokens.push(Token::new(value,TokenKind::Identifier, line, col))
                        }
                    } else {
                        panic!("Can't tokenize character '{}'", c);
                    }
                } 
            }
        }
        
        // indicate end of tokens
        tokens.push(Token::new("".to_string(),TokenKind::Eof, line, col));
        tokens
    }

}

