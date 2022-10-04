use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Word to search for
    #[arg(short, long)]
    pub word: String,
    /// Search for definition
    #[arg(short, long, default_value_t = true)]
    pub defintion: bool,
    /// Search for synonym
    #[arg(short, long, default_value_t = false)]
    pub synonym: bool,
    /// Search for antonym
    #[arg(short, long, default_value_t = false)]
    pub antonym: bool,
}