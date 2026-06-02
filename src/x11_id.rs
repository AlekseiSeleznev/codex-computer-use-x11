#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseX11WindowIdError {
    Empty,
    InvalidHex,
}

pub fn parse_x11_window_id(input: &str) -> Result<u64, ParseX11WindowIdError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(ParseX11WindowIdError::Empty);
    }

    if let Some(hex) = trimmed
        .strip_prefix("0x")
        .or_else(|| trimmed.strip_prefix("0X"))
    {
        if hex.is_empty() {
            return Err(ParseX11WindowIdError::Empty);
        }
        return u64::from_str_radix(hex, 16).map_err(|_| ParseX11WindowIdError::InvalidHex);
    }

    trimmed
        .parse::<u64>()
        .map_err(|_| ParseX11WindowIdError::InvalidHex)
}

#[cfg(test)]
mod tests {
    use super::parse_x11_window_id;

    #[test]
    fn equivalent_hex_ids_parse_to_same_u64() {
        let short = parse_x11_window_id("0x5624b36").expect("short hex parses");
        let padded = parse_x11_window_id("0x05624b36").expect("padded hex parses");
        assert_eq!(short, padded);
        assert_eq!(short, 0x05624b36);
    }

    #[test]
    fn decimal_ids_parse_without_hex_prefix() {
        assert_eq!(parse_x11_window_id("10"), Ok(10));
    }

    #[test]
    fn invalid_inputs_return_specific_errors() {
        assert_eq!(
            parse_x11_window_id(""),
            Err(super::ParseX11WindowIdError::Empty)
        );
        assert_eq!(
            parse_x11_window_id("0xnothex"),
            Err(super::ParseX11WindowIdError::InvalidHex)
        );
    }
}
