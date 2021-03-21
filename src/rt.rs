use crate::exec::{Execute, Redirect, Word};

pub mod simple;
pub trait Runtime: Sized {
    fn bg<E: Execute<Self>>(&mut self, exec: &E);
    fn define<E: 'static + Execute<Self>>(&mut self, name: &str, body: E);
    fn run_command<W: Word<Self>, R: Redirect<Self>>(
        &mut self,
        environment: Vec<(&String, &W)>,
        words: Vec<&W>,
        redirects: Vec<&R>,
    );
}
