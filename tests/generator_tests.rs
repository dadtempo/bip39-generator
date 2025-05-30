use bip39_generator::{
    generator::SeedPhraseGenerator,
    error::BIP39Error,
    validation::SeedPhraseValidator,
};
use proptest::prelude::*;
use test_case::test_case;

#[test]
fn test_generator_creation() {
    assert!(SeedPhraseGenerator::new(12).is_ok());
    assert!(SeedPhraseGenerator::new(24).is_ok());
    assert!(matches!(
        SeedPhraseGenerator::new(16),
        Err(BIP39Error::InvalidWordCount(16))
    ));
}

#[test]
fn test_12_word_generation() {
    let generator = SeedPhraseGenerator::new(12).unwrap();
    let words = generator.generate().unwrap();
    assert_eq!(words.len(), 12);
}

#[test]
fn test_24_word_generation() {
    let generator = SeedPhraseGenerator::new(24).unwrap();
    let words = generator.generate().unwrap();
    assert_eq!(words.len(), 24);
}

#[test]
fn test_generated_phrase_validation() {
    let generator = SeedPhraseGenerator::new(12).unwrap();
    let words = generator.generate().unwrap();
    let phrase = words.join(" ");
    assert!(SeedPhraseValidator::validate(&phrase).unwrap());
}

proptest! {
    #[test]
    fn test_random_generation_always_valid(word_count in prop_oneof![Just(12), Just(24)]) {
        let generator = SeedPhraseGenerator::new(word_count).unwrap();
        let words = generator.generate().unwrap();
        let phrase = words.join(" ");
        prop_assert!(SeedPhraseValidator::validate(&phrase).unwrap());
    }
}

#[test_case(12)]
#[test_case(24)]
fn test_entropy_distribution(word_count: u8) {
    let generator = SeedPhraseGenerator::new(word_count).unwrap();
    let mut word_frequencies = std::collections::HashMap::new();
    
    // Generate multiple phrases to check distribution
    for _ in 0..100 {
        let words = generator.generate().unwrap();
        for word in words {
            *word_frequencies.entry(word).or_insert(0) += 1;
        }
    }
    
    // Check that we're using a good variety of words
    assert!(word_frequencies.len() > word_count as usize * 2);
}

#[test]
fn test_thread_safety() {
    use std::thread;
    
    let handles: Vec<_> = (0..10)
        .map(|_| {
            thread::spawn(|| {
                let generator = SeedPhraseGenerator::new(12).unwrap();
                generator.generate().unwrap()
            })
        })
        .collect();
    
    for handle in handles {
        handle.join().unwrap();
    }
}