use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

use regex::Regex;

use anyhow::Result;

#[cfg(test)]
mod tests;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Start printing when you see this regex, and print that line too.
    #[arg(short = 'f', long, value_name = "REGEX")]
    from_incl: Option<Regex>,

    /// Start printing when you see this regex, but don't print that line.
    #[arg(short = 'F', long, conflicts_with = "from_incl", value_name = "REGEX")]
    from_excl: Option<Regex>,

    /// Stop printing when you see this regex, but print this line.
    #[arg(short = 't', long, value_name = "REGEX")]
    to_incl: Option<Regex>,

    /// Stop printing when you see this regex, but don't print this line
    #[arg(short = 'T', long, conflicts_with = "to_incl", value_name = "REGEX")]
    to_excl: Option<Regex>,

    /// Path of the file to read. "-" for stdin. Omit for stdin.
    input_file: Option<String>,

    /// Stop reading after finding NUM matches.
    #[arg(short, long, value_name = "NUM")]
    max_count: Option<u64>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let input: Box<dyn BufRead> =
        if args.input_file.is_none() || args.input_file.as_ref().is_some_and(|s| s == "-") {
            let stdin = std::io::stdin();
            Box::new(stdin.lock())
        } else if let Some(ref filename) = args.input_file {
            Box::new(BufReader::new(File::open(filename)?))
        } else {
            unreachable!()
        };

    let stdout = std::io::stdout().lock();

    run(args, input, stdout)
}

fn run(args: Args, input: impl BufRead, mut output: impl Write) -> Result<()> {
    let from_incl = args.from_incl.is_some();
    let from_regex = args.from_incl.or(args.from_excl);

    let to_incl = args.to_incl.is_some();
    let to_regex = args.to_incl.or(args.to_excl);

    let mut printing = from_regex.is_none();
    let mut num_matches = 0;

    for line in input.lines() {
        if args.max_count.is_some_and(|m| num_matches >= m) {
            // We've hit the max
            break;
        }

        let line = line?;
        if !printing {
            if from_regex.as_ref().is_some_and(|r| r.is_match(&line)) {
                printing = true;
                if from_incl {
                    writeln!(&mut output, "{line}")?;
                }
            }
        } else {
            // printing = true
            if to_regex.as_ref().is_some_and(|r| r.is_match(&line)) {
                printing = false;
                num_matches += 1;
                if to_incl {
                    writeln!(&mut output, "{line}")?;
                }
            } else {
                writeln!(&mut output, "{line}")?;
            }
        }
    }

    Ok(())
}
