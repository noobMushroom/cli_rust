use std::{fs::File, io::BufRead, io::BufReader, path::PathBuf};
use utils::{Args, CatResult};

pub mod utils;

pub fn run(args: Args) -> CatResult<()> {
    for file in args.files {
        match open_file(&file) {
            Ok(file) => {
                let mut non_blank = 1;
                for (counter, line) in file.lines().enumerate() {
                    let line = line?;
                    if args.line_number {
                        println!("{:>6}\t{}", counter + 1, line)
                    } else if args.number_blank_line {
                        if line.is_empty() {
                            println!()
                        } else {
                            println!("{:>6}\t{}", non_blank, line);
                            non_blank += 1;
                        }
                    } else {
                        println!("{}", line);
                    }
                }
            }
            Err(e) => eprintln!("{:?}: {}", file, e),
        }
    }
    Ok(())
}

fn open_file(filename: &PathBuf) -> CatResult<Box<dyn BufRead>> {
    Ok(Box::new(BufReader::new(File::open(filename)?)))
}
