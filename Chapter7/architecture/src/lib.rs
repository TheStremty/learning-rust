pub mod db;
pub mod utils;

//re-eksport --> (skrócone importowanie)
pub use crate::utils::parser::parse_input as fast_parse;