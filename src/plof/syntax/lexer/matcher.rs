use super::Tokenizer;
use super::token::{Token, TokenType};

macro_rules! token {
    ($tokenizer:expr, $token_type:ident, $accum:expr) => {{
        token!($tokenizer , TokenType::$token_type, $accum)
    }};
    ($tokenizer:expr, $token_type:expr, $accum:expr) => {{
        let tokenizer = $tokenizer as &$crate::xxx::syntax::lexer::Tokenizer;
        let token_type = $token_type as $crate::xxx::syntax::lexer::token::TokenType;
        Some(Token::new(token_type, tokenizer.last_position(), $accum))
    }};
}

pub trait Matcher {
    fn try_match(&self, tokenizer: &mut Tokenizer) -> Option<Token>;
}

pub struct WhitespaceMatcher;

impl Matcher for WhitespaceMatcher {
    fn try_match(&self, tokenizer: &mut Tokenizer) -> Option<Token> {
        let mut found = false;
        while !tokenizer.end() && tokenizer.peek().unwrap().is_whitespace() {
            found = true;
            tokenizer.next();
        }
        if found {
            token!(tokenizer, Whitespace, String::new())
        } else {
            None
        }
    }
}

pub struct IntLiteralMatcher {}

impl Matcher for IntLiteralMatcher {
    fn try_match(&self, tokenizer: &mut Tokenizer) -> Option<Token> {
        let mut accum = String::new();
        let prefix = match tokenizer.peek() {
            Some(&'-') => Some(false),
            Some(&'+') => Some(true),
            _          => None,
        };
        if let Some(_) = prefix {
            tokenizer.advance(1)
        };
        while !tokenizer.end() && tokenizer.peek().unwrap().is_digit(10) {
            accum.push(tokenizer.next().unwrap());
        }
        if !accum.is_empty() {
            let literal: String = if Some(false) == prefix {
                match i64::from_str_radix(accum.as_str(), 10) {
                    Ok(result) => format!("-{}", result),
                    Err(error) => panic!("unable to parse int-literal: {}", error)
                }
            } else {
                match u64::from_str_radix(accum.as_str(), 10) {
                    Ok(result) => result.to_string(),
                    Err(error) => panic!("unable to parse int-literal: {}", error)
                }
            };
            token!(tokenizer, IntLiteral, literal)
        } else {
            None
        }
    }
}

pub struct FloatLiteralMatcher;

impl Matcher for FloatLiteralMatcher {
    fn try_match(&self, tokenizer: &mut Tokenizer) -> Option<Token> {
        let mut accum = String::new();

        let prefix = match tokenizer.peek() {
            Some(&'-') => Some(false),
            Some(&'+') => Some(true),
            _          => None,
        };
        if let Some(_) = prefix {
            tokenizer.advance(1)
        };

        let curr = tokenizer.next().unwrap();
        if curr.is_digit(10) {
            accum.push(curr)
        } else if curr == '.' {
            accum.push_str("0.")
        } else {
            return None
        }
        while !tokenizer.end() {
            let current = *tokenizer.peek().unwrap();
            if !current.is_whitespace() && current.is_digit(10) || current == '.' {
                if current == '.' && accum.contains('.') {
                    panic!("illegal decimal point")
                }
                accum.push(tokenizer.next().unwrap())
            } else {
                break
            }
        }

        if accum == "0.".to_owned() {
            None
        } else if accum.contains('.') {

            let literal: String = if Some(false) == prefix {
                match accum.parse::<f64>() {
                    Ok(result) => format!("-{}", result),
                    Err(error) => panic!("unable to parse float-literal: {}", error)
                }
            } else {
                match accum.parse::<f64>() {
                    Ok(result) => result.to_string(),
                    Err(error) => panic!("unable to parse float-literal: {}", error)
                }
            };

            token!(tokenizer, FloatLiteral, literal)
        } else {
            let literal: String = if Some(false) == prefix {
                match i64::from_str_radix(accum.as_str(), 10) {
                    Ok(result) => format!("-{}", result),
                    Err(error) => panic!("unable to parse int-literal: {}", error)
                }
            } else {
                match u64::from_str_radix(accum.as_str(), 10) {
                    Ok(result) => result.to_string(),
                    Err(error) => panic!("unable to parse int-literal: {}", error)
                }
            };

            token!(tokenizer, IntLiteral, literal)
        }
    }
}

