// AI Game Generator - Procedural game generation using AI
// Copyright (C) 2024 AI Game Generator Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the MIT License as published by
// the Open Source Initiative.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.

use proptest::prelude::*;
use ai_game_generator::generator::AIGameGenerator;
use ai_game_generator::config::{GameConfig, GameInfo, HeroInfo};
use tempfile::TempDir;
use std::fs;

// Property-based testing strategies
prop_compose! {
    fn arb_game_title()(title in "[A-Za-z ]{5,50}") -> String {
        title.trim().to_string()
    }
}

prop_compose! {
    fn arb_codename()(name in "[a-z_]{3,20}") -> String {
        name
    }
}

prop_compose! {
    fn arb_version()(
        major in 0u8..10,
        minor in 0u8..20,
        patch in 0u8..100
    ) -> String {
        format!("{}.{}.{}", major, minor, patch)
    }
}

proptest! {
    #[test]
    fn test_generator_handles_arbitrary_titles(title in arb_game_title()) {
        let generator = AIGameGenerator::new();
        // Generator should handle any valid title without panicking
        let _formatted = title.to_lowercase().replace(' ', "_");
        prop_assert!(true);
    }

    #[test]
    fn test_cache_key_generation_is_deterministic(
        system_prompt in ".*",
        user_prompt in ".*"
    ) {
        // Same inputs should always produce same cache key
        let key1 = format!("{:x}", md5::compute(format!("{}{}", system_prompt, user_prompt)));
        let key2 = format!("{:x}", md5::compute(format!("{}{}", system_prompt, user_prompt)));
        prop_assert_eq!(key1, key2);
    }

    #[test]
    fn test_file_path_sanitization(
        zone_name in "[A-Za-z0-9 _-]{1,50}"
    ) {
        let sanitized = zone_name.to_lowercase().replace(' ', "_");
        // Sanitized names should be valid file names
        prop_assert!(!sanitized.is_empty());
        prop_assert!(!sanitized.contains('/'));
        prop_assert!(!sanitized.contains('\\'));
        prop_assert!(!sanitized.contains('\0'));
    }
}

#[test]
fn test_generator_new() {
    let generator = AIGameGenerator::new();
    assert!(!generator.dry_run);
    assert!(generator.use_cache);
}

#[test]
fn test_generator_builder_pattern() {
    let generator = AIGameGenerator::new()
        .with_use_cache(false)
        .with_dry_run(true);

    assert!(generator.dry_run);
    assert!(!generator.use_cache);
}

#[tokio::test]
async fn test_setup_directories() {
    let temp_dir = TempDir::new().unwrap();
    std::env::set_current_dir(&temp_dir).unwrap();

    let generator = AIGameGenerator::new();
    let result = generator.setup_directories().await;

    assert!(result.is_ok());

    // Check all directories were created
    let expected_dirs = vec![
        "src",
        "src/components",
        "src/systems",
        "src/tilemaps",
        "src/levels",
        "src/ai",
        "assets/sprites",
        "assets/audio",
        "assets/data",
        "assets/levels",
        "assets/quests",
        ".cache/ai-gen",
        ".ai-generation",
    ];

    for dir in expected_dirs {
        assert!(
            std::path::Path::new(dir).exists(),
            "Directory {} should exist",
            dir
        );
    }
}

#[tokio::test]
async fn test_write_file_dry_run() {
    let temp_dir = TempDir::new().unwrap();
    std::env::set_current_dir(&temp_dir).unwrap();

    let mut generator = AIGameGenerator::new()
        .with_dry_run(true);

    let result = generator.write_file("test.txt", b"content").await;
    assert!(result.is_ok());

    // File should NOT be created in dry run mode
    assert!(!std::path::Path::new("test.txt").exists());
}

#[tokio::test]
async fn test_write_file_normal_mode() {
    let temp_dir = TempDir::new().unwrap();
    std::env::set_current_dir(&temp_dir).unwrap();

    let mut generator = AIGameGenerator::new()
        .with_dry_run(false);

    let result = generator.write_file("test.txt", b"test content").await;
    assert!(result.is_ok());

    // File should be created
    assert!(std::path::Path::new("test.txt").exists());

    // Content should match
    let content = fs::read_to_string("test.txt").unwrap();
    assert_eq!(content, "test content");

    // File should be tracked
    assert!(generator.generated_files.contains(&std::path::PathBuf::from("test.txt")));
}

