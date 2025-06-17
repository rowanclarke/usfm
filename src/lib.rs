pub mod parser;
pub mod usfm;

pub use usfm::*;

use parser::UsfmParser;
use parser::{Rule, to_book};
use pest::Parser;

pub fn parse(input: &str) -> Book {
    to_book(UsfmParser::parse(Rule::book, input).unwrap())
}
