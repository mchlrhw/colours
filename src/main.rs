struct Image {}

fn image_to_ppm(_image: Image) -> String {
    String::from(
        r#"P3
3 3
255
0 0 0
0 0 0
0 0 0
0 0 0
0 0 0
0 0 0
0 0 0
0 0 0
0 0 0"#,
    )
}

fn main() {
    let image = Image {};
    let ppm = image_to_ppm(image);
    println!("{}", ppm);
}
