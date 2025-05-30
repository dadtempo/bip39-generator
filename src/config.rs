use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use crate::error::{Result, BIP39Error};
use dirs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    /// Default number of words to generate
    pub default_word_count: u8,
    /// Default output directory for saved seed phrases
    pub output_directory: Option<PathBuf>,
    /// Whether to use colored output by default
    pub use_colors: bool,
    /// Log level (ERROR, WARN, INFO, DEBUG, TRACE)
    pub log_level: String,
    /// Whether to display warnings
    pub show_warnings: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            default_word_count: 12,
            output_directory: None,
            use_colors: true,
            log_level: "INFO".to_string(),
            show_warnings: true,
        }
    }
}

impl Config {
    /// Loads the configuration from the default location
    /// (~/.config/bip39-generator/config.yaml)
    /// If no configuration file exists, returns the default configuration
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;
        
        if !config_path.exists() {
            return Ok(Self::default());
        }

        let config = config::Config::builder()
            .add_source(config::File::from(config_path))
            .build()
            .map_err(|e| BIP39Error::ConfigError(e.to_string()))?;

        config.try_deserialize()
            .map_err(|e| BIP39Error::ConfigError(e.to_string()))
    }

    /// Saves the current configuration to the default location
    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;
        
        // Ensure the config directory exists
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| BIP39Error::ConfigError(e.to_string()))?;
        }

        let yaml = serde_yaml::to_string(self)
            .map_err(|e| BIP39Error::ConfigError(e.to_string()))?;

        std::fs::write(config_path, yaml)
            .map_err(|e| BIP39Error::ConfigError(e.to_string()))?;

        Ok(())
    }

    /// Returns the path to the configuration file
    fn config_path() -> Result<PathBuf> {
        let home = dirs::home_dir()
            .ok_or_else(|| BIP39Error::ConfigError("Could not find home directory".to_string()))?;
        
        Ok(home.join(".config/bip39-generator/config.yaml"))
    }

    /// Updates the configuration with command line arguments
    pub fn update_from_args(&mut self, words: Option<u8>, no_color: bool) {
        if let Some(word_count) = words {
            self.default_word_count = word_count;
        }
        if no_color {
            self.use_colors = false;
        }
    }

    /// Returns whether colored output should be used
    pub fn should_use_colors(&self) -> bool {
        self.use_colors
    }

    /// Returns the configured log level
    pub fn log_level(&self) -> &str {
        &self.log_level
    }

    /// Returns the default word count
    pub fn default_word_count(&self) -> u8 {
        self.default_word_count
    }

    /// Returns the output directory if configured
    pub fn output_directory(&self) -> Option<&PathBuf> {
        self.output_directory.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.default_word_count, 12);
        assert!(config.use_colors);
        assert_eq!(config.log_level, "INFO");
        assert!(config.show_warnings);
    }

    #[test]
    fn test_update_from_args() {
        let mut config = Config::default();
        config.update_from_args(Some(24), true);
        assert_eq!(config.default_word_count, 24);
        assert!(!config.use_colors);
    }

    #[test]
    fn test_save_and_load() -> Result<()> {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.yaml");
        
        let mut config = Config::default();
        config.default_word_count = 24;
        
        // Override the config path for testing
        std::env::set_var("HOME", temp_dir.path());
        
        config.save()?;
        let loaded = Config::load()?;
        
        assert_eq!(loaded.default_word_count, 24);
        Ok(())
    }
}