use clap::Parser;
use headr::run;

use headr::Args;

fn main() {
    let args = Args::parse();
    if let Err(e) = run(args) {
        eprintln!("{}", e);
        std::process::exit(1);
    };
}
