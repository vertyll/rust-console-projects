use clap::{Arg, ArgAction, Command};

fn main() {
    let matches = Command::new("echo")
        .author("gawrmiko@gmail.com")
        .version("1.0.0")
        .about("Echo your input")
        .arg(
            Arg::new("text")
                .required(true)
                .action(ArgAction::Append)
                .value_name("TEXT")
                .help("Input Text"),
        )
        .arg(
            Arg::new("omit-newline")
                .short('n')
                .long("no-newline")
                .action(ArgAction::SetTrue)
                .help("Don't print newline"),
        )
        .get_matches();

    let text: Vec<&str> = matches
        .get_many("text")
        .unwrap()
        .map(String::as_str)
        .collect();

    let omit_new_line: bool = *matches.get_one("omit-newline").unwrap();

    print!(
        "{}{}",
        text.join(" "),
        if omit_new_line { "" } else { "\n" }
    );
}