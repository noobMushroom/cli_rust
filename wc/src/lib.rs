use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use anyhow::Result;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    /// Names of the files
    #[arg(default_value = "-", value_name("FILES"))]
    pub files: Vec<String>,

    /// Line count
    #[arg(long, short('l'), value_name("LINES"))]
    pub lines: bool,

    /// Word count
    #[arg(long, short('w'), value_name("WORDS"))]
    pub words: bool,

    /// Bytes count
    #[arg(long, short('c'), value_name("BYTES"))]
    pub bytes: bool,

    /// char count
    #[arg(long, short('m'), value_name("CHARS"), conflicts_with("bytes"))]
    pub chars: bool,
}

#[derive(PartialEq, Eq, Debug)]
struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_chars: usize,
    num_bytes: usize,
}

impl FileInfo {
    fn new() -> Self {
        Self {
            num_lines: 0,
            num_words: 0,
            num_chars: 0,
            num_bytes: 0,
        }
    }
    fn count(&mut self, mut file: impl BufRead) -> Result<()> {
        let mut buffer = String::new();

        loop {
            let bytes = file.read_line(&mut buffer)?;
            if bytes == 0 {
                break;
            }

            self.num_lines += 1;
            self.num_chars += buffer.chars().count();
            self.num_bytes += bytes;
            self.num_words += buffer.split_whitespace().count();
            buffer.clear();
        }

        Ok(())
    }
}

pub fn run(args: &mut Args) -> Result<()> {
    if [args.lines, args.chars, args.bytes, args.words]
        .iter()
        .all(|v| v == &false)
    {
        args.lines = true;
        args.words = true;
        args.bytes = true;
    }
    let mut total_file_info = FileInfo::new();
    for file_name in &args.files {
        match open(&file_name) {
            Err(e) => eprintln!("{}: {}", file_name, e),
            Ok(file) => {
                let mut file_info = FileInfo::new();
                if let Ok(_) = file_info.count(file) {
                    println!(
                        "{}{}{}{}{}",
                        format_field(file_info.num_lines, args.lines),
                        format_field(file_info.num_words, args.words),
                        format_field(file_info.num_bytes, args.bytes),
                        format_field(file_info.num_chars, args.chars),
                        if file_name == "-" {
                            "".to_string()
                        } else {
                            format!(" {file_name}")
                        },
                    );

                    total_file_info.num_lines += file_info.num_lines;
                    total_file_info.num_words += file_info.num_words;
                    total_file_info.num_bytes += file_info.num_bytes;
                    total_file_info.num_chars += file_info.num_chars;
                }
            }
        }
    }

    if args.files.len() > 1 {
        println!(
            "{}{}{}{} total",
            format_field(total_file_info.num_lines, args.lines),
            format_field(total_file_info.num_words, args.words),
            format_field(total_file_info.num_bytes, args.bytes),
            format_field(total_file_info.num_chars, args.chars)
        );
    }

    Ok(())
}

fn format_field(value: usize, show: bool) -> String {
    if show {
        format!("{value:>8}")
    } else {
        "".to_string()
    }
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

#[cfg(test)]
mod tests {
    use super::FileInfo;
    use std::io::Cursor;
    #[test]
    fn test_count() {
        let mut file_info = FileInfo::new();
        let text = "I don't want the world. I just want your half.\r\n";
        file_info.count(Cursor::new(text)).unwrap();
        let expected = FileInfo {
            num_lines: 1,
            num_words: 10,
            num_chars: 48,
            num_bytes: 48,
        };
        assert_eq!(file_info, expected);
    }
}
