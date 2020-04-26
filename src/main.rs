use libcorrekt::Checker;
use std::{fs, io};
use structopt::StructOpt;

/// Spellcheck with configurable auto-substitution.
#[derive(Clone, Debug, StructOpt)]
struct Opt {
    path: String,
}

fn main() -> io::Result<()> {
    let opt = Opt::from_args();
    let mut checker = Checker::new();

    for error in checker.check(&fs::read_to_string(&opt.path)?) {
        println!("{}", error.text());
    }

    Ok(())
}
