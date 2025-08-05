# Idempotency in AI Game Generation

## Overview

The AI Game Generator implements multiple layers of idempotency to ensure that running the generator multiple times with the same inputs produces consistent results and doesn't unnecessarily regenerate assets.

## Idempotency Mechanisms

### 1. Content-Based Cache Keys

Every AI generation request is converted into a deterministic cache key based on:
- The system prompt
- The user prompt
- Model parameters (temperature, max_tokens)
- Any additional context

```rust
let cache_key = format!("{}:{}:{}",
    md5::compute(&system_prompt),
    md5::compute(&user_prompt),
    md5::compute(&params)
);
```

### 2. Multi-Level Caching

#### Memory Cache (LRU)

- Fast in-memory cache for frequently accessed assets
- Limited to 100 most recent items
- Instant retrieval for hot assets

#### Disk Cache (Compressed)

- Persistent storage using Zstd compression
- Survives application restarts
- Organized by content hash

```
.cache/
├── ai-gen/
│   ├── a1b2c3d4e5f6.zst      # Compressed asset data
│   ├── a1b2c3d4e5f6.meta     # Asset metadata
│   └── ...
└── style-transfer/
    ├── style-embeddings.json
    └── ...
```

### 3. File Tracking System

#### Generated Files Manifest

Every generation run produces a `GENERATION_SUMMARY.json`:

```json
{
  "game": {
    "title": "Echoes of Beastlight",
    "version": "0.1.0"
  },
  "generated": {
    "files": [
      "src/main.rs",
      "src/components.rs",
      "assets/sprites/hero.png",
      ...
    ],
    "timestamp": "2024-01-15T10:30:00Z"
  },
  "cache_keys": {
    "src/main.rs": "a1b2c3d4e5f6",
    "assets/sprites/hero.png": "b2c3d4e5f6g7"
  }
}
```

#### File State Tracking

The generator maintains a `HashSet<PathBuf>` of all files written during the current session:

```rust
pub struct AIGameGenerator {
    generated_files: HashSet<PathBuf>,
    // ...
}
```

### 4. Smart Regeneration

The system only regenerates when:
1. **No cache hit**: The exact prompt combination hasn't been seen before
2. **Force flag**: User explicitly requests regeneration
3. **Dependency change**: A parent asset in the dependency graph has changed

### 5. Dependency Graph

Assets are organized in a directed acyclic graph (DAG) to track dependencies:

```rust
pub struct AssetDependencyGraph {
    graph: DiGraph<AssetNode, AssetRelation>,
    node_map: HashMap<String, NodeIndex>,
}
```

Example dependency relationships:
- Style Guide → All visual assets
- Character Design → Character animations
- Tileset → Level layouts

When a parent asset changes, all dependent assets are marked for regeneration.

### 6. Content Hashing

Each generated asset gets a content hash that includes:
- The actual asset data
- Generation parameters
- Parent asset hashes (if any)

This creates a Merkle tree-like structure where changing any asset invalidates its descendants.

### 7. Metadata Tracking

Every cached asset includes metadata:

```rust
#[derive(Serialize, Deserialize)]
pub struct AssetMetadata {
    pub asset_type: String,
    pub dimensions: Option<(u32, u32)>,
    pub format: String,
    pub generation_params: HashMap<String, String>,
    pub quality_score: f32,
    pub style_consistency: f32,
    pub parent_hash: Option<String>,
    pub generation_timestamp: i64,
}
```

## Idempotency Workflow

1. **Request Received**

   ```
   User requests: Generate character sprite for "Fire Mage"
   ```

2. **Cache Key Generation**

   ```
   Key = hash("character" + "Fire Mage" + style_params)
   Key = "a1b2c3d4e5f6g7h8"
   ```

3. **Cache Lookup**

   ```rust
   if let Some((data, metadata)) = cache.get_asset(&key).await? {
       // Return cached result
       return Ok(data);
   }
   ```

4. **Generation (if needed)**
   - Call OpenAI API
   - Apply style transfer
   - Process to pixel art
   - Save to cache with metadata

