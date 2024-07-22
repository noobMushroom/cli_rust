use catr::run;
use catr::utils::Args;
use clap::Parser;

fn main() {
    let args = Args::parse();
    if let Err(e) = run(args) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
