use clap::{Arg, Command};

fn main() {
    let m = Command::new("My Program")
        .author("Me, me@mail.com")
        .version("1.0.2")
        .about("Explains in brief what the program does")
        .arg(
            Arg::new("in_file")
        )
        .after_help("Longer explanation to appear after the options when \
                 displaying the help information from --help or -h")
        .get_matches();

    let file = m.value_of("in_file").unwrap();
    println!("Przetwarzanie pliku: {}", file);
}
