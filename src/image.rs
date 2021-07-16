use std::fmt;

use crate::{colour::Colour, PixelValue};

#[derive(Debug)]
pub struct Image<'a> {
    width: u8,
    height: u8,
    pixels: Vec<&'a Colour>,
}

impl<'a> Image<'a> {
    pub fn new(width: u8, height: u8, colour: &'a Colour) -> Self {
        let mut pixels = Vec::with_capacity(width as usize * height as usize);
        for _ in 0..pixels.capacity() {
            pixels.push(colour);
        }

        Self {
            width,
            height,
            pixels,
        }
    }
}

impl fmt::Display for Image<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "P3\n{} {}\n{}\n",
            self.width,
            self.height,
            PixelValue::MAX,
        )?;

        for pixel in &self.pixels {
            write!(f, "{} {} {}\n", pixel.r(), pixel.g(), pixel.b())?;
        }

        Ok(())
    }
}
