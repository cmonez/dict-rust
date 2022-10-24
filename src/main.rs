mod args;
mod fetch;
mod parse;

use clap::Parser;
use fetch::fetch::*;

fn main() {
    let args = args::Args::parse();
    let word = args.word;
    // println!("Word {}", &word);
    // println!("Synonym {}", args.synonym);
    // println!("Antonym {}", args.antonym);
    get_definition(&word);
    get_synonym(&word);
    get_antonym(&word);
}