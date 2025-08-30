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
