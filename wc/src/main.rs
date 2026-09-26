use wc::Config;
use clap:: Parser;

fn main() {
    let config = Config::parse();

    if let Err(e) = wc::run(config) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
