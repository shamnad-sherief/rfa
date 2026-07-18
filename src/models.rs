#[derive(Debug)]
pub struct Account {
    /// The service name: e.g., "github", "aws", "google"
    pub service: String,

    /// The user identifier: e.g., "minato@example.com" or "personal" (Optional)
    pub account_id: Option<String>,

    /// The Base32 encoded secret key
    pub secret: String,

    /// The hash algorithm used (defaults to SHA1 for 99% of services)
    pub algorithm: Algorithm,

    /// How many digits the generated code should be (usually 6, sometimes 8)
    pub digits: u8,

    /// How many seconds the code is valid for (almost always 30)
    pub period: u64,
}

impl Account {
    pub fn new(name: String, secret: String) -> Self {
        Self {
            service: name,
            account_id: None,
            secret: secret,
            algorithm: Algorithm::Sha1,
            digits: 6,
            period: 30,
        }
    }
}

#[derive(Debug)]
enum Algorithm {
    Sha1,
    Sha256,
    Sha512,
}
