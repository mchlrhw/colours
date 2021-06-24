mod colour;
pub mod hex;
pub mod image;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid colour: {0}")]
    InvalidColour(String),
    #[error("invalid hex colour: {0}")]
    InvalidHexColour(String),
    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),
    #[error(transparent)]
    Regex(#[from] regex::Error),
}

type PixelValue = u8;
