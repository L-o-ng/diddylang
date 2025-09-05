use std::io::Write;
use clap::Parser;
use anyhow::{Context, Result};

fn main() -> Result<()> {
    let args = Cli::parse();
    let content = std::fs::read_to_string(args.target.clone())
        .with_context(|| format!("Could not read file '{}'", args.target.display()))?;
    let new_file = std::fs::File::create(args.new.clone())
        .with_context(|| format!("Could not write file '{}'", args.new.display()))?;
    for line in content.lines() {
        for byte in line.bytes() {
           for _ in 0..byte {
                write!(&new_file, "diddy ").with_context(|| format!("Could not write file '{}'", args.new.display()))?;
            }
            writeln!(&new_file).with_context(|| format!("Could not write file '{}'", args.new.display()))?;
        }
        writeln!(&new_file, "\n").with_context(|| format!("Could not write file '{}'", args.new.display()))?;
    }
    println!("diddy party at {}", args.new.display());
    Ok(())
}

#[derive(Parser)]
struct Cli {
    target: std::path::PathBuf,
    new: std::path::PathBuf,
}