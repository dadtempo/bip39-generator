use crate::error::{Result, BIP39Error};
use rand::{rngs::OsRng, RngCore, Rng};  // Using OsRng for better entropy
use sha2::{Digest, Sha256, Sha512};
use tracing::{debug, info, warn};
use hmac::Hmac;
use pbkdf2::pbkdf2;
use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct SeedPhraseGenerator {
    word_count: u8,
}

impl SeedPhraseGenerator {
    pub fn new(word_count: u8) -> Result<Self> {
        if word_count != 12 && word_count != 24 {
            return Err(BIP39Error::InvalidWordCount(word_count));
        }
        
        // Verify system entropy source
        if !Self::verify_entropy_source() {
            warn!("System entropy source might not be optimal");
        }
        
        Ok(Self { word_count })
    }

    /// Verifies the quality of the system's entropy source
    fn verify_entropy_source() -> bool {
        let mut samples = Vec::with_capacity(1000);
        let mut rng = OsRng;
        
        // Collect 1000 samples
        for _ in 0..1000 {
            samples.push(rng.next_u64());
        }
        
        // Basic statistical tests
        let mut zeros = 0;
        let mut ones = 0;
        for sample in &samples {
            for i in 0..64 {
                if (sample >> i) & 1 == 0 {
                    zeros += 1;
                } else {
                    ones += 1;
                }
            }
        }
        
        // Check if the distribution is reasonably balanced
        let ratio = (zeros as f64) / (ones as f64);
        (0.95..=1.05).contains(&ratio)
    }

    /// Generates additional entropy from system state
    fn generate_additional_entropy() -> Vec<u8> {
        let mut hasher = DefaultHasher::new();
        
        // Hash various system-specific values
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().hash(&mut hasher);
        std::process::id().hash(&mut hasher);
        std::thread::current().id().hash(&mut hasher);
        
        // Get some CPU-specific timing variations
        let start = SystemTime::now();
        for _ in 0..1000 { let _ = OsRng.next_u64(); }
        let duration = SystemTime::now().duration_since(start).unwrap();
        duration.hash(&mut hasher);
        
        let value = hasher.finish();
        value.to_le_bytes().to_vec()
    }

    pub fn generate(&self) -> Result<Vec<&'static str>> {
        // Calculate required entropy:
        // 12 words = 128 bits = 16 bytes
        // 24 words = 256 bits = 32 bytes
        let entropy_bytes = if self.word_count == 12 { 16 } else { 32 };
        
        // Generate primary entropy using OS-provided CSPRNG
        let mut primary_entropy = vec![0u8; entropy_bytes];
        OsRng.fill_bytes(&mut primary_entropy);
        
        // Generate additional entropy
        let additional_entropy = Self::generate_additional_entropy();
        
        // Combine entropies
        let mut hasher = Sha256::new();
        hasher.update(&primary_entropy);
        hasher.update(&additional_entropy);
        let final_entropy = hasher.finalize()[..entropy_bytes].to_vec();

        debug!("Generated {} bytes of entropy", entropy_bytes);
        
        // Calculate checksum bits (ENT/32)
        let checksum_bits = entropy_bytes / 4;
        
        // Calculate the checksum
        let mut hasher = Sha256::new();
        hasher.update(&final_entropy);
        let hash = hasher.finalize();
        
        // Create the checksum byte
        let checksum_byte = hash[0] >> (8 - checksum_bits);
        
        // Convert entropy to bits
        let mut bits = Vec::with_capacity(entropy_bytes * 8 + checksum_bits);
        for &byte in &final_entropy {
            for i in (0..8).rev() {
                bits.push((byte >> i) & 1);
            }
        }
        
        // Add checksum bits
        for i in 0..checksum_bits {
            bits.push((checksum_byte >> (checksum_bits - 1 - i)) & 1);
        }
        
        // Convert bits to words
        let mut words = Vec::with_capacity(self.word_count as usize);
        for chunk in bits.chunks(11) {
            let mut index = 0u16;
            for &bit in chunk {
                index = (index << 1) | bit as u16;
            }
            words.push(crate::wordlist::WORDLIST[index as usize]);
        }

        // Verify the generated phrase
        if !Self::verify_word_distribution(&words) {
            warn!("Generated phrase shows unusual word distribution");
        }

        debug!("Generated words with checksum");
        info!("Generated {} word seed phrase", self.word_count);
        
        Ok(words)
    }

    /// Verifies that the word distribution looks random enough
    fn verify_word_distribution(words: &[&str]) -> bool {
        use std::collections::HashMap;
        let mut first_char_count = HashMap::new();
        
        for word in words {
            *first_char_count.entry(word.chars().next().unwrap()).or_insert(0) += 1;
        }
        
        // Check if any character appears too frequently
        let max_occurrences = (words.len() as f64 * 0.5).ceil() as usize;
        first_char_count.values().all(|&count| count <= max_occurrences)
    }

    // ... rest of the implementation ...
}