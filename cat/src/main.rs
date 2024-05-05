use clap::Parser;

fn main() -> cat::MyResult<()> {
    let args = cat::Args::parse();
    cat::run(args)
}