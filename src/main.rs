mod args;
use clap::Parser;

fn main() {
    let args = args::Args::parse();
    println!("Word {}", args.word);
    println!("Synonym {}", args.synonym);
    println!("Antonym {}", args.antonym);

}