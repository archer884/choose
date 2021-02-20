use std::io::{self, stdin, Read};

use clap::Clap;
use rand::prelude::SliceRandom;

#[derive(Clap, Clone, Debug)]
struct Opts {
    count: Option<usize>,
}

fn main() -> io::Result<()> {
    let opts = Opts::parse();
    let content = read_stdin()?;
    let choices: Vec<_> = content.lines().collect();

    for &item in choices.choose_multiple(&mut rand::thread_rng(), opts.count.unwrap_or(1)) {
        println!("{}", item);
    }

    Ok(())
}

fn read_stdin() -> io::Result<String> {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf)?;
    Ok(buf)
}
