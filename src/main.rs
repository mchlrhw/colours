use std::{env, fmt, str::FromStr};

use anyhow::bail;

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("invalid colour: {0}")]
    InvalidColour(String),
    #[error("invalid hex colour: {0}")]
    InvalidHexColour(String),
}

type PixelValue = u8;

#[derive(Debug, Clone)]
struct Colour {
    r: PixelValue,
    g: PixelValue,
    b: PixelValue,
}

impl Colour {
    fn from_rgb(rgb: (u8, u8, u8)) -> Self {
        Self {
            r: rgb.0,
            b: rgb.1,
            g: rgb.2,
        }
    }
}

fn parse_hex(s: &str) -> Result<(u8, u8, u8), Error> {
    return Err(Error::InvalidHexColour(s.to_string()));
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

#[derive(Debug)]
struct Image {
    width: u8,
    height: u8,
    pixels: Vec<Colour>,
}

impl Image {
    fn new(width: u8, height: u8, colour: Colour) -> Self {
        let mut pixels = Vec::with_capacity(width as usize * height as usize);
        for _ in 0..pixels.capacity() {
            pixels.push(colour.clone());
        }

        Self {
            width,
            height,
            pixels,
        }
    }
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "P3\n{} {}\n{}\n",
            self.width,
            self.height,
            PixelValue::MAX,
        )?;

        for pixel in &self.pixels {
            write!(f, "{} {} {}\n", pixel.r, pixel.g, pixel.b)?;
        }

        Ok(())
    }
}

fn main() -> Result<(), anyhow::Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        bail!("usage: colours <colour>");
    }

    let colour = args[1].parse()?;

    let image = Image::new(10, 10, colour);
    eprintln!("{:#?}", image);

    println!("{}", image);

    Ok(())
}
