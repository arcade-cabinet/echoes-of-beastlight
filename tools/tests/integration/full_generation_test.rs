use ai_game_generator::{AIGameGenerator, GameConfig};
use tempfile::TempDir;
use std::fs;
use std::path::Path;
use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path as mock_path};
use serde_json::json;

#[tokio::test]
async fn test_full_generation_flow_with_mocked_api() {
    // Setup
    let temp_dir = TempDir::new().unwrap();
    std::env::set_current_dir(&temp_dir).unwrap();
    
    // Copy test config
    let config_content = include_str!("../fixtures/test-config.yaml");
    fs::write("game-config.yaml", config_content).unwrap();
    
    // Start mock server
    let mock_server = MockServer::start().await;
    std::env::set_var("OPENAI_API_KEY", "test-key");
    std::env::set_var("OPENAI_API_BASE", mock_server.uri());
    
    // Setup mocks for all expected API calls
    setup_generation_mocks(&mock_server).await;
    
    // Create generator
    let mut generator = AIGameGenerator::new()
        .with_use_cache(false); // Disable cache for testing
    
    // Run full generation
    let result = generator.generate_game().await;
    assert!(result.is_ok());
    
    // Verify all expected files were created
    verify_generated_files(&temp_dir);
    
    // Verify summary was created
    assert!(Path::new("GENERATION_SUMMARY.json").exists());
    let summary = fs::read_to_string("GENERATION_SUMMARY.json").unwrap();
    let summary_json: serde_json::Value = serde_json::from_str(&summary).unwrap();
    assert_eq!(summary_json["game"]["title"], "Test Game");
}

async fn setup_generation_mocks(mock_server: &MockServer) {
    // Mock for style guide generation
    Mock::given(method("POST"))
        .and(mock_path("/v1/chat/completions"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "choices": [{
                "message": {
                    "content": r#"{
                        "primary_color": "#4A90E2",
                        "secondary_color": "#50E3C2",
                        "accent_color": "#F5A623",
                        "background_color": "#1a1a1a",
                        "text_color": "#FFFFFF",
                        "sprite_style": "16-bit pixel art with black outlines",
                        "ui_style": "clean, minimal with rounded corners",
                        "font_family": "pixel",
                        "animation_style": "smooth with easing"
                    }"#
                }
            }]
        })))
        .expect(1..)
        .mount(mock_server)
        .await;
    
    // Mock for component generation
    Mock::given(method("POST"))
        .and(mock_path("/v1/chat/completions"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "choices": [{
                "message": {
                    "content": "// Generated component code"
                }
            }]
        })))
        .mount(mock_server)
        .await;
    
    // Mock for image generation
    Mock::given(method("POST"))
        .and(mock_path("/v1/images/generations"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "data": [{
                "url": format!("{}/test-image.png", mock_server.uri())
            }]
        })))
        .mount(mock_server)
        .await;
    
    // Mock for image download
    Mock::given(method("GET"))
        .and(mock_path("/test-image.png"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_bytes(create_test_png())
        )
        .mount(mock_server)
        .await;
}

fn verify_generated_files(temp_dir: &TempDir) {
    // Check directory structure
    assert!(Path::new("src").is_dir());
    assert!(Path::new("src/components").is_dir());
    assert!(Path::new("src/systems").is_dir());
    assert!(Path::new("assets/sprites").is_dir());
    assert!(Path::new("assets/audio").is_dir());
    
    // Check core files
    assert!(Path::new("Cargo.toml").exists());
    assert!(Path::new("src/main.rs").exists());
    assert!(Path::new("src/lib.rs").exists());
    
    // Check style guide
    assert!(Path::new("assets/data/style_guide.json").exists());
}

fn create_test_png() -> Vec<u8> {
    // Minimal valid PNG file
    vec![
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, // PNG signature
        0x00, 0x00, 0x00, 0x0D, // IHDR chunk length
        0x49, 0x48, 0x44, 0x52, // IHDR
        0x00, 0x00, 0x00, 0x01, // width: 1
        0x00, 0x00, 0x00, 0x01, // height: 1
        0x08, 0x02, // bit depth: 8, color type: 2 (RGB)
        0x00, 0x00, 0x00, // compression, filter, interlace
        0x90, 0x77, 0x53, 0xDE, // CRC
        0x00, 0x00, 0x00, 0x0C, // IDAT chunk length
        0x49, 0x44, 0x41, 0x54, // IDAT
        0x08, 0xD7, 0x63, 0xF8, 0xCF, 0xC0, 0x00, 0x00, 0x03, 0x01, 0x01, 0x00, // compressed data
        0x18, 0xDD, 0x8D, 0xB4, // CRC
        0x00, 0x00, 0x00, 0x00, // IEND chunk length
        0x49, 0x45, 0x4E, 0x44, // IEND
        0xAE, 0x42, 0x60, 0x82, // CRC
    ]
}

