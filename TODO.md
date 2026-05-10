# Future Improvements Roadmap


- [ ] **Multi-Algorithm Support**: Store and support SHA-256 and SHA-512 alongside the default SHA-1.
- [ ] **Custom Digit Counts**: Allow accounts to specify 8-digit OTPs instead of the default 6.
- [ ] **URI Parsing**: Add a command to import accounts by pasting `otpauth://` URI strings.
- [ ] **Visual Countdown**: Display a real-time progress bar or numeric countdown showing when the current code expires.
- [ ] **Dashboard View**: A command to display current OTPs for all saved accounts in a single table.
- [ ] **Fuzzy Search**: Support partial name matching for the `generate` command.
- [ ] **QR Export**: Render terminal-based QR codes to easily export accounts back to mobile apps.
- [ ] **Account Management**: Commands to safely update or delete existing accounts without manual file editing.
- [ ] **Encryption at Rest**: Implement file encryption using a master password (e.g., using Argon2 and AES-256-GCM).
- [ ] **Memory Protection**: Automatically zero-out secret bytes in memory after calculation.
