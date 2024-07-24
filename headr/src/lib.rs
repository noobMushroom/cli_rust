use std::{fs::File, io::{self, BufRead, BufReader}};

use clap::Parser;

pub type HeadResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Parser, Debug)]
#[command(version, about,author,  long_about = None)]
pub struct Args {
    /// Path of the files
    #[arg(value_name("FILES"), default_value="-")]
    pub file: Vec<String>,

    /// Prints the number of lines
    #[arg(
        short('n'),
        long("lines"),
        value_name("LINES"),
        default_value = "10",
        value_parser = clap::value_parser!(u64).range(1..)
    )]
    pub lines: u64,

    /// Prints the number of bytes
    #[arg(
        short('c'), 
        long("bytes"),
        conflicts_with("lines"),
        value_name("BYTES"),
        value_parser = clap::value_parser!(u64).range(1..)
            )]
    pub bytes: Option<u64>,
}

pub fn run(args: Args) -> HeadResult<()> {

    for (num , file_name) in args.file.iter().enumerate(){
        match open(&file_name) {
            Err(e) => eprintln!("{}: {}", file_name, e),
            Ok(mut file) => {
                if args.file.len() > 1 {
                    println!("{}==> {file_name} <==",
                        if num > 0 {"\n"} else {""}
                        )
                }
                if let Some(num) = args.bytes{
                    let mut buffer = vec![0; num as usize];
                    let bytes = file.read(&mut buffer)?;
                    print!("{}", String::from_utf8_lossy(&buffer[..bytes]));

                }else {
                    let mut temp = String::new();
                    for _ in 0..args.lines {
                        let bytes = file.read_line(&mut temp)?;
                        if bytes == 0{
                            break;
                        }
                        print!("{}", temp);
                        temp.clear();
                    }
                }
            }
        }
    }

    Ok(())
}


fn open(filename: &str) -> HeadResult <Box<dyn BufRead>>{
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _=> Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
}
