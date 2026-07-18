# Future Improvements Roadmap

* [ ] **Account Deletion**: Remove saved accounts using a dedicated `remove` command.
* [x] **Duplicate Protection**: Prevent adding multiple accounts with the same name.
* [ ] **Graceful Duplicate Handling**: If a user tries to add a duplicate service, require them to explicitly provide an `account_id` to disambiguate.
* [ ] **Account Updates**: Update account names or secrets without manually editing files.
* [ ] **Generate All Codes**: Display OTPs for all saved accounts in a single command.
* [ ] **Dashboard View**: Show account names, OTPs, and remaining validity time in a table.
* [ ] **Countdown Timer**: Display seconds remaining before the current OTP expires.
* [ ] **Multi-Algorithm Support**: Support SHA-256 and SHA-512 alongside the default SHA-1.
* [ ] **Custom Digit Counts**: Allow accounts to specify 6-digit or 8-digit OTPs.S
* [ ] **JSON Storage**: Migrate from plain text storage to a structured JSON format.
* [ ] **Fuzzy Search**: Support partial account name matching for the `generate` command.
* [ ] **URI Parsing**: Import accounts from `otpauth://` URI strings.
* [ ] **QR Export**: Generate terminal QR codes for exporting accounts to authenticator apps.
* [ ] **Home Directory Storage**: Store account data in a platform-appropriate user directory.
* [ ] **Configuration File**: Support custom settings such as default digits and storage paths.
* [ ] **Encryption at Rest**: Protect stored secrets using a master password and modern encryption.
* [ ] **Memory Protection**: Automatically clear sensitive secret data from memory after use.
* [ ] **Clipboard Support**: Copy generated OTPs directly to the system clipboard.
* [ ] **Backup & Restore**: Export and import account databases for migration and backups.