5. **File Writing**

   ```rust
   async fn write_file(&mut self, path: P, content: &[u8]) -> Result<()> {
       fs::write(path, content)?;
       self.generated_files.insert(path.to_path_buf());
       self.cache_manifest.insert(path, cache_key);
   }
   ```

## Cascade Idempotency

For cascading generations (where one prompt generates multiple sub-prompts):

1. **Parent Prompt Hash**: The parent prompt is hashed and stored
2. **Child Prompt Generation**: Child prompts include parent hash
3. **Cascade Manifest**: Tracks the entire cascade tree

```json
{
  "cascade_id": "level_design_cascade_123",
  "parent": {
    "prompt": "Generate level for Verdant Flats",
    "hash": "parent123"
  },
  "children": [
    {
      "type": "layout",
      "prompt_hash": "child_layout_456",
      "generated_file": "levels/verdant_flats.yol"
    },
    {
      "type": "entities",
      "prompt_hash": "child_entities_789",
      "generated_file": "levels/verdant_flats_entities.yol"
    }
  ]
}
```

## Benefits

1. **Cost Efficiency**: Avoid redundant OpenAI API calls
2. **Consistency**: Same inputs always produce same outputs
3. **Speed**: Cached results return instantly
4. **Versioning**: Track changes over time
5. **Rollback**: Restore previous generations if needed

## Future Enhancements

1. **Distributed Cache**: Share cache across team members
2. **Cache Invalidation Rules**: Smart expiry based on usage patterns
3. **Differential Generation**: Only regenerate changed parts
4. **Blockchain Verification**: Cryptographic proof of generation history
5. **ML-Based Cache Prediction**: Pregenerate likely requests

## Git-Based Generation Tracking (Implemented)

Since we're already in a git repository, we leverage git2 for sophisticated tracking:

### Generation Manifest in Git

Every generation creates a comprehensive manifest stored in `.ai-generation/manifest.json`:

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "timestamp": "2024-01-15T10:30:00Z",
  "parent_commit": "abc123...",
  "cascade_tree": {
    "root_prompt": {
      "id": "root",
      "prompt_type": "root",
      "children": [
        {
          "id": "level_gen_1",
          "prompt_type": "level_layout",
          "system_prompt": "...",
          "user_prompt": "...",
          "generated_files": ["levels/zone1.yol"],
          "cache_hit": false,
          "children": [...]
        }
      ]
    },
    "total_api_calls": 15,
    "total_cost_estimate": 0.0234
  },
  "generated_files": {
    "src/main.rs": {
      "hash": "a1b2c3...",
      "size": 2048,
      "prompt_hash": "def456...",
      "generation_time": "2024-01-15T10:31:00Z",
      "parent_assets": []
    }
  }
}
```

### Cascade Visualization

Every generation also creates `.ai-generation/cascade.md` showing the prompt tree:

```markdown
- root [root]
  - level_layout [level_gen_1]
    → levels/zone1.yol
    - level_entities [entities_1] (cached)
      → levels/zone1_entities.yol
    - level_visuals [visuals_1]
      → assets/zone1_theme.json
```

### Git Integration Features

1. **Automatic Commits**: Each generation creates a commit with all generated files
2. **Branch Tracking**: Generations happen on `ai-generation-tracking` branch
3. **History Browsing**: View past generations with `git log`
4. **Diff Preview**: See what will change before generation
5. **Rollback**: Revert to any previous generation state
6. **Cross-Session Tracking**: Manifest persists across runs

### Usage

```bash
# Normal generation (checks for changes)
cargo run -p ai-game-generator generate

# Force regeneration
cargo run -p ai-game-generator generate --force

# View generation history
git log --oneline | grep "Generation ID"

# Revert to previous generation
git checkout <commit-id>
```

### Benefits of Git Integration

1. **Perfect Versioning**: Every generation is a git commit
2. **Collaboration**: Team members can share generations
3. **Audit Trail**: Complete history of all AI interactions
4. **Branching**: Experiment with different generation strategies
5. **Merge Conflicts**: Git handles concurrent generations
6. **CI/CD Integration**: Trigger builds on generation commits
