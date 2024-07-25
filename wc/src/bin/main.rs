use clap::Parser;
use wcr::run;
use wcr::Args;

fn main() {
    if let Err(e) = run(&mut Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
