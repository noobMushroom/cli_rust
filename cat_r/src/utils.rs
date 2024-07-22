pub type CatResult<T> = Result<T, Box<dyn std::error::Error>>;
use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    /// path of the file to output
    pub files: Vec<PathBuf>,

    /// Print line numbers
    #[arg(long("number"), short('n'), conflicts_with("number_blank_line"))]
    pub line_number: bool,

    /// Print line number in non blank lines
    #[arg(long("number-nonblank"), short('b'))]
    pub number_blank_line: bool,
}
