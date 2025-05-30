use bip39_generator::{
    cli::{Cli, print_seed_phrase},
    config::Config,
    generator::SeedPhraseGenerator,
    error::Result,
    VERSION,
};
use clap::Parser;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

fn setup_logging(verbose: bool) {
    let level = if verbose {
        Level::DEBUG
    } else {
        Level::INFO
    };

    FmtSubscriber::builder()
        .with_max_level(level)
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .init();
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    setup_logging(cli.verbose);

    info!("Starting BIP39 seed phrase generator v{}", VERSION);

    // Load configuration
    let config = Config::load()?;

    // Use CLI word count or fall back to config default
    let word_count = if cli.words != 12 {
        cli.words
    } else {
        config.default_word_count
    };

    let generator = SeedPhraseGenerator::new(word_count)?;
    let words = generator.generate()?;

    if let Some(output_path) = cli.output {
        use std::fs::write;
        write(&output_path, words.join(" "))?;
        println!("Seed phrase written to: {}", output_path.display());
    } else {
        print_seed_phrase(&words);
        println!("\nBIP39 Path: m/84'/0'/0'/0/0 (Native SegWit)");
        println!("          m/44'/0'/0'/0/0 (Legacy)");
        println!("          m/49'/0'/0'/0/0 (SegWit)");
    }

    Ok(())
}