use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

use regex::Regex;

use anyhow::Result;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Start printing when you see this regex, and print that line too.
    #[arg(short = 'f', long)]
    from_incl: Option<Regex>,

    /// Start printing when you see this regex, but don't print that line.
    #[arg(short = 'F', long, conflicts_with = "from_incl")]
    from_excl: Option<Regex>,

    /// Stop printing when you see this regex, but print this line.
    #[arg(short = 't', long)]
    to_incl: Option<Regex>,

    /// Stop printing when you see this regex, but don't print this line
    #[arg(short = 't', long)]
    #[arg(short = 'T', long, conflicts_with = "to_incl")]
    to_excl: Option<Regex>,

    /// Path of the file to read. "-" for stdin. Omit for stdin.
    input_file: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let from_incl = args.from_incl.is_some();
    let from_regex = args.from_incl.or(args.from_excl);

    let to_incl = args.to_incl.is_some();
    let to_regex = args.to_incl.or(args.to_excl);

    let mut stdout = std::io::stdout();

    let input: Box<dyn BufRead> =
        if args.input_file.is_none() || args.input_file.as_ref().is_some_and(|s| s == "-") {
            let stdin = std::io::stdin();
            Box::new(stdin.lock())
        } else if let Some(filename) = args.input_file {
            Box::new(BufReader::new(File::open(filename)?))
        } else {
            unreachable!()
        };

    let mut printing = from_regex.is_none();

    for line in input.lines() {
        let line = line?;
        if !printing {
            if from_regex.as_ref().is_some_and(|r| r.is_match(&line)) {
                printing = true;
                if from_incl {
                    writeln!(&mut stdout, "{line}")?;
                }
            }
        } else {
            // printing = true
            if to_regex.as_ref().is_some_and(|r| r.is_match(&line)) {
                printing = false;
                if to_incl {
                    writeln!(&mut stdout, "{line}")?;
                }
            } else {
                writeln!(&mut stdout, "{line}")?;
            }
        }
    }

    Ok(())
}
