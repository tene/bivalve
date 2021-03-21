mod conch;

use crate::rt::Runtime;

pub trait Word<R: Runtime> {
    // TODO -> Vec<String>
    fn resolve(&self, rt: &R) -> String;
}

pub fn resolve_concat_words<R: Runtime, W: Word<R>>(rt: &R, words: &Vec<W>) -> String {
    itertools::join(words.iter().map(|w| w.resolve(rt)), "")
}

pub trait Redirect<R: Runtime> {
    fn make_something_up(&self, rt: &mut R);
}

pub trait Execute<R: Runtime> {
    fn exec(&self, rt: &mut R);
}

impl<R: Runtime, E: Execute<R>> Execute<R> for Box<E> {
    fn exec(&self, rt: &mut R) {
        (self.as_ref()).exec(rt)
    }
}

impl<R: Runtime, E: Execute<R>> Execute<R> for std::sync::Arc<E> {
    fn exec(&self, rt: &mut R) {
        (self.as_ref()).exec(rt)
    }
}
