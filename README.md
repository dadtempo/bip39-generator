# Secure BIP39 Seed Phrase Generator

A cryptographically secure BIP39 seed phrase generator for cryptocurrency wallets, featuring maximum entropy and robust security measures.

## Features

- ✨ Generate 12 or 24-word BIP39 seed phrases
- 🔒 Multiple entropy sources for maximum security
- 🧪 Statistical verification of entropy quality
- 💻 Completely offline operation
- 🛡️ Memory zeroing after use
- ✅ Comprehensive test suite

## Security Features

- Uses OS's secure random number generator (CSPRNG)
- Additional entropy from system state
- Multiple entropy sources combined using SHA-256
- Statistical verification of entropy source quality
- Verification of word distribution randomness
- No external network dependencies
- Secure memory handling

## Installation

### Building from Source (Recommended)

For maximum security, build from source:

```bash
# Clone the repository
git clone https://github.com/dadtempo/bip39-generator
cd bip39-generator

# Build in release mode
cargo build --release

# Run tests
cargo test
```

The binary will be available at `target/release/bip39_generator`

## Usage

Generate a 24-word seed phrase (recommended):
```bash
./target/release/bip39_generator -n 24
```

Generate a 12-word seed phrase:
```bash
./target/release/bip39_generator
```

Save to file:
```bash
./target/release/bip39_generator -n 24 -o seed.txt
```

Enable verbose logging:
```bash
./target/release/bip39_generator -v
```

## Security Best Practices

1. 🔒 Build and run on an offline, secure computer
2. 📝 Write down seed phrases on paper (multiple copies)
3. ❌ Never store seed phrases digitally
4. 🏢 Store backups in different secure locations
5. 🔑 Consider using a metal backup solution
6. 🔐 Add a strong passphrase (25th word)
7. ✅ Test wallet recovery before storing funds

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

This project is licensed under either of:

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

at your option.

## Security Audit Status

⚠️ This code has not been professionally audited. Use at your own risk.
