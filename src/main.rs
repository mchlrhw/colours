#[derive(Debug, Clone)]
struct Colour {
    r: u8,
    g: u8,
    b: u8,
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

impl ToString for Image {
    fn to_string(&self) -> String {
        let mut ppm = format!("P3\n{} {}\n255\n", self.width, self.height);

        for pixel in &self.pixels {
            ppm += &format!("{} {} {}\n", pixel.r, pixel.g, pixel.b);
        }

        ppm
    }
}

fn main() {
    let image = Image::new(10, 10, Colour { r: 0, g: 0, b: 0 });
    eprintln!("{:#?}", image);
    let ppm = image.to_string();
    println!("{}", ppm);
}
