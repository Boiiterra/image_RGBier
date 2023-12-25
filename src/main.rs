use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// File to read data from
    #[arg(short, long)]
    image: String,
}

fn main() {
    let args = Args::parse();
    println!("{}", args.image);
    println!("{}", "Hello, there.");
}