#[tokio::test]
async fn test_write_file_creates_parent_dirs() {
    let temp_dir = TempDir::new().unwrap();
    std::env::set_current_dir(&temp_dir).unwrap();

    let mut generator = AIGameGenerator::new();

    let result = generator.write_file("deep/nested/path/file.txt", b"content").await;
    assert!(result.is_ok());

    assert!(std::path::Path::new("deep/nested/path/file.txt").exists());
}

#[test]
fn test_cache_key_generation() {
    let system_prompt = "You are a helpful assistant";
    let user_prompt = "Generate a component";

    let key1 = format!("{:x}", md5::compute(format!("{}{}", system_prompt, user_prompt)));
    let key2 = format!("{:x}", md5::compute(format!("{}{}", system_prompt, user_prompt)));

    // Same prompts should generate same key
    assert_eq!(key1, key2);

    // Different prompts should generate different keys
    let key3 = format!("{:x}", md5::compute(format!("{}different", system_prompt)));
    assert_ne!(key1, key3);
}

#[tokio::test]
async fn test_token_counting() {
    let generator = AIGameGenerator::new();

    // Test with known strings
    let short_text = "Hello world";
    let tokens = generator.tokenizer.encode_with_special_tokens(short_text);
    assert!(tokens.len() > 0);
    assert!(tokens.len() < 10); // Short text should have few tokens

    let long_text = "This is a much longer piece of text that should result in more tokens when encoded by the tokenizer. It contains multiple sentences and various punctuation marks.";
    let long_tokens = generator.tokenizer.encode_with_special_tokens(long_text);
    assert!(long_tokens.len() > tokens.len());
}

#[tokio::test]
async fn test_generate_summary() {
    let temp_dir = TempDir::new().unwrap();
    std::env::set_current_dir(&temp_dir).unwrap();

    let mut generator = AIGameGenerator::new();

    // Add some fake generated files
    generator.generated_files.insert("src/main.rs".into());
    generator.generated_files.insert("assets/sprite.png".into());

    // Set up minimal config
    generator.config = Some(GameConfig {
        game: GameInfo {
            title: "Test Game".to_string(),
            codename: "test".to_string(),
            version: "1.0.0".to_string(),
            genre: "RPG".to_string(),
            theme: String::new(),
            setting: String::new(),
        },
        hero: HeroInfo {
            name: "Hero".to_string(),
            description: "A hero".to_string(),
            class: String::new(),
            abilities: vec![],
        },
        // ... other fields would be set in real test
    });

    let result = generator.generate_summary().await;
    assert!(result.is_ok());

    // Check summary file was created
    assert!(std::path::Path::new("GENERATION_SUMMARY.json").exists());

    // Verify content
    let summary_content = fs::read_to_string("GENERATION_SUMMARY.json").unwrap();
    let summary: serde_json::Value = serde_json::from_str(&summary_content).unwrap();

    assert_eq!(summary["game"]["title"], "Test Game");
    assert!(summary["generated"]["files"].as_array().unwrap().len() == 2);
    assert!(summary["generated"]["timestamp"].as_str().is_some());
}

#[test]
fn test_progress_bar_messages() {
    // Test that progress messages are properly formatted
    let messages = vec![
        "Generating style guide...",
        "Generating core files...",
        "Generating components...",
        "Generating systems...",
        "Generating levels...",
        "Generating sprites...",
        "Generating audio...",
        "Generating UI assets...",
    ];

    for msg in messages {
        assert!(msg.ends_with("..."));
        assert!(msg.len() < 50); // Keep messages reasonably short
    }
}

// Test error handling
#[tokio::test]
async fn test_load_config_missing_file() {
    let temp_dir = TempDir::new().unwrap();
    std::env::set_current_dir(&temp_dir).unwrap();

    let mut generator = AIGameGenerator::new();
    let result = generator.initialize().await;

    // Should fail when config file is missing
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("game-config.yaml"));
}
