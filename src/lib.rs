pub mod cli;
pub mod config;
pub mod error;
pub mod generator;
pub mod validation;
pub mod wordlist;

// Re-export commonly used types
pub use error::{BIP39Error, Result};
pub use generator::SeedPhraseGenerator;
pub use validation::SeedPhraseValidator;
pub use cli::{Cli, print_seed_phrase};

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");