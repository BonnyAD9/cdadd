use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to parse date: invalid date format")]
    ParseDate,
    #[error(transparent)]
    Logger(#[from] flexi_logger::FlexiLoggerError),
    #[error(transparent)]
    Ini(#[from] ini::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),
}
