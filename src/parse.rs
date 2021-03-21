use anyhow::Result;
use std::path::PathBuf;

use conch_parser::{ast::builder::AtomicDefaultBuilder, lexer::Lexer, parse::Parser};

pub fn parse_file(
    filename: PathBuf,
) -> Result<Parser<Lexer<std::vec::IntoIter<char>>, AtomicDefaultBuilder<String>>> {
    let source = std::fs::read(filename)?;
    let source = std::str::from_utf8(&source)?;
    let chars = source.chars().collect::<Vec<_>>().into_iter();
    let lexer = Lexer::new(chars);
    let parser = Parser::with_builder(lexer, AtomicDefaultBuilder::<String>::new());
    return Ok(parser);
}
