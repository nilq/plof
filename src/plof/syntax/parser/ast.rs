use std::rc::Rc;

use super::{ParserResult, ParserError};
use super::super::{SymTab, Env};

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
        param_names: Vec<Rc<String>>,
        param_types: Vec<Type>,
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

impl Expression {
    pub fn visit(&self, sym: &Rc<SymTab>, env: &Rc<Env>) -> ParserResult<()> {
        match *self {
            Expression::Identifier(ref id) => match sym.get_name(&*id) {
                Some((i, env_index)) => {
                    println!("found thing: {} of {:?}", id, env.get_type(i, env_index));
                    Ok(())
                },
                None => Err(ParserError::new(&format!("use of undeclared: {}", id))),
            },

            Expression::Assignment(ref t, ref name, ref expr) => {
                let index = sym.add_name(name);
                if index >= env.size() {
                    env.grow();
                }

                let tp = match *t {
                    Some(ref id) => id.clone(),
                    None         => Type::Any,
                };

                if let Err(e) = env.set_type(index, 0, tp) {
                    panic!("error setting type: {}", e)
                }

                Ok(())
            },

            Expression::Lambda {
                ref name, ref retty, ref param_names, ref param_types, ref body,
            } => {
                if let &Some(ref n) = name {
                    match sym.get_name(&n) {
                        Some((_, _)) => return Err(ParserError::new(&format!("can't redefine lambda '{}'!", n))),
                        None => {
                            let index = sym.add_name(&n);
                            if index >= env.size() {
                                env.grow();
                            }
                        },
                    }
                }

                let local_sym = Rc::new(SymTab::new(sym.clone(), &param_names));
                let local_env = Rc::new(Env::new(env.clone(), &param_types));

                println!("lambda: {:?} of type {:?}:\n", name.clone().unwrap(), retty);

                local_sym.visualize(1);
                local_env.visualize(1);

                println!("\n");

                Ok(())
            },

            ref c => Err(ParserError::new(&format!("undefined visitor: {:?}", c))),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Statement {
    Expression(Rc<Expression>),
}

impl Statement {
    pub fn visit(&self, sym: &Rc<SymTab>, env: &Rc<Env>) -> ParserResult<()> {
        match *self {
            Statement::Expression(ref e) => e.visit(sym, env),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Type {
    Str, Num, Bool, Any, Nil, Undefined,
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
