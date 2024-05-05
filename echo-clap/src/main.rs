use clap::{Parser};

/// Echo your input
#[derive(Parser, Debug)]
#[command(author = "gawrmiko@gmail.com", version = "1.0.0", about = "Echo your input")]
struct Args {
    /// Input text
    #[arg(required = true)]
    text: Vec<String>,

    /// Don't print newline
    #[arg(short = 'n', long = "no-newline")]
    omit_newline: bool,
}

fn  main() {
    let args = Args::parse();

    print!(
        "{}{}",
        args.text.join(" "),
        if args.omit_newline {""} else {"\n"}
    );
}