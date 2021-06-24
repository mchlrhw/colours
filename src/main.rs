struct Colour {
    r: u8,
    g: u8,
    b: u8,
}

struct Image {
    width: u8,
    height: u8,
    pixels: Vec<Colour>,
}

fn image_to_ppm(image: Image) -> String {
    let mut ppm = format!("P3\n{} {}\n255\n", image.width, image.height);

    for pixel in image.pixels {
        ppm += &format!("{} {} {}\n", pixel.r, pixel.g, pixel.b);
    }

    ppm
}

fn main() {
    let image = Image {
        width: 1,
        height: 1,
        pixels: vec![Colour { r: 0, g: 0, b: 0 }],
    };
    let ppm = image_to_ppm(image);
    println!("{}", ppm);
}
