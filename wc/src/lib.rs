use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use colored::*;

pub type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, PartialEq)]
pub struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
}

#[derive(Parser, Debug)]
#[command(author = "gawrmiko@mail.com>", version = "0.1.0", about = "Rust wc")]
pub struct Config {
    #[arg(required = true, value_name = "FILE")]
    files: Vec<String>,

    #[arg(short = 'l', long, help = "Show line count")]
    lines: bool,

    #[arg(short = 'w', long, help = "Show word count")]
    words: bool,

    #[arg(short = 'c', long, help = "Show byte count")]
    bytes: bool,

    #[arg(short = 'm', long, help = "Show character count")]
    chars: bool,
}

// --------------------------------------------------
pub fn run(config: Config) -> MyResult<()> {
    let mut total_lines = 0;
    let mut total_words = 0;
    let mut total_bytes = 0;
    let mut total_chars = 0;
    let total_counts_needed = config.files.len() > 1;

    // Ustawienie domyślnych wartości na `true`, jeśli żadna opcja nie jest wybrana
    let show_lines = config.lines || (!config.lines && !config.words && !config.bytes && !config.chars);
    let show_words = config.words || (!config.lines && !config.words && !config.bytes && !config.chars);
    let show_bytes = config.bytes || (!config.lines && !config.words && !config.bytes && !config.chars);
    let show_chars = config.chars || (!config.lines && !config.words && !config.bytes && !config.chars);

    for filename in &config.files {
        match open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(file) => {
                if let Ok(info) = count(file) {
                    println!(
                        "{}{}{}{}{}",
                        format_field(info.num_lines, show_lines),
                        format_field(info.num_words, show_words),
                        format_field(info.num_bytes, show_bytes),
                        format_field(info.num_chars, show_chars),
                        format!(" {}", &filename.green())
                    );

                    total_lines += info.num_lines;
                    total_words += info.num_words;
                    total_bytes += info.num_bytes;
                    total_chars += info.num_chars;
                }
            }
        }
    }

    if total_counts_needed {
        println!(
            "{}{}{}{} {}",
            format_field(total_lines, show_lines).italic().bold(),
            format_field(total_words, show_words).italic().bold(),
            format_field(total_bytes, show_bytes).italic().bold(),
            format_field(total_chars, show_chars).italic().bold(),
            "total".italic().red().bold()
        );
    }

    Ok(())
}

// --------------------------------------------------
fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

// --------------------------------------------------
fn format_field(value: usize, show: bool) -> String {
    if show {
        format!("{:>10}", value.to_string().blue())
    } else {
        "".to_string()
    }
}

// --------------------------------------------------
fn count(mut file: impl BufRead) -> MyResult<FileInfo> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;
    let mut line = String::new();

    while file.read_line(&mut line)? != 0 {
        num_bytes += line.as_bytes().len();
        num_lines += 1;
        num_words += line.split_whitespace().count();
        num_chars += line.chars().count();
        line.clear();
    }

    Ok(FileInfo {
        num_lines,
        num_words,
        num_bytes,
        num_chars,
    })
}

// --------------------------------------------------
#[cfg(test)]
mod tests {
    use super::{count, format_field, FileInfo};
    use std::io::Cursor;

    #[test]
    fn test_count() {
        let text = "Kiedy pada deszcz w Łodzi.\r\n";
        let info = count(Cursor::new(text));
        assert!(info.is_ok());
        let expected = FileInfo {
            num_lines: 1,
            num_words: 5,
            num_chars: 28,
            num_bytes: 29,
        };
        assert_eq!(info.unwrap(), expected);
    }

    #[test]
    fn test_format_field() {
        assert_eq!(format_field(1, false), "");
        assert_eq!(format_field(3, true), "\u{1b}[34m         3\u{1b}[0m");
        assert_eq!(format_field(10, true), "\u{1b}[34m        10\u{1b}[0m");
    }
}
