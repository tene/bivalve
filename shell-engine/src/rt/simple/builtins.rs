use std::collections::HashMap;

use super::Simple;

pub type Builtin = fn(rt: &mut Simple, name: String, args: Vec<String>);

pub fn echo(_rt: &mut Simple, _name: String, args: Vec<String>) {
    let out = args.join(" ");
    println!("{}", out);
}

pub fn list_builtins(rt: &mut Simple, _name: String, _args: Vec<String>) {
    let names: Vec<String> = rt.builtins.keys().map(|s| (*s).to_owned()).collect();
    let out = names.join(" ");
    println!("{}", out);
}

pub const BUILTINS: [(&'static str, Builtin); 2] = [("echo", echo), ("builtins", list_builtins)];

pub fn builtins() -> HashMap<&'static str, Builtin> {
    std::iter::FromIterator::from_iter(BUILTINS.iter().cloned())
}
