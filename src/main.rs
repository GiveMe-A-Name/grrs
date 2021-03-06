use clap::Parser;
use grrs::find_matches;
use std::fs;
use std::io;

#[derive(Debug, Parser)]
struct Cli {
    pattern: String,
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout.lock());
    let args = Cli::parse();
    let content = fs::read_to_string(&args.path)
        .map_err(|_| format!("could not read file {:?}", &args.path))?;

    find_matches(&content, &args.pattern, &mut handle);
    Ok(())
}
