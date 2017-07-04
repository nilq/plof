use std::rc::Rc;

use super::{ParserResult, ParserError};

#[derive(Debug, Clone)]
pub enum Expression {
    Block(Rc<Vec<Statement>>),
    NumberLiteral(f64),
    StringLiteral(Rc<String>),
    Identifier(Rc<String>),
    BoolLiteral(bool),
    Assignment(Option<Type>, Rc<String>, Rc<Expression>),
    Call(Rc<Expression>, Rc<Vec<Expression>>),
    Lambda {
        name:       Option<Rc<String>>,
        retty:      Type,
        parameters: Vec<(Option<Type>, Rc<String>)>,
        body:       Rc<Expression>,
    },
    Nil,
    EOF,
    Operation {
        left:  Rc<Expression>,
        op:    Operand,
        right: Rc<Expression>,
    }
}

#[derive(Debug, Clone)]
pub enum Statement {
    Expression(Rc<Expression>),
}

#[derive(Debug, Clone)]
pub enum Type {
    Str, Num, Bool, Any, Nil,
}

pub fn get_type(v: &str) -> Option<Type> {
    match v {
        "str"  => Some(Type::Str),
        "num"  => Some(Type::Num),
        "bool" => Some(Type::Bool),
        "any"  => Some(Type::Any),
        "nil"  => Some(Type::Nil),
        _      => None,
    }
}

#[derive(Debug, Clone)]
pub enum Operand {
    Pow,
    Mul, Div, Mod,
    Add, Sub,
    Equal, NEqual,
    Lt, Gt, LtEqual, GtEqual,
    And, Or, Not,
    Append,
}

pub fn get_operand(v: &str) -> Option<(Operand, u8)> {
    match v {
        "^"   => Some((Operand::Pow, 0)),
        "*"   => Some((Operand::Mul, 1)),
        "/"   => Some((Operand::Div, 1)),
        "%"   => Some((Operand::Mod, 1)),
        "+"   => Some((Operand::Add, 2)),
        "-"   => Some((Operand::Sub, 2)),
        "=="  => Some((Operand::Equal, 3)),
        "!="  => Some((Operand::NEqual, 3)),
        "<"   => Some((Operand::Lt, 4)),
        ">"   => Some((Operand::Gt, 4)),
        "<="  => Some((Operand::LtEqual, 4)),
        ">="  => Some((Operand::GtEqual, 4)),
        "!"   => Some((Operand::Not, 4)),
        "and" => Some((Operand::And, 4)),
        "or"  => Some((Operand::Or, 4)),
        "++"  => Some((Operand::Append, 5)),
        _ => None,
    }
}
