use crate::error::{Result, ShufflerError};
use crate::models::{FileType, InputRecord, Record};
use rand::prelude::{IndexedRandom, SliceRandom};

pub fn process_csv_file(filename: &str, delimiter: u8, group_size: usize) -> Result<()> {
    let file_type = detect_file_type(filename, delimiter)?;

    match file_type {
        FileType::TeamsList => process_teams_file(filename, delimiter, group_size),
        FileType::LoginsList => process_logins_file(filename, delimiter, group_size),
    }
}

fn detect_file_type(filename: &str, delimiter: u8) -> Result<FileType> {
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(delimiter)
        .from_path(filename)?;

    let headers = reader.headers()?;

    if headers.iter().any(|h| h == "master") && headers.iter().any(|h| h == "member0") {
        Ok(FileType::TeamsList)
    } else if headers.iter().any(|h| h == "Login") {
        Ok(FileType::LoginsList)
    } else {
        Err(ShufflerError::InvalidFileType)
    }
}

fn process_teams_file(filename: &str, delimiter: u8, group_size: usize) -> Result<()> {
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

    generate_teams(&logins, delimiter, group_size)?;

    Ok(())
}

fn process_logins_file(filename: &str, delimiter: u8, group_size: usize) -> Result<()> {
    println!("Processing a login file...");

    let mut reader = csv::ReaderBuilder::new()
        .delimiter(delimiter)
        .from_path(filename)?;

    let records: Vec<Record> = reader
        .deserialize()
        .collect::<std::result::Result<_, _>>()?;

    generate_teams(&records, delimiter, group_size)?;

    Ok(())
}

fn generate_teams(records: &[Record], delimiter: u8, group_size: usize) -> Result<()> {
    let mut shuffled_records = records.to_vec();
    shuffled_records.shuffle(&mut rand::rng());

    let groups: Vec<_> = shuffled_records.chunks(group_size).collect();

    let output_path = get_output_filename();
    let mut writer = csv::WriterBuilder::new()
        .delimiter(delimiter)
        .from_path(&output_path)?;

    let mut headers: Vec<String> = vec!["name".to_string(), "code".to_string()];
    if group_size > 0 {
        headers.push("master".to_string());
        for i in 0..(group_size - 1) {
            headers.push(format!("member{}", i));
        }
    }
    writer.write_record(&headers)?;

    let words = diceware_wordlists::Wordlist::get_list(&Default::default());
    let mut rng = rand::rng();

    for group in groups {
        let mut row: Vec<String> = Vec::new();

        let random_word = format!(
            "{}_{}",
            words.choose(&mut rng).unwrap(),
            words.choose(&mut rng).unwrap()
        );
        row.push(random_word);
        row.push(String::new());

        for i in 0..group_size {
            row.push(group.get(i).map_or(String::new(), |rec| rec.login.clone()));
        }

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
