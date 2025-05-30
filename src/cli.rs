use clap::Parser;
use colored::*;
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "bip39-generator",
    about = "A secure BIP39 seed phrase generator",
    version,
    author
)]
pub struct Cli {
    /// Number of words to generate (12 or 24)
    #[arg(short = 'n', long, default_value = "12")]
    pub words: u8,

    /// Output file for the generated seed phrase
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Enable verbose logging
    #[arg(short, long)]
    pub verbose: bool,

    /// Verify an existing seed phrase
    #[arg(short = 'c', long, help = "Check/verify an existing seed phrase")]
    pub verify: bool,
}

pub fn print_seed_phrase(words: &[&str]) {
    println!("\n{}", "Generated Seed Phrase:".green().bold());
    println!("{}", "===================".green());
    
    for (i, word) in words.iter().enumerate() {
        print!("{:2}. {:<12}", i + 1, word.truecolor(255, 140, 0).bold());  // Bright orange color
        if (i + 1) % 4 == 0 {
            println!();
        }
    }
    println!("\n");
    println!("{}", "WARNING: Store this seed phrase securely!".red().bold());
    println!("{}", "Anyone with access to this phrase can access your funds.".red());
}