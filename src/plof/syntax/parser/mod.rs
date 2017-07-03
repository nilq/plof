pub mod error;
pub mod ast;

pub use self::error::*;
pub use self::ast::*;

pub type ParserResult<T> = Result<T, ParserError>;

pub use super::lexer;
