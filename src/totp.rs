use base32::{Alphabet, decode};
use hmac::{Hmac, Mac};
use sha1::Sha1;

pub fn generate_totp(secret: &str, time_sec: u64) -> Result<String, TotpError> {
    let dec =
        decode(Alphabet::Rfc4648 { padding: true }, secret).ok_or(TotpError::Base32DecodeFailed)?;
    let current_time = time_sec / 30;

    let mut hasher: Hmac<Sha1> = Mac::new_from_slice(dec.as_ref())?;
    hasher.update(current_time.to_be_bytes().as_ref());
    let result = hasher.finalize();
    let hash = result.into_bytes();

    // Step 1: dynamic offset
    let offset = (hash[hash.len() - 1] & 0x0f) as usize;

    // Step 2: 4-byte slice
    let binary = ((hash[offset] as u32 & 0x7f) << 24)
        | ((hash[offset + 1] as u32) << 16)
        | ((hash[offset + 2] as u32) << 8)
        | (hash[offset + 3] as u32);

    // Step 3: mod to get OTP
    let otp = binary % 1_000_000;
    // println!("{:06}", otp);
    Ok(format!("{:06}", otp))
}

#[derive(Debug)]
pub enum TotpError {
    Base32DecodeFailed,
    SystemTimeBackwards,
    HmacInvalidKeyLength,
}

impl From<std::time::SystemTimeError> for TotpError {
    fn from(_value: std::time::SystemTimeError) -> Self {
        TotpError::SystemTimeBackwards
    }
}

impl From<hmac::digest::InvalidLength> for TotpError {
    fn from(_value: hmac::digest::InvalidLength) -> Self {
        Self::HmacInvalidKeyLength
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_totp_generation() {
        let secret = "GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ"; // A standard test key
        let time = 1111111111; // A fixed epoch timestamp

        let code = generate_totp(secret, time);
        assert_eq!(code.unwrap(), "050471");
    }
}
