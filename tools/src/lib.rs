pub mod config;
pub mod generator;
pub mod templates;
pub mod git_tracker;

// Re-export commonly used types
pub use config::GameConfig;
pub use generator::AIGameGenerator;
pub use templates::Templates;