use conch_parser::ast;

use crate::exec::{resolve_concat_words, Execute, Redirect, Runtime, Word};

impl<R: Runtime, W: Word<R>> Redirect<R> for ast::Redirect<W> {
    fn make_something_up(&self, rt: &mut R) {
        todo!("Redirects");
    }
}

impl<R: Runtime> Word<R> for ast::AtomicTopLevelWord<String> {
    fn resolve(&self, rt: &R) -> String {
        match &self.0 {
            ast::ComplexWord::Concat(words) => resolve_concat_words(rt, words),
            ast::ComplexWord::Single(word) => word.resolve(rt),
        }
    }
}

impl<R: Runtime, P, S> Word<R> for ast::SimpleWord<String, P, S> {
    fn resolve(&self, rt: &R) -> String {
        match self {
            ast::SimpleWord::Literal(lit) => lit.clone(),
            ast::SimpleWord::Escaped(esc) => esc.clone(),
            ast::SimpleWord::Param(_) => {
                todo!("Param")
            }
            ast::SimpleWord::Subst(_) => {
                todo!("Subst")
            }
            ast::SimpleWord::Star => {
                todo!("Star")
            }
            ast::SimpleWord::Question => {
                todo!("Question")
            }
            ast::SimpleWord::SquareOpen => {
                todo!("SquareOpen")
            }
            ast::SimpleWord::SquareClose => {
                todo!("SquareClose")
            }
            ast::SimpleWord::Tilde => {
                todo!("Tilde")
            }
            ast::SimpleWord::Colon => {
                todo!("Colon")
            }
        }
    }
}

impl<R: Runtime, W: Word<R>> Word<R> for ast::Word<String, W> {
    fn resolve(&self, rt: &R) -> String {
        match self {
            ast::Word::Simple(w) => w.resolve(rt),
            ast::Word::DoubleQuoted(words) => resolve_concat_words(rt, words),
            ast::Word::SingleQuoted(sq) => sq.clone(),
        }
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

impl<R: Runtime, S: Execute<R>, C: Execute<R>, F: 'static + Execute<R> + Clone> Execute<R>
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

impl<RT: Runtime, W: Word<RT>, RD: Redirect<RT>> Execute<RT> for ast::SimpleCommand<String, W, RD> {
    fn exec(&self, rt: &mut RT) {
        let mut redirects = vec![];
        let mut environment = vec![];
        let mut words = vec![];
        for i in &self.redirects_or_env_vars {
            match i {
                ast::RedirectOrEnvVar::Redirect(r) => redirects.push(r),
                ast::RedirectOrEnvVar::EnvVar(name, Some(value)) => environment.push((name, value)),
                _ => {}
            }
        }
        for i in &self.redirects_or_cmd_words {
            match i {
                ast::RedirectOrCmdWord::CmdWord(w) => words.push(w),
                ast::RedirectOrCmdWord::Redirect(r) => redirects.push(r),
            }
        }
        rt.run_command(environment, words, redirects);
    }
}

impl<RT: Runtime, Word, Redirect> Execute<RT>
    for ast::ShellCompoundCommand<String, Word, Redirect>
{
    fn exec(&self, rt: &mut RT) {
        /*
            let redirects = &self.io;
            match &self.kind {
                ast::CompoundCommandKind::Brace(_) => {}
                ast::CompoundCommandKind::Subshell(_) => {}
                ast::CompoundCommandKind::While(_) => {}
                ast::CompoundCommandKind::Until(_) => {}
                ast::CompoundCommandKind::If {
                    conditionals,
                    else_branch,
                } => {}
                ast::CompoundCommandKind::For { var, words, body } => {}
                ast::CompoundCommandKind::Case { word, arms } => {}
            }
        */
    }
}
