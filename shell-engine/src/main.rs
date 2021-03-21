use std::path::PathBuf;
use structopt::StructOpt;

use shell_engine::{
    exec::Execute,
    rt::{simple::Simple, Runtime},
};

#[derive(StructOpt, Debug)]
#[structopt(name = "bivalve")]
struct Opt {
    file: PathBuf,
}

fn main() {
    let opt = Opt::from_args();
    let mut rt = Simple::new();
    if let Ok(mut parser) = shell_engine::parse::parse_file(opt.file) {
        while let Ok(Some(cmd)) = parser.complete_command() {
            cmd.exec(&mut rt);
        }
    }
}
