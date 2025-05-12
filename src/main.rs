use clap::Parser;
use rand::prelude::{IndexedRandom, SliceRandom};
use rand::rng;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// A filename
    #[arg(short, long)]
    filename: String,

    /// The delimiter to be used in CSV parsing.
    /// Defaults to ';' if not specified by the user.
    #[arg(short, long, default_value_t = ';')]
    delim: char,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct InputRecord {
    #[serde(rename = "name", default)]
    name: String,
    #[serde(rename = "code", default)]
    code: String,
    #[serde(rename = "master", default)]
    master: String,
    #[serde(rename = "member0", default)]
    member0: String,
    #[serde(rename = "member1", default)]
    member1: String,
}

#[derive(Debug, Deserialize, Clone)]
struct Record {
    login: String,
}

fn process_csv_file(filename: &str, delimiter: u8) -> Result<(), Box<dyn std::error::Error>> {
    let file_type = detect_file_type(filename, delimiter)?;

    match file_type {
        FileType::TeamsList => process_teams_file(filename, delimiter),
        FileType::LoginsList => process_logins_file(filename, delimiter),
    }
}

enum FileType {
    TeamsList,  // Format "name;code;master;member0;member1"
    LoginsList, // Format avec au moins des logins
}

fn detect_file_type(filename: &str, delimiter: u8) -> Result<FileType, Box<dyn std::error::Error>> {
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(delimiter)
        .from_path(filename)?;

    let headers = reader.headers()?;

    // If the headers contain "master", "member0", it's probably a teams file ahahaahahahhaha
    if headers.iter().any(|h| h == "master") && headers.iter().any(|h| h == "member0") {
        Ok(FileType::TeamsList)
    } else {
        Ok(FileType::LoginsList)
    }
}

fn process_teams_file(filename: &str, delimiter: u8) -> Result<(), Box<dyn std::error::Error>> {
    println!("Processing an existing team file...");

    let mut reader = csv::ReaderBuilder::new()
        .delimiter(delimiter)
        .from_path(filename)?;

    let mut logins = Vec::new();
    for result in reader.deserialize() {
        let record: InputRecord = result?;
        if !record.master.is_empty() {
            logins.push(Record {
                login: record.master,
            });
        }
        if !record.member0.is_empty() {
            logins.push(Record {
                login: record.member0,
            });
        }
        if !record.member1.is_empty() {
            logins.push(Record {
                login: record.member1,
            });
        }
    }

    generate_teams(&logins, delimiter)?;

    Ok(())
}

fn process_logins_file(filename: &str, delimiter: u8) -> Result<(), Box<dyn std::error::Error>> {
    println!("Processing a login file...");

    let mut reader = csv::ReaderBuilder::new()
        .delimiter(delimiter)
        .from_path(filename)?;

    let records: Vec<Record> = reader.deserialize().collect::<Result<_, _>>()?;

    generate_teams(&records, delimiter)?;

    Ok(())
}

fn generate_teams(records: &[Record], delimiter: u8) -> Result<(), Box<dyn std::error::Error>> {
    let mut shuffled_records = records.to_vec();
    shuffled_records.shuffle(&mut rng());

    let pairs: Vec<_> = shuffled_records.chunks(2).collect();

    let output_path = get_output_filename();
    let mut writer = csv::WriterBuilder::new()
        .delimiter(delimiter)
        .from_path(&output_path)?;

    writer.write_record(&["name", "code", "master", "member0", "member1"])?;

    let words = diceware_wordlists::Wordlist::get_list(&Default::default()).to_vec();
    let mut rng = rand::rng();

    for pair in pairs {
        let master = pair.get(0).map_or(String::new(), |rec| rec.login.clone());
        let member0 = pair.get(1).map_or(String::new(), |rec| rec.login.clone());

        let random_word = format!(
            "{}_{}",
            words.choose(&mut rng).unwrap(),
            words.choose(&mut rng).unwrap()
        );

        let row = vec![random_word, String::new(), master, member0, String::new()];

        writer.write_record(&row)?;
    }

    writer.flush()?;
    println!("CSV file '{}' has been generated.", output_path);

    Ok(())
}

fn get_output_filename() -> String {
    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
    format!("generated_{}.csv", timestamp)
}

fn main() {
    let args = Args::parse();

    println!("Processing file: {}", args.filename);

    match process_csv_file(&args.filename, args.delim as u8) {
        Ok(_) => println!("Processing completed successfully."),
        Err(e) => eprintln!("Error during processing: {}", e),
    }
}
