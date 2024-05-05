use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};
use clap::Parser;
use once_cell::sync::OnceCell;
use regex::Regex;

pub type MyResult<T> = Result<T, Box<dyn Error>>;

static NUM_RE: OnceCell<Regex> = OnceCell::new();

#[derive(Debug)]
enum ParseValue {
    PlusZero,
    OtherNum(i64),
}

#[derive(Parser, Debug)]
#[command(author="gawrmiko@gmail.com", version="0.1.0", about="Rust tail")]
pub struct Args {
    #[arg(required = true, value_name = "FILE")]
    files: Vec<String>,

    #[arg(short='n', long="lines", value_name = "LINES", default_value = "10")]
    lines: String,

    #[arg(short='c', long="bytes", value_name = "BYTES", conflicts_with = "lines")]
    bytes: Option<String>,

    #[arg(short='q', long="quiet")]
    quiet: bool,
}

pub fn run(config: Args) -> MyResult<()> {
    let num_files = config.files.len();
    let lines_val = parse_num(&config.lines)?;
    let bytes_val = match &config.bytes {
        Some(val) => Some(parse_num(val)?),
        None => None,
    };

    for (file_num, filename) in config.files.iter().enumerate() {
        match File::open(filename) {
            Err(err) => eprintln!("{} {}", filename, err),
            Ok(file) => {
                if !config.quiet && num_files > 1 {
                    println!(
                        "{}==> {} <==",
                        if file_num > 0 {"\n"} else {""},
                        filename
                    )
                }

                let (total_lines, total_bytes) = count_lines_bytes(filename)?;
                let file = BufReader::new(file);
                match &bytes_val {
                    Some(bytes_value) => print_bytes(file, bytes_value, total_bytes)?,
                    None => print_lines(file, &lines_val, total_lines)?,
                }


            }
        }
    }
    Ok(())
}

fn print_lines(mut file: impl BufRead, num_lines: &ParseValue, total_lines: i64) -> MyResult<()> {
    if let Some(start) = get_start_index(num_lines, total_lines) {
        let mut line_num = 0;
        let mut buf = Vec::new();

        loop {
            let bytes_read = file.read_until(b'\n', &mut buf)?;
            if bytes_read == 0 {
                break;
            }
            if line_num >= start {
                print!("{}", String::from_utf8_lossy(&buf));
            }
            line_num += 1;
            buf.clear();
        }
    }

    Ok(())
}

fn get_start_index(take_val: &ParseValue, total: i64) -> Option<u64> {
    match take_val {
        ParseValue::PlusZero => {
            if total > 0 {
                Some(0)
            } else {
                None
            }
        }

        ParseValue::OtherNum(num) => {
            if num == &0 || total == 0 || num > &total {
                None
            } else {
                let start = if num <&0 {total + num} else {num - 1};
                Some(if start < 0 {0} else {start as u64})
            }
        }
    }
}

fn print_bytes<T>(mut file: T, num_bytes: &ParseValue, total_bytes: i64) -> MyResult<()>
    where
        T: Read + Seek
{
    if let Some(start) = get_start_index(num_bytes, total_bytes) {
        file.seek(SeekFrom::Start(start))?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        if !buffer.is_empty() {
            print!("{}", String::from_utf8_lossy(&buffer));
        }
    }

    Ok(())
}

fn count_lines_bytes(filename: &str) -> MyResult<(i64, i64)> {
    let mut file = BufReader::new(File::open(filename)?);
    let mut num_lines = 0;
    let mut num_bytes = 0;
    let mut buf = Vec::new();

    loop {
        let bytes_read = file.read_until(b'\n', &mut buf)?;
        if bytes_read == 0 {
            break;
        }
        num_lines += 1;
        num_bytes += bytes_read as i64;
        buf.clear();
    }

    Ok((num_lines, num_bytes))
}

fn parse_num(val: &str) -> MyResult<ParseValue> {
    let num_re = NUM_RE.get_or_init(|| Regex::new(r"^(?P<sign>[+-])?(?P<value>\d+)$").unwrap());

    match num_re.captures(val) {
        Some(caps) => {
            let sign = caps.name("sign").map_or("-", |m| m.as_str());
            let num = format!("{}{}", sign, caps.name("value").unwrap().as_str());
            if let Ok(val) = num.parse() {
                if sign == "+" && val == 0 {
                    Ok(ParseValue::PlusZero)
                } else {
                    Ok(ParseValue::OtherNum(val))
                }
            } else {
                Err(From::from(val))
            }
        }
        _ => Err(From::from(val))
    }
}