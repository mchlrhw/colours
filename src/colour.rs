use std::str::FromStr;

use crate::{hex::parse_hex, Error, PixelValue};

#[derive(Debug, Clone)]
pub struct Colour {
    r: PixelValue,
    g: PixelValue,
    b: PixelValue,
}

impl Colour {
    pub fn from_rgb(rgb: (u8, u8, u8)) -> Self {
        Self {
            r: rgb.0,
            g: rgb.1,
            b: rgb.2,
        }
    }

    pub fn r(&self) -> u8 {
        self.r
    }

    pub fn g(&self) -> u8 {
        self.g
    }

    pub fn b(&self) -> u8 {
        self.b
    }
}

impl FromStr for Colour {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with('#') {
            let rgb = parse_hex(s)?;
            return Ok(Colour::from_rgb(rgb));
        }

        let colour = match s {
            "black" => Self { r: 0, g: 0, b: 0 },
            _ => return Err(Error::InvalidColour(s.to_string())),
        };

        Ok(colour)
    }
}
