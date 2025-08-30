use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use clap::Parser;

#[derive(Debug, Parser)]
#[command(
    author = "Bruno-Camargo-Ferreira<bruno.ferreira.v3ktor@gmail.com>",
    version = "0.1.0",
    about = "Rust version of 'uniq'"
)]
pub struct Args {
    ///Input file
    #[arg(value_name = "IN_FILE", default_value = "-")]
    in_file: String,

    // Output file
    #[arg(value_name = "OUT_FILE")]
    out_file: Option<String>,

    // Show counts
    #[arg(short, long)]
    count: bool,
}

pub fn open(filename: &str) -> anyhow::Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run(args: Args) -> anyhow::Result<()> {
    println!("{args:?}");
    Ok(())
}
