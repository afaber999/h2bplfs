// https://github.com/tlaceby/guide-to-interpreters-series
// -----------------------------------------------------------
// ---------------          LEXER          -------------------
// ---  Responsible for producing tokens from the source   ---
// -----------------------------------------------------------

// Represents tokens that our language understands in parsing.
    



#[derive(Debug, Clone, Copy)]
pub enum TokenType {
	// Literal Types
	Number,
	Identifier,

	// Keywords
	Let,

	// Grouping * Operators
	BinaryOperator,
	Equals,
	OpenParen,
	CloseParen,    
}


#[derive(Debug)]
pub struct Token {
    value : String,
    tt    : TokenType,
}


impl Token {
    pub fn new(value : String, tt : TokenType) -> Self {
        Self {
            value,
            tt,
        }
    }
}

#[derive(Debug)]
pub struct Keyword {
    text : &'static str,
    tt   : TokenType,
}

impl Keyword {
    pub const fn new(text: &'static str, tt : TokenType) -> Self {
        Self {
            text,
            tt,
        }
    } 
}

pub static keywords : [Keyword; 1] = [ Keyword::new( "let", TokenType::Let)];

fn is_integer(x: &char) -> bool {
    x.is_numeric()
}

fn is_alphabetic(x: &char) -> bool {
    let extra_chars = "_.";
    x.is_alphabetic() || extra_chars.contains(*x)
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
        
        // build each token until end of file
        while let Some(c) = src.next() {

            let mut value = c.to_string();

            match c {
                '(' => tokens.push(Token::new(value,TokenType::OpenParen)),
                ')' => tokens.push(Token::new(value,TokenType::CloseParen)),
                '+' | '-' | '*' | '/'  => tokens.push(Token::new(value,TokenType::BinaryOperator)),
                '=' => tokens.push(Token::new(value,TokenType::Equals)),

                ' ' | '\t' | '\n' | '\r' => {
                    // ignore whitespace
                }
                _ => {
                    if c.is_numeric() {
                        while let Some(c) = src.next_if(is_integer) {
                            value.push(c);
                        }
                        tokens.push(Token::new(value,TokenType::Number))
                    } else  if c.is_alphabetic() {
                        while let Some(c) = src.next_if(is_alphanumeric) {
                            value.push(c);
                        }

                        // check for reserve keywords
                        keywords.iter().for_each(|k| {
                            if k.text == &value {
                                tokens.push(Token::new(k.text.to_string(),k.tt));
                                value.clear();                                
                            }
                        });

                        if value.len() > 0 {
                            tokens.push(Token::new(value,TokenType::Identifier))
                        }
                    } else {
                        panic!("Can't tokenize character {}", c);
                    }
                } 
            }
        }

        tokens
    }
}


fn main() {
    let src_code = "let x = ( 10 + 5 ) * 2".to_string();
    let tokens = Lexer::tokenize(&src_code);

    println!("lexer, EP_01!");
    for token in tokens {
        println!("Token: {:?}", token);
    }
}
