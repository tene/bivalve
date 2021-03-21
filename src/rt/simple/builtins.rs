use std::collections::HashMap;

use super::Simple;

pub type Builtin = fn(rt: &mut Simple, name: String, args: Vec<String>);

pub fn echo(_rt: &mut Simple, _name: String, args: Vec<String>) {
    let out = args.join(" ");
    println!("{}", out);
}

pub const BUILTINS: [(&'static str, Builtin); 1] = [("echo", echo)];

pub fn builtins() -> HashMap<&'static str, Builtin> {
    std::iter::FromIterator::from_iter(BUILTINS.iter().cloned())
}
