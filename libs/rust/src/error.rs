use thiserror::Error;

#[derive(Error, Debug)]
pub enum LakhuaError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("H3 error: {0}")]
    H3(String),

    #[error("Invalid coordinate: lat={0}, lon={1}")]
    InvalidCoordinate(f64, f64),

    #[error("Invalid H3 index: {0}")]
    InvalidH3Index(String),

    #[error("Data file not found for resolution {0}")]
    DataFileNotFound(u8),
}
