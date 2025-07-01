const BASE64_TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

pub fn hex_to_base64(hex: &str) -> Result<String, &'static str> {
    // Convert hex string to bytes
    let mut bytes = Vec::new();
    let chars: Vec<char> = hex.chars().collect();
    if chars.len() % 2 != 0 {
        return Err("Hex string has odd length");
    }
    for i in (0..chars.len()).step_by(2) {
        let hi = chars[i].to_digit(16).ok_or("Invalid hex")?;
        let lo = chars[i + 1].to_digit(16).ok_or("Invalid hex")?;
        bytes.push((hi << 4 | lo) as u8);
    }

    // Encode bytes to base64
    let mut base64 = String::new();
    let mut i = 0;
    while i < bytes.len() {
        let b0 = bytes[i];
        let b1 = if i + 1 < bytes.len() { bytes[i + 1] } else { 0 };
        let b2 = if i + 2 < bytes.len() { bytes[i + 2] } else { 0 };

        base64.push(BASE64_TABLE[(b0 >> 2) as usize] as char);
        base64.push(BASE64_TABLE[(((b0 & 0b11) << 4) | (b1 >> 4)) as usize] as char);

        if i + 1 < bytes.len() {
            base64.push(BASE64_TABLE[(((b1 & 0b1111) << 2) | (b2 >> 6)) as usize] as char);
        } else {
            base64.push('=');
        }

        if i + 2 < bytes.len() {
            base64.push(BASE64_TABLE[(b2 & 0b111111) as usize] as char);
        } else {
            base64.push('=');
        }

        i += 3;
    }

    Ok(base64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_base64() {
        let hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        assert_eq!(hex_to_base64(hex).unwrap(), expected);
    }
}