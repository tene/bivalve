use crate::{
    exec::{Execute, Redirect, Word},
    rt::Runtime,
};
use std::collections::HashMap;

pub struct Simple {
    functions: HashMap<String, Box<dyn Execute<Simple>>>,
}

impl Simple {
    pub fn new() -> Self {
        Simple {
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
        let words: Vec<String> = words.into_iter().map(|w| w.resolve(self)).collect();
        println!("{:?}", &words);
    }
}
