use clap::Parser;
use headr::{run, Args};

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprint!("{e}");
        std::process::exit(1);
    }
}
