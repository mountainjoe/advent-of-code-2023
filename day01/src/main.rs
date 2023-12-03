use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    puzzle_input: String
}

fn main() {
    let args = Args::parse();
    println!("input: {:?}", args.puzzle_input);
}