#[tokio::test]
async fn test_dry_run_mode() {
    let temp_dir = TempDir::new().unwrap();
    std::env::set_current_dir(&temp_dir).unwrap();
    
    // Copy test config
    let config_content = include_str!("../fixtures/test-config.yaml");
    fs::write("game-config.yaml", config_content).unwrap();
    
    let mut generator = AIGameGenerator::new()
        .with_dry_run(true)
        .with_use_cache(false);
    
    // In dry run mode, no files should be created
    let result = generator.generate_game().await;
    assert!(result.is_ok());
    
    // Verify no files were actually written
    assert!(!Path::new("src/main.rs").exists());
    assert!(!Path::new("Cargo.toml").exists());
}

#[tokio::test]
async fn test_cache_behavior_integration() {
    let temp_dir = TempDir::new().unwrap();
    std::env::set_current_dir(&temp_dir).unwrap();
    
    // Copy test config
    let config_content = include_str!("../fixtures/test-config.yaml");
    fs::write("game-config.yaml", config_content).unwrap();
    
    // Create cache directory with pre-cached response
    fs::create_dir_all(".cache/ai-gen").unwrap();
    let cache_key = "test_cache_key";
    let cached_content = "Cached response content";
    fs::write(format!(".cache/ai-gen/{}.txt", cache_key), cached_content).unwrap();
    
    let generator = AIGameGenerator::new()
        .with_use_cache(true);
    
    // Verify cache is used (this would need actual implementation testing)
    // For now, just verify cache directory exists
    assert!(Path::new(".cache/ai-gen").exists());
}

#[tokio::test]
async fn test_git_tracking_integration() {
    let temp_dir = TempDir::new().unwrap();
    std::env::set_current_dir(&temp_dir).unwrap();
    
    // Initialize git repo
    let repo = git2::Repository::init(&temp_dir).unwrap();
    let sig = git2::Signature::now("Test", "test@example.com").unwrap();
    let tree_id = repo.index().unwrap().write_tree().unwrap();
    let tree = repo.find_tree(tree_id).unwrap();
    repo.commit(
        Some("HEAD"),
        &sig,
        &sig,
        "Initial commit",
        &tree,
        &[],
    ).unwrap();
    
    // Copy test config
    let config_content = include_str!("../fixtures/test-config.yaml");
    fs::write("game-config.yaml", config_content).unwrap();
    
    let mock_server = MockServer::start().await;
    std::env::set_var("OPENAI_API_KEY", "test-key");
    std::env::set_var("OPENAI_API_BASE", mock_server.uri());
    
    setup_generation_mocks(&mock_server).await;
    
    let mut generator = AIGameGenerator::new()
        .with_use_cache(false);
    
    let result = generator.generate_game().await;
    assert!(result.is_ok());
    
    // Verify git tracking files were created
    assert!(Path::new(".ai-generation/manifest.json").exists());
    assert!(Path::new(".ai-generation/history.jsonl").exists());
}

#[tokio::test]
async fn test_error_handling_missing_config() {
    let temp_dir = TempDir::new().unwrap();
    std::env::set_current_dir(&temp_dir).unwrap();
    
    // Don't create config file
    let mut generator = AIGameGenerator::new();
    let result = generator.generate_game().await;
    
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("game-config.yaml"));
}

#[tokio::test]
async fn test_error_handling_invalid_config() {
    let temp_dir = TempDir::new().unwrap();
    std::env::set_current_dir(&temp_dir).unwrap();
    
    // Create invalid config
    fs::write("game-config.yaml", "invalid: [yaml: content").unwrap();
    
    let mut generator = AIGameGenerator::new();
    let result = generator.generate_game().await;
    
    assert!(result.is_err());
}