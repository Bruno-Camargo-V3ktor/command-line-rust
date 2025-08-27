use clap::Parser;

#[derive(Debug, Parser)]
#[command(
    author = "Bruno-Camargo-Ferreira <brunp.ferreira.v3ktor@gmail.com>",
    version,
    about = "Rust version of 'wc'"
)]
pub struct Args {
    ///Input file(s)
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,

    ///Show line count
    #[arg(short, long)]
    lines: bool,

    ///Show word count
    #[arg(short, long)]
    words: bool,

    ///Show byte count
    #[arg(short('c'), long)]
    bytes: bool,

    ///Show character count
    #[arg(short('m'), long, conflicts_with("bytes"))]
    chars: bool,
}

pub fn get_args() -> anyhow::Result<Args> {
    Ok(Args::parse())
}
