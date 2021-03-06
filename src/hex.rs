use regex::Regex;

use crate::Error;

/// Convert a string of the form "#ffffff" into a tuple of three `u8`s
///
/// # Example
///
/// ```
/// # use colours::hex::parse_hex;
/// # fn main() -> Result<(), anyhow::Error> {
/// let hex_string = "#0000ff";
/// let parsed = parse_hex(hex_string)?;
///
/// assert_eq!(parsed, (0, 0, 255));
/// # Ok(())
/// # }
/// ```
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_hex() {
        let s = "#ffffff";
        let expected = (255, 255, 255);

        let parsed = parse_hex(s).unwrap();

        assert_eq!(parsed, expected);
    }

    #[test]
    #[should_panic]
    fn test_invalid_hex() {
        let s = "foobarbaz";

        parse_hex(s).unwrap();
    }
}
