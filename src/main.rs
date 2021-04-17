use std::io::{self, Read};

use rand::prelude::SliceRandom;
use structopt::StructOpt;

#[derive(StructOpt, Clone, Debug)]
struct Opts {
    count: Option<usize>,
    #[structopt(short, long)]
    shuffle: bool,
}

fn main() -> io::Result<()> {
    let opts = Opts::from_args();
    let content = read_stdin()?;
    let mut choices: Vec<_> = content.lines().collect();

    if opts.shuffle {
        choices.shuffle(&mut rand::thread_rng());
        for item in choices {
            println!("{}", item);
        }
    } else {
        for &item in choices.choose_multiple(&mut rand::thread_rng(), opts.count.unwrap_or(1)) {
            println!("{}", item);
        }
    }

    Ok(())
}

fn read_stdin() -> io::Result<String> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;
    Ok(buf)
}
