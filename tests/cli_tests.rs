use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;
use std::fs;

#[test]
fn test_default_generation() {
    let mut cmd = Command::cargo_bin("bip39-generator").unwrap();
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Generated Seed Phrase"))
        .stdout(predicate::str::contains("WARNING"));
}

#[test]
fn test_24_word_generation() {
    let mut cmd = Command::cargo_bin("bip39-generator").unwrap();
    
    cmd.arg("--words")
        .arg("24")
        .assert()
        .success()
        .stdout(predicate::str::contains("Generated Seed Phrase"));
}

#[test]
fn test_invalid_word_count() {
    let mut cmd = Command::cargo_bin("bip39-generator").unwrap();
    
    cmd.arg("--words")
        .arg("16")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Invalid word count"));
}

#[test]
fn test_output_to_file() {
    let temp_dir = TempDir::new().unwrap();
    let output_file = temp_dir.path().join("seed.txt");
    
    let mut cmd = Command::cargo_bin("bip39-generator").unwrap();
    
    cmd.arg("--output")
        .arg(&output_file)
        .assert()
        .success();
    
    assert!(output_file.exists());
    
    let content = fs::read_to_string(&output_file).unwrap();
    assert_eq!(content.split_whitespace().count(), 12);
}

#[test]
fn test_verbose_output() {
    let mut cmd = Command::cargo_bin("bip39-generator").unwrap();
    
    cmd.arg("--verbose")
        .assert()
        .success()
        .stdout(predicate::str::contains("DEBUG"));
}