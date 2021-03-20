use anyhow::Result;
use std::path::PathBuf;

use conch_parser::{ast::builder::AtomicDefaultBuilder, lexer::Lexer, parse::Parser};

pub fn parse_file(filename: PathBuf) -> Result<()> {
    let source = std::fs::read(filename)?;
    let source = std::str::from_utf8(&source)?;
    let lexer = Lexer::new(source.chars());
    let mut parser = Parser::with_builder(lexer, AtomicDefaultBuilder::<String>::new());
    //let mut parser = DefaultParser::new(lexer);
    while let Ok(Some(out)) = parser.complete_command() {
        println!("{:#?}", out);
    }
    Ok(())
}
