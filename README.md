# rfa (Rust 2-Factor Authentication)

A blazing fast, minimalist command-line tool for managing and generating 2FA (TOTP) codes.

No electron apps, no cloud syncing, no bloated UI. Just your terminal and your TOTP tokens.


<img src="assets/demo.GIF" alt="rfa demo">


## Installation

You can install `rfa` directly from crates.io (the package is named `r2fa`):

```bash
cargo install r2fa
```

*(Note: Once installed, the command you type in your terminal is simply `rfa`)*

## Usage

### 1. Add a new account
To add a secret (Base32 encoded) for a service:
```bash
rfa add <account_name> <secret>
```
*Example:* `rfa add github TQVIW6UAU7WZL5FA`

### 2. Generate a 2FA code
To get the current 6-digit code for an account:
```bash
rfa <account_name>
```
*Example:* `rfa github`

### 3. List all accounts
To see all the accounts you have saved:
```bash
rfa list
```

## License
MIT
