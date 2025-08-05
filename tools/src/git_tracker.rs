use anyhow::{Result, Context};
use git2::{Repository, Signature, Oid, Tree, Commit, DiffOptions, IndexAddOption};
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

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerationManifest {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub parent_commit: Option<String>,
    pub cascade_tree: CascadeTree,
    pub generated_files: HashMap<PathBuf, FileMetadata>,
    pub cache_keys: HashMap<String, String>,
    pub config_snapshot: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CascadeTree {
    pub root_prompt: PromptNode,
    pub total_api_calls: u32,
    pub total_cost_estimate: f32,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct FileMetadata {
    pub hash: String,
    pub size: u64,
    pub prompt_hash: String,
    pub generation_time: DateTime<Utc>,
    pub parent_assets: Vec<String>,
}

impl GitGenerationTracker {
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
        // Navigate to the parent node
        let mut current = &mut manifest.cascade_tree.root_prompt;
        for segment in parent_path {
            current = current.children
                .iter_mut()
                .find(|child| child.id == segment)
                .context("Parent node not found in cascade tree")?;
        }
        
        // Add the new prompt node
        current.children.push(prompt);
        
        // Update statistics
        if !prompt.cache_hit {
            manifest.cascade_tree.total_api_calls += 1;
            // Estimate cost based on prompt length
            let tokens = (prompt.system_prompt.len() + prompt.user_prompt.len()) / 4;
            manifest.cascade_tree.total_cost_estimate += (tokens as f32) * 0.00001; // Rough estimate
        }
        
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
        let hash = format!("{:x}", md5::compute(content));
        
        let metadata = FileMetadata {
            hash: hash.clone(),
            size: content.len() as u64,
            prompt_hash,
            generation_time: Utc::now(),
            parent_assets,
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
            let current_hash = format!("{:x}", md5::compute(&current_content));
            
            if current_hash != metadata.hash {
                return Ok(true);
            }
            
            // Check parent dependencies if requested
            if check_parents {
                for parent in &metadata.parent_assets {
                    if let Some(parent_path) = manifest.cache_keys.iter()
                        .find(|(_, hash)| *hash == parent)
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
            
            // Check if all files still exist with correct content
            for (path, metadata) in &manifest.generated_files {
                if path.exists() {
                    let content = std::fs::read(path)?;
                    let hash = format!("{:x}", md5::compute(&content));
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