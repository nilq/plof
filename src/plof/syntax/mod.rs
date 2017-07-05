pub mod lexer;
pub mod parser;
pub mod symtab;
pub mod env;
pub mod error;

pub type RunResult<T> = Result<T, RunError>;

pub use self::parser::*;
pub use self::lexer::*;
pub use self::symtab::*;
pub use self::env::*;
pub use self::error::*;
