use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let omit_newline: bool = args.contains(&String::from("-n"));

    let start_index = if omit_newline { 2 } else { 1 };

    let text: Vec<&str> = args
        .iter()
        .skip(start_index)
        .map(|s| s.as_str())
        .collect();

    print!("{}", text.join(" "));

    if !omit_newline {
        println!();
    }
}
