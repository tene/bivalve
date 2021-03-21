use crate::{
    exec::{Execute, Redirect, Word},
    rt::Runtime,
};
use std::collections::HashMap;

pub mod builtins;
use builtins::Builtin;

pub struct Simple {
    builtins: HashMap<&'static str, Builtin>,
    functions: HashMap<String, Box<dyn Execute<Simple>>>,
}

impl Simple {
    pub fn new() -> Self {
        Simple {
            builtins: builtins::builtins(),
            functions: HashMap::new(),
        }
    }
}

impl Default for Simple {
    fn default() -> Self {
        Self::new()
    }
}

impl Runtime for Simple {
    fn bg<E: Execute<Self>>(&mut self, exec: &E) {
        exec.exec(self);
    }

    fn define<E: 'static + Execute<Self>>(&mut self, name: &str, body: E) {
        self.functions.insert(name.to_owned(), Box::new(body));
    }

    fn run_command<W: Word<Self>, R: Redirect<Self>>(
        &mut self,
        _environment: Vec<(&String, &W)>,
        words: Vec<&W>,
        _redirects: Vec<&R>,
    ) {
        let mut words_iter = words.into_iter().map(|w| w.resolve(self));
        let name = words_iter.next().expect("Need at least 1 word for command");
        let words: Vec<String> = words_iter.collect();
        println!("{} {:?}", &name, &words);

        if let Some(&cmd) = self.builtins.get(name.as_str()) {
            cmd(self, name, words);
        }
    }
}
