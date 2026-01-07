mod args;
mod error;
mod logic;
mod models;

use clap::Parser;

fn main() {
    let args = args::Args::parse();

    println!("Processing file: {}", args.filename);

    match logic::process_csv_file(&args.filename, args.delim as u8, args.group_size) {
        Ok(_) => println!("Processing completed successfully."),
        Err(e) => eprintln!("Error during processing: {}", e),
    }
}

