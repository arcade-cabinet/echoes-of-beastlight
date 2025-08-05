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

use anyhow::{Result, Context};
use git2::{Repository, Signature, Oid, Commit};
use serde::{Serialize, Deserialize};
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Git-based generation tracker for perfect idempotency and versioning
pub struct GitGenerationTracker {
    repo: Repository,
    generation_branch: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationManifest {
    pub id: Uuid,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub timestamp: DateTime<Utc>,
    pub parent_commit: Option<String>,
    pub cascade_tree: CascadeTree,
    pub generated_files: HashMap<PathBuf, FileMetadata>,
    pub cache_keys: HashMap<String, String>,
    pub config_snapshot: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CascadeTree {
    pub root_prompt: PromptNode,
    pub total_api_calls: u32,
    pub total_cost_estimate: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptNode {
    pub id: String,
    pub prompt_type: String,
    pub prompt_hash: String,
    pub system_prompt: String,
    pub user_prompt: String,
    pub children: Vec<PromptNode>,
    pub generated_files: Vec<PathBuf>,
    pub cache_hit: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetadata {
    pub hash: String,
    pub size: u64,
    pub prompt_hash: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub generation_time: DateTime<Utc>,
    pub parent_assets: Vec<PathBuf>,
}

impl GitGenerationTracker {
    /// Calculate hash of file content
    fn calculate_file_hash(content: &[u8]) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        content.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }

    /// Estimate API cost based on token counts
    fn estimate_cost(prompt_tokens: usize, completion_tokens: usize) -> f32 {
        // GPT-4 Turbo pricing (approximate)
        let prompt_cost = prompt_tokens as f32 * 0.01 / 1000.0;
        let completion_cost = completion_tokens as f32 * 0.03 / 1000.0;
        prompt_cost + completion_cost
    }

    /// Initialize tracker with the current repository
    pub fn new(repo_path: impl AsRef<Path>) -> Result<Self> {
        let repo = Repository::open(repo_path)?;
        Ok(Self {
            repo,
            generation_branch: "ai-generation-tracking".to_string(),
        })
    }

    /// Start a new generation session
    pub fn start_generation(&self, config: &serde_json::Value) -> Result<GenerationManifest> {
        let manifest = GenerationManifest {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            parent_commit: self.get_current_commit()?,
            cascade_tree: CascadeTree {
                root_prompt: PromptNode {
                    id: "root".to_string(),
                    prompt_type: "root".to_string(),
                    prompt_hash: "".to_string(),
                    system_prompt: "".to_string(),
                    user_prompt: "".to_string(),
                    children: vec![],
                    generated_files: vec![],
                    cache_hit: false,
                },
                total_api_calls: 0,
                total_cost_estimate: 0.0,
            },
            generated_files: HashMap::new(),
            cache_keys: HashMap::new(),
            config_snapshot: config.clone(),
        };

        // Create or switch to generation tracking branch
        self.ensure_tracking_branch()?;

        Ok(manifest)
    }

    /// Track a prompt in the cascade tree
    pub fn track_prompt(
        &self,
        manifest: &mut GenerationManifest,
        parent_path: Vec<String>,
        prompt: PromptNode,
    ) -> Result<()> {
        let mut current = &mut manifest.cascade_tree.root_prompt;

        // Navigate to parent
        for segment in parent_path {
            current = current.children.iter_mut()
                .find(|child| child.id == segment)
                .context("Parent path not found in cascade tree")?;
        }

        // Calculate cost
        let prompt_tokens = prompt.system_prompt.len() + prompt.user_prompt.len();
        let estimated_tokens = prompt_tokens / 4; // Rough estimate
        let completion_tokens = 500; // Rough estimate
        let cost = Self::estimate_cost(estimated_tokens, completion_tokens);

        manifest.cascade_tree.total_api_calls += 1;
        manifest.cascade_tree.total_cost_estimate += cost;

        current.children.push(prompt);

        Ok(())
    }

    /// Track a generated file
    pub fn track_file(
        &self,
        manifest: &mut GenerationManifest,
        path: PathBuf,
        content: &[u8],
        prompt_hash: String,
        parent_assets: Vec<String>,
    ) -> Result<()> {
        let hash = Self::calculate_file_hash(content);

        let metadata = FileMetadata {
            hash: hash.clone(),
            size: content.len() as u64,
            prompt_hash,
            generation_time: Utc::now(),
            parent_assets: parent_assets.into_iter().map(PathBuf::from).collect(),
        };

        manifest.generated_files.insert(path.clone(), metadata);
        manifest.cache_keys.insert(path.to_string_lossy().to_string(), hash);

        Ok(())
    }

    /// Check if a file needs regeneration
    pub fn needs_regeneration(
        &self,
        manifest: &GenerationManifest,
        path: &Path,
        check_parents: bool,
    ) -> Result<bool> {
        // Check if file exists in current manifest
        if let Some(metadata) = manifest.generated_files.get(path) {
            // Check if file exists on disk
            if !path.exists() {
                return Ok(true);
            }

            // Check if content matches
            let current_content = std::fs::read(path)?;
                            let current_hash = Self::calculate_file_hash(&current_content);

            if current_hash != metadata.hash {
                return Ok(true);
            }

            // Check parent dependencies if requested
            if check_parents {
                for parent in &metadata.parent_assets {
                    if let Some(parent_path) = manifest.cache_keys.iter()
                        .find(|(_, hash)| **hash == parent.to_string_lossy().to_string())
                        .map(|(path, _)| PathBuf::from(path))
                    {
                        if self.needs_regeneration(manifest, &parent_path, false)? {
                            return Ok(true);
                        }
                    }
                }
            }

            Ok(false)
        } else {
            // File not tracked, needs generation
            Ok(true)
        }
    }

    /// Commit the current generation state
    pub fn commit_generation(
        &self,
        manifest: &GenerationManifest,
        message: &str,
    ) -> Result<Oid> {
        // Stage all generated files
        let mut index = self.repo.index()?;

        for (path, _) in &manifest.generated_files {
            index.add_path(path)?;
        }

        // Write manifest file
        let manifest_path = PathBuf::from(".ai-generation/manifest.json");
        std::fs::create_dir_all(".ai-generation")?;
        let manifest_json = serde_json::to_string_pretty(manifest)?;
        std::fs::write(&manifest_path, manifest_json)?;
        index.add_path(&manifest_path)?;

        // Write cascade visualization
        let cascade_path = PathBuf::from(".ai-generation/cascade.md");
        let cascade_md = self.visualize_cascade(&manifest.cascade_tree)?;
        std::fs::write(&cascade_path, cascade_md)?;
        index.add_path(&cascade_path)?;

        index.write()?;

        // Create commit
        let sig = Signature::now("AI Generator", "ai@echoes-of-beastlight.com")?;
        let tree_id = index.write_tree()?;
        let tree = self.repo.find_tree(tree_id)?;

        let parent_commit = self.get_head_commit()?;
        let commit_id = self.repo.commit(
            Some("HEAD"),
            &sig,
            &sig,
            &format!("{}\n\nGeneration ID: {}", message, manifest.id),
            &tree,
            &[&parent_commit],
        )?;

        Ok(commit_id)
    }

    /// Get diff of what will be generated
    pub fn preview_changes(&self, manifest: &GenerationManifest) -> Result<String> {
        let mut preview = String::new();

        preview.push_str("# Generation Preview\n\n");
        preview.push_str(&format!("Generation ID: {}\n", manifest.id));
        preview.push_str(&format!("Timestamp: {}\n", manifest.timestamp));
        preview.push_str(&format!("Total API Calls: {}\n", manifest.cascade_tree.total_api_calls));
        preview.push_str(&format!("Estimated Cost: ${:.4}\n\n", manifest.cascade_tree.total_cost_estimate));

        preview.push_str("## Files to Generate:\n");
        for (path, metadata) in &manifest.generated_files {
            let status = if path.exists() { "update" } else { "create" };
            preview.push_str(&format!("- {} {} ({})\n", status, path.display(), metadata.size));
        }

        preview.push_str("\n## Cascade Tree:\n");
        preview.push_str(&self.visualize_cascade(&manifest.cascade_tree)?);

        Ok(preview)
    }

    /// Load previous generation manifest
    pub fn load_manifest(&self, generation_id: Option<Uuid>) -> Result<Option<GenerationManifest>> {
        let manifest_path = PathBuf::from(".ai-generation/manifest.json");

        if let Some(id) = generation_id {
            // Load specific generation from git history
            let output = std::process::Command::new("git")
                .args(&["show", &format!("HEAD:.ai-generation/manifest.json")])
                .output()?;

            if output.status.success() {
                let content = String::from_utf8(output.stdout)?;
                let manifest: GenerationManifest = serde_json::from_str(&content)?;
                if manifest.id == id {
                    return Ok(Some(manifest));
                }
            }
        } else if manifest_path.exists() {
            // Load current manifest
            let content = std::fs::read_to_string(manifest_path)?;
            let manifest: GenerationManifest = serde_json::from_str(&content)?;
            return Ok(Some(manifest));
        }

        Ok(None)
    }

    /// Save manifest to the repository
    pub fn save_manifest(&self, manifest: &GenerationManifest) -> Result<()> {
        let workdir = self.repo.workdir().context("No workdir")?;
        let manifest_path = workdir.join(".ai-generation/manifest.json");

        std::fs::create_dir_all(manifest_path.parent().unwrap())?;
        std::fs::write(&manifest_path, serde_json::to_string_pretty(manifest)?)?;

        Ok(())
    }

    /// Get generation history
    pub fn get_history(&self, limit: usize) -> Result<Vec<GenerationSummary>> {
        let mut history = Vec::new();

        let mut revwalk = self.repo.revwalk()?;
        revwalk.push_head()?;

        for oid in revwalk.take(limit) {
            let oid = oid?;
            let commit = self.repo.find_commit(oid)?;

            // Check if this is a generation commit
            if commit.message().unwrap_or("").contains("Generation ID:") {
                let tree = commit.tree()?;

                // Try to read manifest from this commit
                if let Ok(entry) = tree.get_path(Path::new(".ai-generation/manifest.json")) {
                    if let Ok(blob) = self.repo.find_blob(entry.id()) {
                        if let Ok(manifest) = serde_json::from_slice::<GenerationManifest>(blob.content()) {
                            history.push(GenerationSummary {
                                id: manifest.id,
                                timestamp: manifest.timestamp,
                                commit_id: oid.to_string(),
                                files_generated: manifest.generated_files.len(),
                                api_calls: manifest.cascade_tree.total_api_calls,
                            });
                        }
                    }
                }
            }
        }

        Ok(history)
    }

    // Helper methods

    fn ensure_tracking_branch(&self) -> Result<()> {
        // Check if branch exists
        match self.repo.find_branch(&self.generation_branch, git2::BranchType::Local) {
            Ok(_) => {
                // Branch exists, check it out
                self.repo.set_head(&format!("refs/heads/{}", self.generation_branch))?;
            }
            Err(_) => {
                // Create new branch
                let head_commit = self.get_head_commit()?;
                self.repo.branch(&self.generation_branch, &head_commit, false)?;
                self.repo.set_head(&format!("refs/heads/{}", self.generation_branch))?;
            }
        }

        Ok(())
    }

    fn get_current_commit(&self) -> Result<Option<String>> {
        match self.repo.head() {
            Ok(head) => Ok(head.target().map(|oid| oid.to_string())),
            Err(_) => Ok(None),
        }
    }

    fn get_head_commit(&self) -> Result<Commit> {
        let head = self.repo.head()?;
        let oid = head.target().context("HEAD has no target")?;
        Ok(self.repo.find_commit(oid)?)
    }

    fn visualize_cascade(&self, tree: &CascadeTree) -> Result<String> {
        let mut output = String::new();
        self.visualize_node(&tree.root_prompt, &mut output, 0)?;
        Ok(output)
    }

    fn visualize_node(&self, node: &PromptNode, output: &mut String, depth: usize) -> Result<()> {
        let indent = "  ".repeat(depth);
        let cache_marker = if node.cache_hit { " (cached)" } else { "" };

        output.push_str(&format!(
            "{}- {} [{}]{}\n",
            indent,
            node.prompt_type,
            node.id,
            cache_marker
        ));

        if !node.generated_files.is_empty() {
            for file in &node.generated_files {
                output.push_str(&format!("{}  → {}\n", indent, file.display()));
            }
        }

        for child in &node.children {
            self.visualize_node(child, output, depth + 1)?;
        }

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerationSummary {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub commit_id: String,
    pub files_generated: usize,
    pub api_calls: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerationHistory {
    pub id: Uuid,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub timestamp: DateTime<Utc>,
    pub commit_hash: String,
    pub files_generated: usize,
    pub cascade_depth: usize,
}

/// Integration with the main generator
impl GitGenerationTracker {
    /// Check if we can skip generation entirely
    pub fn can_skip_generation(&self, config: &serde_json::Value) -> Result<bool> {
        // Load the last manifest
        if let Some(manifest) = self.load_manifest(None)? {
            // Check if config has changed
            if manifest.config_snapshot != *config {
                return Ok(false);
            }

            // Get the repository workdir
            let workdir = self.repo.workdir().context("No workdir")?;

            // Check if all files still exist with correct content
            for (path, metadata) in &manifest.generated_files {
                let full_path = workdir.join(path);
                if full_path.exists() {
                    let content = std::fs::read(&full_path)?;
                    let hash = Self::calculate_file_hash(&content);
                    if hash != metadata.hash {
                        return Ok(false);
                    }
                } else {
                    return Ok(false);
                }
            }

            // Everything matches, we can skip
            return Ok(true);
        }

        Ok(false)
    }

    /// Get files that need regeneration
    pub fn get_stale_files(&self, manifest: &GenerationManifest) -> Result<Vec<PathBuf>> {
        let mut stale = Vec::new();

        for (path, _) in &manifest.generated_files {
            if self.needs_regeneration(manifest, path, true)? {
                stale.push(path.clone());
            }
        }

        Ok(stale)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs;

    fn setup_test_repo() -> Result<TempDir> {
        let temp_dir = TempDir::new()?;
        let repo = Repository::init(&temp_dir)?;

        // Create initial commit
        let sig = Signature::now("Test User", "test@example.com")?;
        let tree_id = {
            let mut index = repo.index()?;
            index.write_tree()?
        };
        let tree = repo.find_tree(tree_id)?;
        repo.commit(
            Some("HEAD"),
            &sig,
            &sig,
            "Initial commit",
            &tree,
            &[],
        )?;

        Ok(temp_dir)
    }

    #[test]
    fn test_new_tracker() {
        let temp_dir = setup_test_repo().unwrap();
        let tracker = GitGenerationTracker::new(&temp_dir).unwrap();
        assert_eq!(tracker.generation_branch, "ai-generation-tracking");
    }

    #[test]
    fn test_start_generation() {
        let temp_dir = setup_test_repo().unwrap();
        let tracker = GitGenerationTracker::new(&temp_dir).unwrap();

        let config = serde_json::json!({
            "game": {
                "title": "Test Game"
            }
        });

        let manifest = tracker.start_generation(&config).unwrap();

        assert!(manifest.parent_commit.is_some());
        assert_eq!(manifest.cascade_tree.root_prompt.id, "root");
        assert_eq!(manifest.cascade_tree.total_api_calls, 0);
        assert_eq!(manifest.cascade_tree.total_cost_estimate, 0.0);
        assert_eq!(manifest.config_snapshot, config);
    }

    #[test]
    fn test_track_prompt() {
        let temp_dir = setup_test_repo().unwrap();
        let tracker = GitGenerationTracker::new(&temp_dir).unwrap();

        let config = serde_json::json!({});
        let mut manifest = tracker.start_generation(&config).unwrap();

        let prompt_node = PromptNode {
            id: "test-prompt".to_string(),
            prompt_type: "component".to_string(),
            prompt_hash: "abc123".to_string(),
            system_prompt: "You are a game developer".to_string(),
            user_prompt: "Create a player component".to_string(),
            children: vec![],
            generated_files: vec![],
            cache_hit: false,
        };

        tracker.track_prompt(&mut manifest, vec![], prompt_node).unwrap();

        assert_eq!(manifest.cascade_tree.root_prompt.children.len(), 1);
        assert_eq!(manifest.cascade_tree.root_prompt.children[0].id, "test-prompt");
        assert_eq!(manifest.cascade_tree.total_api_calls, 1);
        assert!(manifest.cascade_tree.total_cost_estimate > 0.0);
    }

    #[test]
    fn test_track_file() {
        let temp_dir = setup_test_repo().unwrap();
        let tracker = GitGenerationTracker::new(&temp_dir).unwrap();

        let mut manifest = GenerationManifest {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            parent_commit: Some("abc123".to_string()),
            cascade_tree: CascadeTree {
                root_prompt: PromptNode {
                    id: "root".to_string(),
                    prompt_type: "root".to_string(),
                    prompt_hash: "".to_string(),
                    system_prompt: "".to_string(),
                    user_prompt: "".to_string(),
                    children: vec![],
                    generated_files: vec![],
                    cache_hit: false,
                },
                total_api_calls: 5,
                total_cost_estimate: 0.25,
            },
            generated_files: HashMap::new(),
            cache_keys: HashMap::new(),
            config_snapshot: serde_json::Value::Null,
        };

        let file_path = PathBuf::from("test.txt");
        let content = b"test content";

        tracker.track_file(&mut manifest, file_path.clone(), content, "prompt123".to_string(), vec![]).unwrap();

        assert!(manifest.generated_files.contains_key(&file_path));
        let metadata = &manifest.generated_files[&file_path];
        assert_eq!(metadata.size, 12);
        assert_eq!(metadata.prompt_hash, "prompt123");
        assert!(!metadata.hash.is_empty());
    }

    #[test]
    fn test_can_skip_generation() {
        let temp_dir = setup_test_repo().unwrap();

        // Change to the temp directory so relative paths work
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(&temp_dir).unwrap();

        let tracker = GitGenerationTracker::new(".").unwrap();

        // First generation
        let config = serde_json::json!({
            "game": { "title": "Test" }
        });
        let mut manifest = tracker.start_generation(&config).unwrap();

        // Track a file - write it in the current directory
        let file_path = PathBuf::from("test.txt");
        fs::write(&file_path, "content").unwrap();
        tracker.track_file(&mut manifest, file_path, b"content", "hash1".to_string(), vec![]).unwrap();

        // Save manifest using the tracker's save method
        tracker.save_manifest(&manifest).unwrap();

        // Check if we can skip
        assert!(tracker.can_skip_generation(&config).unwrap());

        // Change config
        let new_config = serde_json::json!({
            "game": { "title": "Different" }
        });
        assert!(!tracker.can_skip_generation(&new_config).unwrap());

        // Restore original directory
        std::env::set_current_dir(original_dir).unwrap();
    }

    #[test]
    fn test_prompt_node_serialization() {
        let node = PromptNode {
            id: "test".to_string(),
            prompt_type: "system".to_string(),
            prompt_hash: "hash123".to_string(),
            system_prompt: "System".to_string(),
            user_prompt: "User".to_string(),
            children: vec![],
            generated_files: vec![PathBuf::from("file.txt")],
            cache_hit: true,
        };

        let json = serde_json::to_string(&node).unwrap();
        let deserialized: PromptNode = serde_json::from_str(&json).unwrap();

        assert_eq!(node.id, deserialized.id);
        assert_eq!(node.prompt_type, deserialized.prompt_type);
        assert_eq!(node.cache_hit, deserialized.cache_hit);
        assert_eq!(node.generated_files, deserialized.generated_files);
    }

    #[test]
    fn test_generation_manifest_serialization() {
        let manifest = GenerationManifest {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            parent_commit: Some("abc123".to_string()),
            cascade_tree: CascadeTree {
                root_prompt: PromptNode {
                    id: "root".to_string(),
                    prompt_type: "root".to_string(),
                    prompt_hash: "".to_string(),
                    system_prompt: "".to_string(),
                    user_prompt: "".to_string(),
                    children: vec![],
                    generated_files: vec![],
                    cache_hit: false,
                },
                total_api_calls: 5,
                total_cost_estimate: 0.25,
            },
            generated_files: HashMap::new(),
            cache_keys: HashMap::new(),
            config_snapshot: serde_json::json!({"test": true}),
        };

        let json = serde_json::to_string(&manifest).unwrap();
        let deserialized: GenerationManifest = serde_json::from_str(&json).unwrap();

        assert_eq!(manifest.id, deserialized.id);
        assert_eq!(manifest.parent_commit, deserialized.parent_commit);
        assert_eq!(manifest.cascade_tree.total_api_calls, deserialized.cascade_tree.total_api_calls);
        assert_eq!(manifest.config_snapshot, deserialized.config_snapshot);
    }

    #[test]
    fn test_calculate_file_hash() {
        let content1 = b"Hello, world!";
        let content2 = b"Hello, world!";
        let content3 = b"Different content";

        let hash1 = GitGenerationTracker::calculate_file_hash(content1);
        let hash2 = GitGenerationTracker::calculate_file_hash(content2);
        let hash3 = GitGenerationTracker::calculate_file_hash(content3);

        assert_eq!(hash1, hash2); // Same content = same hash
        assert_ne!(hash1, hash3); // Different content = different hash
    }

    #[test]
    fn test_estimate_cost() {
        // Test with known token counts
        let cost1 = GitGenerationTracker::estimate_cost(100, 50);
        let cost2 = GitGenerationTracker::estimate_cost(1000, 500);

        assert!(cost1 > 0.0);
        assert!(cost2 > cost1); // More tokens = higher cost
        assert!(cost2 < 1.0); // Reasonable upper bound for test
    }
}
