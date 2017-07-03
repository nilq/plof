use super::Tokenizer;
use super::matcher::*;
use super::token::{Token, TokenType, TokenPosition};
use super::block_tree::{ChunkValue, Branch, Chunk};

use std::str::Chars;
use std::rc::Rc;

pub fn lexer(data: &mut Chars) -> Lexer {
    let tokenizer = Tokenizer::new(data);
    let mut lexer = Lexer::new(tokenizer);

    let symbols = vec![
        "(",
        ")",
        "[",
        "]",
        ",",
        ":",
        "!",
        "{",
        "}",
        "!",
        "|",
        ".",
    ].iter().map(|&x| x.to_string()).collect();

    let operators = vec![
        "++",
        "+",
        "-",
        "*",
        "/",
        "/",
        "%",
        "^",
        ">",
        "<",
        ">=",
        "<=",
        "==",
        "!=",
        "=",
    ].iter().map(|&x| x.to_string()).collect();

    let keywords = vec![
        "if", "else", "elif",
    ].iter().map(|&x| x.to_string()).collect();

    let types = vec![
        "num", "str", "bool", "any", "nil"
    ].iter().map(|&x| x.to_string()).collect();

    let boolean = vec![
        "true",
        "false",
    ].iter().map(|&x| x.to_string()).collect();

    let matcher_symbol         = ConstantMatcher::new(TokenType::Symbol, symbols);
    let matcher_operator       = ConstantMatcher::new(TokenType::Operator, operators);
    let matcher_types          = KeyMatcher::new(TokenType::Keyword, types);
    let matcher_keyword        = KeyMatcher::new(TokenType::Keyword, keywords);
    let matcher_boolean        = KeyMatcher::new(TokenType::BoolLiteral, boolean);
    let matcher_whitespace     = WhitespaceMatcher {};
    let matcher_int_literal    = IntLiteralMatcher {};
    let matcher_float_literal  = FloatLiteralMatcher {};
    let matcher_identifier     = IdentifierMatcher {};
    let matcher_string_literal = StringLiteralMatcher {};

    lexer.matchers_mut().push(Rc::new(matcher_whitespace));
    lexer.matchers_mut().push(Rc::new(matcher_float_literal));
    lexer.matchers_mut().push(Rc::new(matcher_int_literal));
    lexer.matchers_mut().push(Rc::new(matcher_string_literal));
    lexer.matchers_mut().push(Rc::new(matcher_boolean));
    lexer.matchers_mut().push(Rc::new(matcher_types));
    lexer.matchers_mut().push(Rc::new(matcher_keyword));
    lexer.matchers_mut().push(Rc::new(matcher_symbol));
    lexer.matchers_mut().push(Rc::new(matcher_operator));
    lexer.matchers_mut().push(Rc::new(matcher_identifier));
    lexer
}

pub fn lex_branch(branch: &Branch) -> Branch {
    let mut lexed_branch = Branch::new(Vec::new());
    for c in branch.value.iter() {
        match c.value() {
            &ChunkValue::Source(ref s) => {
                let mut line: Vec<Token> = lexer(&mut s.clone().chars()).collect();
                line.push(Token::new(TokenType::EOL, TokenPosition::default(), "\n".to_owned()));

                let chunk = ChunkValue::Tokens(line);
                lexed_branch.value.push(Chunk::new(chunk))
            },
            &ChunkValue::Block(ref b) => {
                let chunk = ChunkValue::Block(lex_branch(&b));
                lexed_branch.value.push(Chunk::new(chunk))
            },
            _ => (),
        }
    }
    lexed_branch
}

pub fn flatten_branch(branch: &Branch) -> Vec<Token> {
    let mut flat = Vec::new();
    for c in branch.value.iter() {
        match c.value() {
            &ChunkValue::Tokens(ref t) => flat.append(&mut t.clone()),
            &ChunkValue::Block(ref b)  => flat.push(Token::new(TokenType::Block(flatten_branch(b)), TokenPosition::new(0, 0), "".to_string())),
            _ => continue,
        }
    }
    flat
}

pub fn process_branch(branch: &Branch) -> Vec<Token> {
    flatten_branch(&lex_branch(branch))
}

pub struct Lexer {
    tokenizer: Tokenizer,
    matchers: Vec<Rc<Matcher>>,
}

#[allow(dead_code)]
impl Lexer {
    pub fn new(tokenizer: Tokenizer) -> Lexer {
        Lexer {
            tokenizer,
            matchers: Vec::new(),
        }
    }

    pub fn match_token(&mut self) -> Option<Token> {
        for matcher in &mut self.matchers {
            match self.tokenizer.try_match_token(matcher.as_ref()) {
                Some(t) => return Some(t),
                None => continue,
            }
        }
        None
    }

    pub fn matchers(&self) -> &Vec<Rc<Matcher>> {
        &self.matchers
    }

    pub fn matchers_mut(&mut self) -> &mut Vec<Rc<Matcher>> {
        &mut self.matchers
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        let token = self.match_token().unwrap();
        match token.token_type {
            TokenType::EOF => None,
            TokenType::Whitespace => {
                match self.next() {
                    Some(t) => Some(t),
                    None => None,
                }
            }
            _ => Some(token),
        }
    }
}
