//! Utilities for handling SHA-256 hashes.

/// Converts a SHA-256 hash in hexadecimal string format to a byte array.
#[derive(Debug)]
#[non_exhaustive]
pub enum HexDecodeError {
    /// The provided hex string does not have the correct length.
    InvalidLength,
    /// The provided hex string contains invalid characters.
    InvalidHexCharacter,
}

/// Converts a SHA-256 hash in hexadecimal string format to a byte array.
///
/// # Errors
///
/// Returns [`HexDecodeError`] if the input string is not a valid SHA-256 hex string.
pub fn sha256_hex_to_bytes(hex: &str) -> Result<[u8; 32], HexDecodeError> {
    if hex.len() != 64 {
        return Err(HexDecodeError::InvalidLength);
    }

    let mut out = [0u8; 32];
    let bytes = hex.as_bytes();

    for i in 0..32 {
        let hi = from_hex(bytes[i * 2])?;
        let lo = from_hex(bytes[i * 2 + 1])?;
        out[i] = (hi << 4) | lo;
    }

    Ok(out)
}

#[inline]
fn from_hex(b: u8) -> Result<u8, HexDecodeError> {
    match b {
        b'0'..=b'9' => Ok(b - b'0'),
        b'a'..=b'f' => Ok(b - b'a' + 10),
        b'A'..=b'F' => Ok(b - b'A' + 10),
        _ => Err(HexDecodeError::InvalidHexCharacter),
    }
}
