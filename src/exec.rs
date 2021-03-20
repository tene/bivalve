use conch_parser::ast;

pub trait Runtime: Sized {
    fn bg<E: Execute<Self>>(&mut self, exec: &E);
    fn define<E: Execute<Self>>(&mut self, name: &str, body: E);
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

impl<R: Runtime> Execute<R> for ast::AtomicTopLevelCommand<String> {
    fn exec(&self, rt: &mut R) {
        match self.0 {
            ast::Command::Job(ref j) => rt.bg(j),
            ast::Command::List(ref l) => Execute::exec(l, rt),
        }
    }
}

impl<R: Runtime, E: Execute<R>> Execute<R> for ast::AndOrList<E> {
    fn exec(&self, rt: &mut R) {
        if self.rest.len() == 0 {
            return self.first.exec(rt);
        }
        todo!("AndOrList")
    }
}

impl<R: Runtime, E: Execute<R>> Execute<R> for ast::ListableCommand<E> {
    fn exec(&self, rt: &mut R) {
        match self {
            ast::ListableCommand::Pipe(_, _) => {
                todo!("Pipe")
            }
            ast::ListableCommand::Single(ref s) => s.exec(rt),
        }
    }
}

impl<R: Runtime, S: Execute<R>, C: Execute<R>, F: Execute<R> + Clone> Execute<R>
    for ast::PipeableCommand<String, S, C, F>
{
    fn exec(&self, rt: &mut R) {
        match self {
            ast::PipeableCommand::Simple(s) => s.exec(rt),
            ast::PipeableCommand::Compound(c) => c.exec(rt),
            ast::PipeableCommand::FunctionDef(name, body) => rt.define(name, body.clone()),
        }
    }
}

impl<RT: Runtime, Word, Redirect> Execute<RT> for ast::SimpleCommand<String, Word, Redirect> {
    fn exec(&self, rt: &mut RT) {
        todo!("Simple Command")
    }
}

impl<RT: Runtime, Word, Redirect> Execute<RT>
    for ast::ShellCompoundCommand<String, Word, Redirect>
{
    fn exec(&self, rt: &mut RT) {
        todo!("Compound Command")
    }
}
