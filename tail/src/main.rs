use clap:: Parser;

fn main() -> tail::MyResult<()> {
    let args = tail::Args::parse();
    tail::run(args)
}