pub struct StringLiteralMatcher;

impl Matcher for StringLiteralMatcher {
    fn try_match(&self, tokenizer: &mut Tokenizer) -> Option<Token> {
        let mut raw_marker = false;
        let delimeter  = match tokenizer.peek().unwrap() {
            &'"'  => Some('"'),
            &'\'' => Some('\''),
            &'r' => match *tokenizer.peek_n(1).unwrap() {
                c @ '"' | c @ '\'' => {
                    raw_marker = true;
                    tokenizer.advance(1); // Skips prefix
                    Some(c)
                },
                _ => return None,
            },
            _ => return None,
        }.expect("Invalid delimiter");
        tokenizer.advance(1); // Skips the opening delimeter
        let mut string       = String::new();
        let mut found_escape = false;
        loop {
            if tokenizer.end() {
                break
            }
            if raw_marker {
                let c = *tokenizer.peek().unwrap();
                if c == delimeter {
                    break
                }
                string.push(tokenizer.next().unwrap())
            } else {
                if found_escape {
                    string.push(
                        match tokenizer.next().unwrap() {
                            c @ '\\' | c @ '\'' | c @ '"' => c,
                            'n' => '\n',
                            'r' => '\r',
                            't' => '\t',
                            s => panic!("unwanted character escape: {}", s),
                        }
                    );
                    found_escape = false
                } else {
                    match tokenizer.peek().unwrap() {
                        &'\\' => {
                            tokenizer.next();
                            found_escape = true
                        },
                        &c if &c == &delimeter => break,
                        _ => string.push(tokenizer.next().unwrap()),
                    }
                }
            }
        }
        tokenizer.advance(1); // Skips the closing delimeter
        token!(tokenizer, StringLiteral, string)
    }
}

pub struct ConstantMatcher {
    token_type: TokenType,
    constants: Vec<String>,
}

impl ConstantMatcher {
    pub fn new(token_type: TokenType, constants: Vec<String>) -> Self {
        ConstantMatcher {
            token_type: token_type,
            constants: constants,
        }
    }
}

impl Matcher for ConstantMatcher {
    fn try_match(&self, tokenizer: &mut Tokenizer) -> Option<Token> {
        for constant in self.constants.clone() {
            let dat = tokenizer.clone().take(constant.len());
            if dat.size_hint().1.unwrap() != constant.len() {
                return None;
            }
            if dat.collect::<String>() == constant {
                tokenizer.advance(constant.len());
                return token!(tokenizer, self.token_type.clone(), constant)
            }
        }
        None
    }
}

pub struct KeyMatcher {
    token_type: TokenType,
    keys: Vec<String>,
}

impl KeyMatcher {
    pub fn new(token_type: TokenType, keys: Vec<String>) -> Self {
        KeyMatcher {
            token_type: token_type,
            keys: keys,
        }
    }
}

impl Matcher for KeyMatcher {
    fn try_match(&self, tokenizer: &mut Tokenizer) -> Option<Token> {
        for key in self.keys.clone() {
            let dat = tokenizer.clone().take(key.len());
            if dat.size_hint().1.unwrap() != key.len() {
                return None;
            }

            match tokenizer.clone().peek_n(dat.size_hint().1.unwrap()) {
                Some(c) if c.is_digit(10) || c.is_alphabetic() => return None,
                _ => if dat.collect::<String>() == key {
                    tokenizer.advance(key.len());
                    return token!(tokenizer, self.token_type.clone(), key)
                },
            }
        }
        None
    }
}

pub struct IdentifierMatcher;

impl Matcher for IdentifierMatcher {
    fn try_match(&self, tokenizer: &mut Tokenizer) -> Option<Token> {
        let mut identifier = String::new();
        while !tokenizer.end() {
            let current = *tokenizer.peek().unwrap();
            if !current.is_whitespace() && ("_@?'".contains(current) || current.is_alphanumeric()) {
                identifier.push(tokenizer.next().unwrap());
            } else {
                break
            }
        }
        if !identifier.is_empty() {
            token!(tokenizer, Identifier, identifier)
        } else {
            None
        }
    }
}
