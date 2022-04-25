use anyhow::{Context, Error};
use clap::Parser;
use std::fs;
use std::io::{self, Write};

#[derive(Debug, Parser)]
struct Cli {
    pattern: String,
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<(), Error> {
    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout.lock());
    let args = Cli::parse();
    let content = fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file {:?}", &args.path))?;

    for line in content.lines() {
        if line.contains(&args.pattern) {
            writeln!(handle, "foo: {}", 42)?;
        }
    }
    Ok(())
}
