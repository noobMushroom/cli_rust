use clap::{command, Arg, ArgAction};

fn main() {
    let matches = command!()
        .arg(Arg::new("text"))
        .arg(
            Arg::new("omit-newline")
                .short('n')
                .long("omit-newline")
                .required(false)
                .action(ArgAction::SetFalse),
        )
        .get_matches();

    if let Some(text) = matches.get_one::<String>("text") {
        if let Some(value) = matches.get_one::<bool>("omit-newline") {
            if *value {
                println!("{}", text);
            } else {
                print!("{}", text);
            }
        }
    }
}
