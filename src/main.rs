use std::fmt;

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("invalid colour: {0}")]
    InvalidColour(String),
}

type PixelValue = u8;

#[derive(Debug, Clone)]
struct Colour {
    r: PixelValue,
    g: PixelValue,
    b: PixelValue,
}

impl Colour {
    fn from_str(spec: &str) -> Result<Self, Error> {
        let colour = match spec {
            "black" => Self { r: 0, g: 0, b: 0 },
            _ => return Err(Error::InvalidColour(spec.to_string())),
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
    let colour = Colour::from_str("black")?;
    let image = Image::new(10, 10, colour);
    eprintln!("{:#?}", image);
    println!("{}", image);

    Ok(())
}
