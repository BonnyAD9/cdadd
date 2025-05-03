use std::borrow::Cow;

use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid usage: {0}")]
    InvalidUsage(Cow<'static, str>),
    #[error("Failed to parse date: invalid date format")]
    ParseDate,
    #[error("Failed to parse cddb file.")]
    ParseCddb,
    #[error("Failed to parse featuring from track name: {0}")]
    ParseFeat(&'static str),
    #[error(transparent)]
    Logger(#[from] flexi_logger::FlexiLoggerError),
    #[error(transparent)]
    Ini(#[from] ini::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),
    #[error(transparent)]
    Pareg(#[from] Box<pareg::ArgError>),
    #[error(transparent)]
    Termal(#[from] termal::error::Error),
}

impl From<pareg::ArgError> for Error {
    fn from(value: pareg::ArgError) -> Self {
        Box::new(value).into()
    }
}
