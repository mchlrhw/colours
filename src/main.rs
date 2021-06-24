use std::env;

use anyhow::bail;

use colours::image::Image;

fn main() -> Result<(), anyhow::Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        bail!("usage: colours <colour>");
    }

    let colour = args[1].parse()?;

    let image = Image::new(100, 100, colour);
    eprintln!("{:#?}", image);

    println!("{}", image);

    Ok(())
}
