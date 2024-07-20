use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    /// Input
    #[arg(required(true))]
    text: Vec<String>,

    ///Flag to print omit newline
    #[arg(short('n'))]
    omit_newline: bool,
}

fn main() {
    let args = Args::parse();

    print!(
        "{}{}",
        args.text.join(" "),
        if args.omit_newline { "" } else { "\n" }
    )
}
