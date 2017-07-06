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
    Definition(Option<Type>, Rc<String>, Rc<Expression>),
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

            Expression::Definition(ref t, ref name, ref expr) => {
                try!(expr.visit(sym, env));

                let tp = match *t {
                    Some(ref tt) => {
                        if *tt != try!(expr.get_type(sym, env)) {
                            return Err(ParserError::new(&format!("right hand doesn't match type of: {}", name)))
                        }
                         tt.clone()
                    },
                    None         => try!(expr.get_type(sym, env)),
                };

                match sym.get_name(&name) {
                    Some((i, env_index)) => {
                        match env.get_type(i, env_index) {
                            Ok(tp2)  => if tp2 != tp {
                                println!("angery potential bad typing")
                            } else {
                                println!("might be okok")
                            },
                            Err(e) => return Err(ParserError::new(&format!("{}", e))),
                        }
                    },
                    None => (),
                }

                let index = sym.add_name(name);
                if index >= env.size() {
                    env.grow();
                }

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

                            if let Err(e) = env.set_type(index, 0, try!(self.get_type(sym, env))) {
                                panic!("error setting type: {}", e)
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

            Expression::Call(ref id, ref args) => {
                match try!(id.get_type(sym, env)) {
                    Type::Lambda(ref params) => {
                        let mut arg_types = Vec::new();

                        for arg in args.iter() {
                            arg_types.push(try!(arg.get_type(sym, env)));
                        }

                        if params[1..].to_vec() != arg_types.as_slice() {
                            Err(ParserError::new(&format!("can't invoke lambda with bad args!")))
                        } else {
                            println!("called '{:?}' with '{:?}'", id, args);
                            Ok(())
                        }
                    },

                    _ => Err(ParserError::new(&format!("can't call non-lambda"))),
                }
            }

            _ => Ok(()),
        }
    }

    pub fn get_type(&self, sym: &Rc<SymTab>, env: &Rc<Env>) -> ParserResult<Type> {
        match *self {
            Expression::NumberLiteral(_)  => Ok(Type::Num),
            Expression::StringLiteral(_)  => Ok(Type::Str),
            Expression::BoolLiteral(_)    => Ok(Type::Bool),
            Expression::Identifier(ref n) => match sym.get_name(&*n) {
                Some((i, env_index)) => {
                    Ok(env.get_type(i, env_index).unwrap())
                },
                None => Err(ParserError::new(&format!("can't get type of undeclared: {}", n))),
            },

            Expression::Definition(ref t, _, ref expr) => {
                match *t {
                    Some(ref tp) => return Ok(tp.clone()),
                    None     => (),
                }

                Ok(try!(expr.get_type(sym, env)))
            },

            Expression::Lambda {
                ref name, ref retty, ref param_names, ref param_types, ref body,
            } => {
                let mut tp = vec![retty.clone()];

                for t in param_types.iter() {
                    tp.push(t.clone())
                }

                Ok(Type::Lambda(Rc::new(tp)))
            },

            Expression::Call(ref id, ref args) => {
                match try!(id.get_type(sym, env)) {
                    Type::Lambda(ref params) => {
                        Ok(params.get(0).unwrap().clone())
                    },
                    _ => Err(ParserError::new(&format!("can't call non-lambda"))),
                }
            },

            Expression::Operation {
                ref left, ref op, ref right,
            } => {
                Ok(try!(op.operate((try!(left.get_type(sym, env)), try!(right.get_type(sym, env))))))
            },

            _ => Ok(Type::Undefined),
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

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Str, Num, Bool, Any, Nil, Undefined, Lambda(Rc<Vec<Type>>),
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
}

impl Operand {
    pub fn operate(&self, lr: (Type, Type)) -> ParserResult<Type> {
        match *self {
            Operand::Pow => match lr {
                (Type::Num, Type::Num) => Ok(Type::Num),
                (Type::Any, Type::Num) => Ok(Type::Any),
                (Type::Num, Type::Any) => Ok(Type::Any),
                (Type::Str, Type::Num) => Ok(Type::Str),
                (Type::Str, Type::Any) => Ok(Type::Any),
                (Type::Any, Type::Any) => Ok(Type::Any),
                (a, b) => Err(ParserError::new(&format!("failed to pow: {:?} and {:?}", a, b))),
            },

            Operand::Mul => match lr {
                (Type::Num, Type::Num)  => Ok(Type::Num),
                (Type::Any, Type::Num)  => Ok(Type::Any),
                (Type::Num, Type::Any)  => Ok(Type::Any),
                (Type::Str, Type::Num)  => Ok(Type::Str),
                (Type::Str, Type::Str)  => Ok(Type::Str),
                (Type::Any, Type::Any)  => Ok(Type::Any),
                (a, b) => Err(ParserError::new(&format!("failed to multiply: {:?} and {:?}", a, b))),
            },

            Operand::Div => match lr {
                (Type::Num, Type::Num)  => Ok(Type::Num),
                (Type::Any, Type::Num)  => Ok(Type::Any),
                (Type::Num, Type::Any)  => Ok(Type::Any),
                (Type::Any, Type::Any)  => Ok(Type::Any),
                (a, b) => Err(ParserError::new(&format!("failed to divide: {:?} and {:?}", a, b))),
            },

            Operand::Mod => match lr {
                (Type::Num, Type::Num)  => Ok(Type::Num),
                (Type::Any, Type::Num)  => Ok(Type::Any),
                (Type::Num, Type::Any)  => Ok(Type::Any),
                (Type::Any, Type::Any)  => Ok(Type::Any),
                (a, b) => Err(ParserError::new(&format!("failed to mod: {:?} and {:?}", a, b))),
            },

            Operand::Add => match lr {
                (Type::Num, Type::Num)  => Ok(Type::Num),
                (Type::Any, Type::Num)  => Ok(Type::Any),
                (Type::Num, Type::Any)  => Ok(Type::Any),
                (Type::Str, Type::Num)  => Ok(Type::Str),
                (Type::Str, Type::Str)  => Ok(Type::Str),
                (Type::Str, Type::Bool) => Ok(Type::Str),
                (Type::Any, Type::Any)  => Ok(Type::Any),
                (a, b) => Err(ParserError::new(&format!("failed to add: {:?} and {:?}", a, b))),
            },

            Operand::Sub => match lr {
                (Type::Num, Type::Num)  => Ok(Type::Num),
                (Type::Any, Type::Num)  => Ok(Type::Any),
                (Type::Num, Type::Any)  => Ok(Type::Any),
                (Type::Str, Type::Num)  => Ok(Type::Str),
                (Type::Str, Type::Str)  => Ok(Type::Str),
                (Type::Any, Type::Any)  => Ok(Type::Any),
                (a, b) => Err(ParserError::new(&format!("failed to subtract: {:?} and {:?}", a, b))),
            },

            Operand::Equal | Operand::NEqual => Ok(Type::Bool),

            Operand::Lt | Operand::Gt | Operand::LtEqual | Operand::GtEqual => match lr {
                (a @ Type::Bool, b @ _) => Err(ParserError::new(&format!("failed to '{:?} < {:?}'", a, b))),
                (a @ _, b @ Type::Bool) => Err(ParserError::new(&format!("failed to '{:?} < {:?}'", a, b))),
                (a @ Type::Str, b @ _)  => Err(ParserError::new(&format!("failed to '{:?} < {:?}'", a, b))),
                (a @ _, b @ Type::Str)  => Err(ParserError::new(&format!("failed to '{:?} < {:?}'", a, b))),
                _ => Ok(Type::Bool),
            },

            Operand::And | Operand::Or | Operand::Not => Ok(Type::Bool),
        }
    }
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
        _ => None,
    }
}
