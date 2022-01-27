use std::io::{self, Read};

use clap::Parser;
use rand::prelude::SliceRandom;
use squirrel_rng::SquirrelRng;

#[derive(Clone, Debug, Parser)]
#[clap(author, version)]
struct Args {
    count: Option<usize>,
    #[clap(short, long)]
    shuffle: bool,
}

fn main() {
    if let Err(e) = run(&Args::parse()) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn run(args: &Args) -> io::Result<()> {
    let content = read_stdin()?;

    let mut choices: Vec<_> = content.lines().collect();
    let mut rng = SquirrelRng::new();

    if args.shuffle {
        choices.shuffle(&mut rng);
        for item in choices {
            println!("{}", item);
        }
    } else {
        for &item in choices.choose_multiple(&mut rng, args.count.unwrap_or(1)) {
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
