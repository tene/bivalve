use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "bivalve")]
struct Opt {
    file: PathBuf,
}

fn main() {
    let opt = Opt::from_args();
    let _ = bivalve::parse::parse_file(opt.file);
}
