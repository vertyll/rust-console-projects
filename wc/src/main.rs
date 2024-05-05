use wc::Config;
use clap:: Parser;

fn main() {
    // The clap library automatically generates a parse function
    // based on the Config struct. This function will parse the command line arguments.
    let config = Config::parse();

    // Run the program with the parsed configuration.
    // If an error occurs, print the error and exit with a non-zero status code.
    if let Err(e) = wc::run(config) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
