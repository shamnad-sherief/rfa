# rfa (Rust 2-Factor Authentication)

A simple, lightweight command-line tool for generating 2FA (TOTP) codes, written in Rust.

## Features

- **Add Accounts**: Securely store your 2FA secrets.
- **Generate Codes**: Generate standard 6-digit TOTP codes (30-second intervals).
- **List Accounts**: View all stored account names.
- **Local Storage**: Data is stored in a simple local file named `2fa`.

## Installation

Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed.

```bash
git clone <your-repo-url>
cd rfa
cargo build --release
```

## Usage

### 1. Add a new account
To add a secret (Base32 encoded) for a service:
```bash
cargo run -- add <account_name> <secret>
```
*Example:* `cargo run -- add github TQVIW6UAU7wZl5fA`

### 2. Generate a 2FA code
To get the current 6-digit code:
```bash
cargo run -- generate <account_name>
```
*Example:* `cargo run -- generate github`

### 3. List all accounts
```bash
cargo run -- list
```

## Storage
Accounts are stored in a file named `2fa` in the project root in the following format:
```
<name> <secret>
```

## Dependencies
- `base32`: For decoding secrets.
- `hmac` & `sha1`: For generating the TOTP hash.
- `std::time`: For time-based synchronization.

## License
MIT / Apache 2.0
