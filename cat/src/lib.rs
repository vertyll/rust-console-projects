use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use clap::{Parser};

pub type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[command(author="gawrmiko@gmail.com", version = "0.1.0", about = "Rust cat")]
pub struct Args {
    /// files to operate on
    #[arg(required = true, value_name = "FILE")]
    files: Vec<String>,

    /// number all lines
    #[arg(short = 'n', long="number")]
    number_lines: bool,

    /// number only non-blank lines
    #[arg(short = 'b', long="number-nonblank")]
    number_nonblank_lines: bool,

    /// mark ends with $
    #[arg(short = 'e', long="show-ends")]
    show_ends: bool,
}

pub fn run(config: Args) -> MyResult<()> {
    for filename in &config.files {
        match open(filename) {
            Err(e) => eprintln!("{}: {}", filename, e),
            Ok(file) => process_file(file, &config)?,
        }
    }

    Ok(())
}

fn process_file<T: BufRead>(file: T, config: &Args) -> MyResult<()> {
    let mut last_num = 0;
    for (line_num, line_result) in file.lines().enumerate() {
        let mut line = line_result?;

        if config.show_ends {
            line.push('$');
        }

        if config.number_lines {
            print_numbered_line(line_num + 1, &line);
        } else if config.number_nonblank_lines {
            last_num = print_nonblank_line(last_num, &line);
        } else {
            println!("{}", line);
        }

    }

    Ok(())
}

fn print_nonblank_line(mut last_num: usize, line: &str) -> usize {
    if !line.is_empty() {
        last_num += 1;
        print_numbered_line(last_num, line);
    } else {
        println!();
    }
    last_num
}

fn print_numbered_line(line_num: usize, line: &str) {
    println!("{:6} {}", line_num, line);
}


fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
}
