use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// A filename
    #[arg(short, long)]
    pub filename: String,

    /// The delimiter to be used in CSV parsing.
    /// Defaults to ';' if not specified by the user.
    #[arg(short, long, default_value_t = ';')]
    pub delim: char,

    /// The desired size of each group.
    /// Defaults to 2 if not specified.
    #[arg(short, long, default_value_t = 2)]
    pub group_size: usize,
}
