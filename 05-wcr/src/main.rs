use wcr::{get_args, run};

fn main() {
    if let Err(e) = run(get_args().unwrap()) {
        std::process::exit(1)
    }
}
