use crate::error::{Result, BIP39Error};
use crate::wordlist::WORD_INDICES;
use sha2::{Sha256, Digest};

pub struct SeedPhraseValidator;

impl SeedPhraseValidator {
    /// Validates a seed phrase
    pub fn validate(phrase: &str) -> Result<bool> {
        let words: Vec<&str> = phrase.split_whitespace().collect();
        
        // Check word count
        match words.len() {
            12 | 24 => (),
            len => return Err(BIP39Error::InvalidWordCount(len as u8)),
        }

        // Validate each word exists in the wordlist
        let indices: Result<Vec<usize>> = words
            .iter()
            .map(|&word| {
                WORD_INDICES
                    .get(word)
                    .copied()
                    .ok_or_else(|| BIP39Error::WordListError(format!("Invalid word: {}", word)))
            })
            .collect();

        let indices = indices?;
        
        // Convert indices back to entropy
        let entropy = Self::indices_to_entropy(&indices)?;
        
        // Validate checksum
        Self::validate_checksum(&entropy, words.len())
    }

    fn indices_to_entropy(indices: &[usize]) -> Result<Vec<u8>> {
        let mut entropy = Vec::new();
        let mut buffer = 0u32;
        let mut bits = 0;

        for &index in indices {
            buffer = (buffer << 11) | (index as u32);
            bits += 11;

            while bits >= 8 {
                bits -= 8;
                entropy.push((buffer >> bits) as u8);
            }
        }

        Ok(entropy)
    }

    fn validate_checksum(entropy: &[u8], word_count: usize) -> Result<bool> {
        let checksum_bits = word_count / 3;
        let entropy_bytes = (word_count * 11 - checksum_bits) / 8;
        
        let mut hasher = Sha256::new();
        hasher.update(&entropy[..entropy_bytes]);
        let hash = hasher.finalize();
        
        let calculated_checksum = hash[0] >> (8 - checksum_bits);
        let provided_checksum = entropy[entropy_bytes];
        
        Ok(calculated_checksum == provided_checksum)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_12_word_phrase() {
        let phrase = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        assert!(SeedPhraseValidator::validate(phrase).unwrap());
    }

    #[test]
    fn test_invalid_word() {
        let phrase = "abandon abandon abandon abandon abandon invalid abandon abandon abandon abandon abandon about";
        assert!(matches!(
            SeedPhraseValidator::validate(phrase),
            Err(BIP39Error::WordListError(_))
        ));
    }

    #[test]
    fn test_invalid_word_count() {
        let phrase = "abandon abandon abandon";
        assert!(matches!(
            SeedPhraseValidator::validate(phrase),
            Err(BIP39Error::InvalidWordCount(_))
        ));
    }
}