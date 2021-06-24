use regex::Regex;

use crate::Error;

pub fn parse_hex(s: &str) -> Result<(u8, u8, u8), Error> {
    let re = Regex::new(r"#([0-9a-f]{2})([0-9a-f]{2})([0-9a-f]{2})")?;
    let caps = match re.captures(s) {
        Some(caps) => caps,
        None => return Err(Error::InvalidHexColour(s.to_string())),
    };

    let r = u8::from_str_radix(&caps[1], 16)?;
    let g = u8::from_str_radix(&caps[2], 16)?;
    let b = u8::from_str_radix(&caps[3], 16)?;

    Ok((r, g, b))
}
