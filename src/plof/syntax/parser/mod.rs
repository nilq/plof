pub mod error;

pub type ParserResult<T> = Result<T, ParserError>;

pub use super::lexer;
