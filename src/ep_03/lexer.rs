// hkindps://github.com/tlaceby/guide-to-interpreters-series
// -----------------------------------------------------------
// ---------------          LEXER          -------------------
// ---  Responsible for producing tokens from the source   ---
// -----------------------------------------------------------

// Represents tokens that our language understands in parsing.

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenKind {
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

    // Other
    Eof,
}


#[derive(Debug)]
pub struct Token {
    pub value : String,
    pub kind  : TokenKind,
}

impl Token {
    pub fn new(value : String, kind : TokenKind) -> Self {
        Self {
            value,
            kind,
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

static KEYWORDS : [Keyword; 1] = [ Keyword::new( "let", TokenKind::Let)];

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
        
        // build each token until end of file
        while let Some(c) = src.next() {

            let mut value = c.to_string();

            match c {
                '(' => tokens.push(Token::new(value,TokenKind::OpenParen)),
                ')' => tokens.push(Token::new(value,TokenKind::CloseParen)),
                '+' | '-' | '*' | '/' | '%'  => tokens.push(Token::new(value,TokenKind::BinaryOperator)),
                '=' => tokens.push(Token::new(value,TokenKind::Equals)),

                ' ' | '\t' | '\n' | '\r' => {
                    // ignore whitespace
                }
                _ => {
                    if c.is_numeric() {
                        while let Some(c) = src.next_if(is_integer) {
                            value.push(c);
                        }
                        tokens.push(Token::new(value,TokenKind::Number))
                    } else  if c.is_alphabetic() {
                        while let Some(c) = src.next_if(is_alphanumeric) {
                            value.push(c);
                        }

                        // check for reserve keywords
                        KEYWORDS.iter().for_each(|k| {
                            if k.text == &value {
                                tokens.push(Token::new(k.text.to_string(),k.kind));
                                value.clear();                                
                            }
                        });

                        if value.len() > 0 {
                            tokens.push(Token::new(value,TokenKind::Identifier))
                        }
                    } else {
                        panic!("Can't tokenize character '{}'", c);
                    }
                } 
            }
        }
        
        // indicate end of tokens
        tokens.push(Token::new("".to_string(),TokenKind::Eof));
        tokens
    }

}

