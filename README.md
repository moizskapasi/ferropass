# FerroPass

FerroPass is a secure, command-line password manager built in Rust. It offers robust encryption, strong password generation, and simple credential management in a lightweight package.

## Features

- **Strong Encryption**: AES-256-GCM encryption with Argon2 key derivation
- **Secure Password Generation**: Creates strong, randomized passwords that meet modern security standards
- **Offline Storage**: All data is stored locally in encrypted database files
- **Command-Line Interface**: Simple TUI (Text User Interface) for ease of use
- **Clipboard Integration**: Copy passwords to clipboard without displaying them on screen
- **Multiple Databases**: Create and manage separate password databases for different purposes

## Installation

### Prerequisites

- Rust and Cargo (1.86.0 or later recommended)

### Building from Source

1. Clone the repository:
```
git clone https://github.com/moizskapasi/ferropass.git
cd ferropass
```

2. Build the application:
```
cargo build --release
```

3. Run the application:
```
./target/release/ferropass
```

## Usage

### Creating a New Database

1. Start FerroPass and select "Create a new password database"
2. Enter a name for your database (will be saved with a `.fp` extension)
3. Create a master passkey (must be at least 15 characters with uppercase, lowercase, numbers, and special characters)

### Opening an Existing Database

1. Start FerroPass and select "Open an existing password database"
2. Enter the absolute path to your `.fp` database file
3. Enter your master passkey

### Managing Accounts

Within a database, you can:

- **List Accounts**: View all stored accounts
- **View/Edit Account**: Edit usernames, descriptions, or passwords
- **Add New Account**: Store credentials for a new service
- **Delete Account**: Remove an account from the database
- **Copy Password**: Copy an account's password to your clipboard

### Password Generation

FerroPass can generate secure passwords for you that:
- Are 20 characters long
- Include uppercase and lowercase letters
- Include numbers
- Include special characters
- Are randomly shuffled for maximum security

## Security Features

- **Zero Trust**: Your master passkey is never stored anywhere
- **Key Derivation**: Uses Argon2id to derive encryption keys from your passkey
- **Authenticated Encryption**: AES-256-GCM provides both confidentiality and integrity
- **No Plaintext**: Passwords are never displayed on screen unless explicitly requested
- **Memory Safety**: Built in Rust for memory safety and thread safety

## Database Structure

FerroPass databases (`.fp` files) contain:
- Encrypted account details (usernames, passwords, descriptions)
- Salt for key derivation
- Nonce for encryption
- All data is stored in a tamper-evident format

## Dependencies

- `aes-gcm`: For AES-256-GCM encryption
- `argon2`: For secure key derivation
- `rand`: For cryptographically secure random number generation
- `serde` & `serde_json`: For serialization
- `rpassword`: For secure password input
- `crossterm`: For terminal interface
- `clipboard`: For clipboard operations
- `base64`: For encoding binary data
- `rust-crypto`: For hashing operations

## Security Recommendations

- Use a strong, unique master passkey that you can remember but others can't guess
- Store your `.fp` database files in a secure location
- Consider creating separate databases for different security domains (e.g., personal, work)
- Regularly update your passwords for critical accounts
- FerroPass cannot protect against malware that records keystrokes or monitors your system memory

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

---

Made with ♥ in Rust
