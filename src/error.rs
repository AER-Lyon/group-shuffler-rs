use thiserror::Error;

#[derive(Error, Debug)]
pub enum ShufflerError {
    #[error("Error reading CSV file: {0}")]
    CsvReadError(#[from] csv::Error),
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Invalid file type detected")]
    InvalidFileType,
}

pub type Result<T> = std::result::Result<T, ShufflerError>;
