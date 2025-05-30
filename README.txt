# BIP39 Seed Phrase Generator

A secure, production-grade tool for generating BIP39 seed phrases for cryptocurrency wallets.

## Features

- Generate 12 or 24 word seed phrases
- Cryptographically secure random number generation
- Checksum validation
- Comprehensive error handling
- Detailed logging
- Configurable output options
- Colorized terminal output
- Extensive test coverage

## Installation

```bash
cargo install bip39-generator
```

## Usage

Generate a 12-word seed phrase:
```bash
bip39-generator
```

Generate a 24-word seed phrase:
```bash
bip39-generator --words 24
```

Save to file:
```bash
bip39-generator --output seed.txt
```

Enable verbose logging:
```bash
bip39-generator --verbose
```

## Security Considerations

- All entropy is generated using cryptographically secure random number generation
- The program does not store or transmit seed phrases
- Memory containing sensitive data is zeroed after use
- No network connectivity required
- Checksum validation ensures phrase integrity

## Development

### Prerequisites

- Rust 1.70 or higher
- Cargo

### Building

```bash
cargo build --release
```

### Testing

```bash
cargo test
```

## License

Licensed under either of:

- Apache License, Version 2.0
- MIT license

at your option.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.