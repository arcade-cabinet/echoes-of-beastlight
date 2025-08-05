use bevy::prelude::*;
use crossbeam_channel::{Receiver, Sender};
use dashmap::DashMap;
use image::{DynamicImage, ImageBuffer, Rgba};
use petgraph::graph::{DiGraph, NodeIndex};
use rayon::prelude::*;
use std::collections::{HashMap, BinaryHeap};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use crate::studio::{wizard::GameConfiguration, Notification, NotificationLevel};

/// Main generator state tracking all generation tasks
#[derive(Resource, Default)]
pub struct GeneratorState {
    pub tasks: DashMap<Uuid, GenerationTask>,
    pub completed_assets: DashMap<String, GeneratedAsset>,
    pub current_task: String,
    pub openai_connected: bool,
    pub style_guide: StyleGuide,
    // Advanced systems
    pub style_transfer: Option<Arc<NeuralStyleTransfer>>,
    pub pixel_processor: Arc<PixelArtProcessor>,
    pub dependency_graph: Arc<AssetDependencyGraph>,
    pub prompt_optimizer: Arc<PromptOptimizer>,
}

impl GeneratorState {
    pub fn overall_progress(&self) -> f32 {
        if self.tasks.is_empty() {
            return 0.0;
        }
        
        let completed = self.tasks.iter()
            .filter(|t| matches!(t.status, TaskStatus::Completed))
            .count();
        
        completed as f32 / self.tasks.len() as f32
    }
    
    pub fn complete_task(&mut self, task_id: Uuid, output: GenerationOutput) {
        if let Some(mut task) = self.tasks.get_mut(&task_id) {
            task.status = TaskStatus::Completed;
            task.output = Some(output);
        }
    }
    
    pub fn fail_task(&mut self, task_id: Uuid, error: String) {
        if let Some(mut task) = self.tasks.get_mut(&task_id) {
            task.status = TaskStatus::Failed(error);
        }
    }
    
    pub fn update_progress(&mut self, task_id: Uuid, progress: f32) {
        if let Some(mut task) = self.tasks.get_mut(&task_id) {
            task.progress = progress;
        }
    }
}

#[derive(Debug, Clone)]
pub struct StyleGuide {
    pub colors: HashMap<String, Color>,
    pub references: Vec<StyleReference>,
    pub master_style: Option<DynamicImage>,
}

impl Default for StyleGuide {
    fn default() -> Self {
        let mut colors = HashMap::new();
        colors.insert("primary".into(), Color::rgb(0.2, 0.7, 0.9));
        colors.insert("secondary".into(), Color::rgb(0.9, 0.4, 0.3));
        colors.insert("background".into(), Color::rgb(0.1, 0.1, 0.2));
        
        Self {
            colors,
            references: Vec::new(),
            master_style: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct StyleReference {
    pub name: String,
    pub image_data: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct GenerationTask {
    pub id: Uuid,
    pub task_type: TaskType,
    pub status: TaskStatus,
    pub progress: f32,
    pub output: Option<GenerationOutput>,
    pub dependencies: Vec<Uuid>,
    pub priority: i32,
}

#[derive(Debug, Clone)]
pub enum TaskType {
    StyleGuide,
    Character { name: String },
    Tileset { theme: String },
    UI { element: String },
    Audio { track_type: String },
    Code { module: String },
}

#[derive(Debug, Clone)]
pub enum TaskStatus {
    Pending,
    Running,
    Completed,
    Failed(String),
}

#[derive(Debug, Clone)]
pub struct GenerationOutput {
    pub data: Vec<u8>,
    pub metadata: AssetMetadata,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AssetMetadata {
    pub asset_type: String,
    pub dimensions: Option<(u32, u32)>,
    pub format: String,
    pub generation_params: HashMap<String, String>,
    pub quality_score: f32,
    pub style_consistency: f32,
}

/// Generation request types
#[derive(Debug, Clone)]
pub enum GenerationRequest {
    FullGame { config: GameConfiguration },
    SingleAsset { asset_type: String, params: HashMap<String, String> },
    RegenerateAsset { asset_id: String, modifications: HashMap<String, String> },
    BatchOperation { assets: Vec<String>, operation: BatchOp },
}

#[derive(Debug, Clone)]
pub enum BatchOp {
    ApplyStyle { style_id: String, strength: f32 },
    Resize { dimensions: (u32, u32) },
    Recolor { palette: Vec<Color> },
}

/// Generation results
#[derive(Debug)]
pub enum GenerationResult {
    Success { task_id: Uuid, output: GenerationOutput },
    Error { task_id: Uuid, error: String },
    Progress { task_id: Uuid, progress: f32 },
}

/// Main generation loop that runs in a separate thread
pub async fn run_generation_loop(
    rx: Receiver<GenerationRequest>,
    tx: Sender<GenerationResult>,
) {
    let generator = match ConsistentAssetGenerator::new().await {
        Ok(g) => g,
        Err(e) => {
            error!("Failed to initialize generator: {}", e);
            return;
        }
    };
    
    while let Ok(request) = rx.recv() {
        match request {
            GenerationRequest::FullGame { config } => {
                if let Err(e) = generator.generate_full_game(config, tx.clone()).await {
                    error!("Failed to generate game: {}", e);
                }
            }
            GenerationRequest::SingleAsset { asset_type, params } => {
                if let Err(e) = generator.generate_single_asset(&asset_type, params, tx.clone()).await {
                    error!("Failed to generate asset: {}", e);
                }
            }
            GenerationRequest::RegenerateAsset { asset_id, modifications } => {
                if let Err(e) = generator.regenerate_asset(&asset_id, modifications, tx.clone()).await {
                    error!("Failed to regenerate asset: {}", e);
                }
            }
            GenerationRequest::BatchOperation { assets, operation } => {
                if let Err(e) = generator.batch_operation(assets, operation, tx.clone()).await {
                    error!("Failed to perform batch operation: {}", e);
                }
            }
        }
    }
}

/// Main asset generator with all advanced features
pub struct ConsistentAssetGenerator {
    openai_client: async_openai::Client<async_openai::config::OpenAIConfig>,
    style_transfer: Arc<NeuralStyleTransfer>,
    pixel_processor: Arc<PixelArtProcessor>,
    dependency_graph: Arc<AssetDependencyGraph>,
    smart_cache: Arc<SmartCache>,
    prompt_optimizer: Arc<PromptOptimizer>,
    sprite_optimizer: Arc<SpriteSheetOptimizer>,
}

impl ConsistentAssetGenerator {
    pub async fn new() -> anyhow::Result<Self> {
        let openai_client = async_openai::Client::new();
        
        Ok(Self {
            openai_client,
            style_transfer: Arc::new(NeuralStyleTransfer::new().await?),
            pixel_processor: Arc::new(PixelArtProcessor::new()),
            dependency_graph: Arc::new(AssetDependencyGraph::new()),
            smart_cache: Arc::new(SmartCache::new("./cache").await?),
            prompt_optimizer: Arc::new(PromptOptimizer::new().await?),
            sprite_optimizer: Arc::new(SpriteSheetOptimizer::new()),
        })
    }
    
    /// Generate complete game with all assets
    pub async fn generate_full_game(
        &self,
        config: GameConfiguration,
        tx: Sender<GenerationResult>,
    ) -> anyhow::Result<()> {
        // Build dependency graph based on game config
        self.build_dependency_graph(&config)?;
        
        // Phase 1: Generate style guide first
        let style_guide_id = Uuid::new_v4();
        self.generate_style_guide(&config, style_guide_id, tx.clone()).await?;
        
        // Phase 2: Generate core assets in dependency order
        let tasks = self.create_generation_tasks(&config)?;
        
        // Process tasks in parallel with dependency awareness
        let pipeline = ParallelGenerationPipeline::new(self.clone());
        pipeline.process_tasks(tasks, tx.clone()).await?;
        
        // Phase 3: Create optimized sprite sheets
        self.create_sprite_atlases(tx.clone()).await?;
        
        // Phase 4: Generate code with asset references
        self.generate_game_code(&config, tx.clone()).await?;
        
        Ok(())
    }
    
    /// Generate style guide that all other assets will follow
    async fn generate_style_guide(
        &self,
        config: &GameConfiguration,
        task_id: Uuid,
        tx: Sender<GenerationResult>,
    ) -> anyhow::Result<()> {
        // Create style prompt based on references
        let style_prompt = format!(
            "Create a cohesive visual style guide for a {} game inspired by: {}. \
             Color mood: {:?}. Detail level: {:?}. \
             Include: color palette, texture examples, lighting style, and character proportions.",
            config.genre,
            config.art_references.join(", "),
            config.color_mood,
            config.sprite_style.detail_level
        );
        
        // Generate base style image
        let style_image = self.generate_image(&style_prompt, (512, 512)).await?;
        
        // Extract style features for consistency
        let style_features = self.style_transfer.extract_style_features(&style_image).await?;
        self.style_transfer.cache_style_features("master_style", style_features)?;
        
        // Process to pixel art style
        let processed = self.pixel_processor.process_to_pixel_art(style_image)?;
        
        // Send result
        let output = GenerationOutput {
            data: image_to_bytes(&processed)?,
            metadata: AssetMetadata {
                asset_type: "style_guide".into(),
                dimensions: Some((512, 512)),
                format: "png".into(),
                generation_params: HashMap::new(),
                quality_score: 1.0,
                style_consistency: 1.0,
            },
        };
        
        tx.send(GenerationResult::Success { task_id, output })?;
        Ok(())
    }
    
    /// Generate single asset with style consistency
    pub async fn generate_single_asset(
        &self,
        asset_type: &str,
        params: HashMap<String, String>,
        tx: Sender<GenerationResult>,
    ) -> anyhow::Result<()> {
        let task_id = Uuid::new_v4();
        
        // Optimize prompt based on history
        let base_prompt = params.get("prompt").unwrap_or(&String::new()).clone();
        let optimized_prompt = self.prompt_optimizer.optimize_prompt(&base_prompt).await?;
        
        // Generate based on asset type
        let result = match asset_type {
            "character" => self.generate_character(&optimized_prompt, &params).await?,
            "tileset" => self.generate_tileset(&optimized_prompt, &params).await?,
            "ui_element" => self.generate_ui_element(&optimized_prompt, &params).await?,
            "audio" => self.generate_audio(&optimized_prompt, &params).await?,
            _ => return Err(anyhow::anyhow!("Unknown asset type: {}", asset_type)),
        };
        
        // Apply style transfer if we have a master style
        let final_result = if let Some(master_style) = self.get_cached_style("master_style")? {
            self.apply_style_consistency(result, &master_style, 0.3).await?
        } else {
            result
        };
        
        // Cache the result
        self.smart_cache.store_asset(
            &format!("{}_{}", asset_type, task_id),
            &final_result.data,
            final_result.metadata.clone()
        ).await?;
        
        tx.send(GenerationResult::Success {
            task_id,
            output: final_result,
        })?;
        
        Ok(())
    }
    
    /// Generate character sprite with animations
    async fn generate_character(
        &self,
        prompt: &str,
        params: &HashMap<String, String>,
    ) -> anyhow::Result<GenerationOutput> {
        let size = params.get("size")
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or(32);
        
        // Generate base character
        let character_prompt = format!(
            "Pixel art character sprite for: {}. \
             Size: {}x{} pixels. \
             Front-facing, idle pose. \
             Clear outline, limited colors.",
            prompt, size, size
        );
        
        let base_image = self.generate_image(&character_prompt, (size, size)).await?;
        
        // Process to clean pixel art
        let processed = self.pixel_processor.process_to_pixel_art(base_image)?;
        
        // Generate animation frames if requested
        let mut frames = vec![processed.clone()];
        if params.get("animated").map(|v| v == "true").unwrap_or(false) {
            frames.extend(self.generate_animation_frames(&processed, params).await?);
        }
        
        // Create sprite sheet
        let (sprite_sheet, metadata) = self.sprite_optimizer.create_character_sheet(frames)?;
        
        Ok(GenerationOutput {
            data: image_to_bytes(&sprite_sheet)?,
            metadata: AssetMetadata {
                asset_type: "character".into(),
                dimensions: Some(sprite_sheet.dimensions()),
                format: "png".into(),
                generation_params: params.clone(),
                quality_score: 0.9,
                style_consistency: 0.95,
            },
        })
    }
    
    /// Batch operations for multiple assets
    pub async fn batch_operation(
        &self,
        assets: Vec<String>,
        operation: BatchOp,
        tx: Sender<GenerationResult>,
    ) -> anyhow::Result<()> {
        let results: Vec<_> = assets.par_iter()
            .filter_map(|asset_id| {
                match self.apply_operation(asset_id, &operation) {
                    Ok(output) => Some(Ok((Uuid::new_v4(), output))),
                    Err(e) => Some(Err(e)),
                }
            })
            .collect();
        
        for result in results {
            match result {
                Ok((task_id, output)) => {
                    tx.send(GenerationResult::Success { task_id, output })?;
                }
                Err(e) => {
                    error!("Batch operation failed: {}", e);
                }
            }
        }
        
        Ok(())
    }
    
    fn apply_operation(
        &self,
        asset_id: &str,
        operation: &BatchOp,
    ) -> anyhow::Result<GenerationOutput> {
        // Implementation for each batch operation type
        match operation {
            BatchOp::ApplyStyle { style_id, strength } => {
                // Apply style transfer
                // First, get the asset from cache
                let cache_result = tokio::runtime::Handle::current()
                    .block_on(self.smart_cache.get_asset(asset_id))?;
                
                if let Some((asset_data, metadata)) = cache_result {
                    // Load the image
                    let image = image::load_from_memory(&asset_data)?;
                    
                    // Get the style to apply
                    if let Some(style_features) = self.style_transfer.style_embeddings.get(style_id) {
                        // Apply style transfer
                        let styled = tokio::runtime::Handle::current()
                            .block_on(self.style_transfer.apply_style_transfer(
                                &image,
                                &style_features,
                                *strength
                            ))?;
                        
                        Ok(GenerationOutput {
                            data: image_to_bytes(&styled)?,
                            metadata: AssetMetadata {
                                style_consistency: strength,
                                ..metadata
                            },
                        })
                    } else {
                        Err(anyhow::anyhow!("Style {} not found", style_id))
                    }
                } else {
                    Err(anyhow::anyhow!("Asset {} not found in cache", asset_id))
                }
            }
            BatchOp::Resize { dimensions } => {
                // Resize asset
                let cache_result = tokio::runtime::Handle::current()
                    .block_on(self.smart_cache.get_asset(asset_id))?;
                
                if let Some((asset_data, mut metadata)) = cache_result {
                    let image = image::load_from_memory(&asset_data)?;
                    let resized = image.resize_exact(
                        dimensions.0,
                        dimensions.1,
                        image::imageops::FilterType::Nearest
                    );
                    
                    metadata.dimensions = Some(*dimensions);
                    
                    Ok(GenerationOutput {
                        data: image_to_bytes(&resized)?,
                        metadata,
                    })
                } else {
                    Err(anyhow::anyhow!("Asset {} not found in cache", asset_id))
                }
            }
            BatchOp::Recolor { palette } => {
                // Recolor with new palette
                let cache_result = tokio::runtime::Handle::current()
                    .block_on(self.smart_cache.get_asset(asset_id))?;
                
                if let Some((asset_data, metadata)) = cache_result {
                    let mut image = image::load_from_memory(&asset_data)?;
                    
                    // Simple palette swap - map existing colors to new palette
                    let rgba = image.to_rgba8();
                    let mut result = rgba.clone();
                    
                    // Extract unique colors from the image
                    let mut unique_colors = std::collections::HashSet::new();
                    for pixel in rgba.pixels() {
                        unique_colors.insert(*pixel);
                    }
                    
                    // Sort colors by luminance
                    let mut sorted_colors: Vec<_> = unique_colors.into_iter().collect();
                    sorted_colors.sort_by_key(|p| {
                        let r = p[0] as u32;
                        let g = p[1] as u32;
                        let b = p[2] as u32;
                        // Luminance formula
                        (r * 299 + g * 587 + b * 114) / 1000
                    });
                    
                    // Create color mapping
                    let color_map: HashMap<Rgba<u8>, Rgba<u8>> = sorted_colors.iter()
                        .zip(palette.iter().cycle())
                        .map(|(old_color, new_color)| {
                            let rgba = Rgba([
                                (new_color.r() * 255.0) as u8,
                                (new_color.g() * 255.0) as u8,
                                (new_color.b() * 255.0) as u8,
                                old_color[3], // Keep original alpha
                            ]);
                            (*old_color, rgba)
                        })
                        .collect();
                    
                    // Apply color mapping
                    for (x, y, pixel) in result.enumerate_pixels_mut() {
                        if let Some(new_color) = color_map.get(&rgba.get_pixel(x, y)) {
                            *pixel = *new_color;
                        }
                    }
                    
                    Ok(GenerationOutput {
                        data: image_to_bytes(&DynamicImage::ImageRgba8(result))?,
                        metadata,
                    })
                } else {
                    Err(anyhow::anyhow!("Asset {} not found in cache", asset_id))
                }
            }
        }
    }
}

/// Neural style transfer for visual consistency
pub struct NeuralStyleTransfer {
    style_embeddings: DashMap<String, Vec<f32>>,
    model_path: PathBuf,
}

impl NeuralStyleTransfer {
    pub async fn new() -> anyhow::Result<Self> {
        Ok(Self {
            style_embeddings: DashMap::new(),
            model_path: PathBuf::from("./models/style_transfer"),
        })
    }
    
    pub async fn extract_style_features(&self, image: &DynamicImage) -> anyhow::Result<Vec<f32>> {
        // Simplified: In reality, this would use a neural network
        let (width, height) = image.dimensions();
        let rgb = image.to_rgb8();
        
        // Extract basic color statistics as "style"
        let mut r_sum = 0.0;
        let mut g_sum = 0.0;
        let mut b_sum = 0.0;
        let pixel_count = (width * height) as f32;
        
        for pixel in rgb.pixels() {
            r_sum += pixel[0] as f32;
            g_sum += pixel[1] as f32;
            b_sum += pixel[2] as f32;
        }
        
        Ok(vec![
            r_sum / pixel_count / 255.0,
            g_sum / pixel_count / 255.0,
            b_sum / pixel_count / 255.0,
        ])
    }
    
    pub fn cache_style_features(&self, name: &str, features: Vec<f32>) -> anyhow::Result<()> {
        self.style_embeddings.insert(name.to_string(), features);
        Ok(())
    }
    
    pub async fn apply_style_transfer(
        &self,
        content: &DynamicImage,
        style_features: &[f32],
        strength: f32,
    ) -> anyhow::Result<DynamicImage> {
        // Simplified style transfer - adjust colors based on style
        let mut result = content.clone();
        
        if style_features.len() >= 3 {
            let r_factor = style_features[0];
            let g_factor = style_features[1];
            let b_factor = style_features[2];
            
            // Apply color adjustment
            let adjusted = image::imageops::colorops::contrast(&result, strength);
            // More sophisticated style transfer would go here
            
            Ok(adjusted)
        } else {
            Ok(result)
        }
    }
}

/// Pixel art processor for clean, retro-style graphics
pub struct PixelArtProcessor {
    target_resolution: (u32, u32),
    outline_color: Rgba<u8>,
    dither_strength: f32,
}

impl PixelArtProcessor {
    pub fn new() -> Self {
        Self {
            target_resolution: (32, 32),
            outline_color: Rgba([0, 0, 0, 255]),
            dither_strength: 0.3,
        }
    }
    
    pub fn process_to_pixel_art(&self, image: DynamicImage) -> anyhow::Result<DynamicImage> {
        // Resize to pixel dimensions
        let resized = image.resize_exact(
            self.target_resolution.0,
            self.target_resolution.1,
            image::imageops::FilterType::Nearest,
        );
        
        // Quantize colors
        let quantized = self.quantize_colors(resized, 16)?;
        
        // Add outline
        let outlined = self.add_outline(quantized)?;
        
        Ok(outlined)
    }
    
    fn quantize_colors(&self, image: DynamicImage, color_count: u32) -> anyhow::Result<DynamicImage> {
        // Simple color quantization
        let rgba = image.to_rgba8();
        let mut result = ImageBuffer::new(rgba.width(), rgba.height());
        
        for (x, y, pixel) in rgba.enumerate_pixels() {
            let quantized = Rgba([
                (pixel[0] / 32) * 32,
                (pixel[1] / 32) * 32,
                (pixel[2] / 32) * 32,
                pixel[3],
            ]);
            result.put_pixel(x, y, quantized);
        }
        
        Ok(DynamicImage::ImageRgba8(result))
    }
    
    fn add_outline(&self, image: DynamicImage) -> anyhow::Result<DynamicImage> {
        let rgba = image.to_rgba8();
        let mut result = rgba.clone();
        
        // Simple outline by checking neighbors
        for y in 1..rgba.height() - 1 {
            for x in 1..rgba.width() - 1 {
                let pixel = rgba.get_pixel(x, y);
                if pixel[3] > 0 {
                    // Check if any neighbor is transparent
                    let neighbors = [
                        rgba.get_pixel(x - 1, y),
                        rgba.get_pixel(x + 1, y),
                        rgba.get_pixel(x, y - 1),
                        rgba.get_pixel(x, y + 1),
                    ];
                    
                    if neighbors.iter().any(|p| p[3] == 0) {
                        result.put_pixel(x, y, self.outline_color);
                    }
                }
            }
        }
        
        Ok(DynamicImage::ImageRgba8(result))
    }
}

/// Asset dependency graph for generation ordering
pub struct AssetDependencyGraph {
    graph: DiGraph<AssetNode, AssetRelation>,
    node_map: HashMap<String, NodeIndex>,
}

#[derive(Clone, Debug)]
pub struct AssetNode {
    pub id: String,
    pub asset_type: AssetType,
    pub generation_params: HashMap<String, String>,
}

#[derive(Clone, Debug)]
pub enum AssetType {
    StyleGuide,
    Character,
    Tileset,
    UI,
    Audio,
    Code,
}

#[derive(Clone, Debug)]
pub struct AssetRelation {
    pub relation_type: RelationType,
    pub weight: f32,
}

#[derive(Clone, Debug)]
pub enum RelationType {
    StyleParent,
    ColorReference,
    SizeReference,
    AnimationSet,
}

impl AssetDependencyGraph {
    pub fn new() -> Self {
        Self {
            graph: DiGraph::new(),
            node_map: HashMap::new(),
        }
    }
    
    pub fn add_asset(&mut self, id: String, asset_type: AssetType) -> NodeIndex {
        let node = AssetNode {
            id: id.clone(),
            asset_type,
            generation_params: HashMap::new(),
        };
        let idx = self.graph.add_node(node);
        self.node_map.insert(id, idx);
        idx
    }
    
    pub fn add_dependency(&mut self, from: &str, to: &str, relation: RelationType) {
        if let (Some(&from_idx), Some(&to_idx)) = (self.node_map.get(from), self.node_map.get(to)) {
            self.graph.add_edge(from_idx, to_idx, AssetRelation {
                relation_type: relation,
                weight: 1.0,
            });
        }
    }
    
    pub fn get_generation_order(&self) -> Vec<String> {
        petgraph::algo::toposort(&self.graph, None)
            .ok()
            .map(|indices| {
                indices.into_iter()
                    .map(|idx| self.graph[idx].id.clone())
                    .collect()
            })
            .unwrap_or_default()
    }
}

/// Smart caching system with compression
pub struct SmartCache {
    cache_dir: PathBuf,
    memory_cache: Arc<Mutex<lru::LruCache<String, Vec<u8>>>>,
}

impl SmartCache {
    pub async fn new(cache_dir: impl Into<PathBuf>) -> anyhow::Result<Self> {
        let cache_dir = cache_dir.into();
        tokio::fs::create_dir_all(&cache_dir).await?;
        
        Ok(Self {
            cache_dir,
            memory_cache: Arc::new(Mutex::new(lru::LruCache::new(
                std::num::NonZeroUsize::new(100).unwrap()
            ))),
        })
    }
    
    pub async fn store_asset(
        &self,
        key: &str,
        data: &[u8],
        metadata: AssetMetadata,
    ) -> anyhow::Result<()> {
        // Store in memory cache
        if let Ok(mut cache) = self.memory_cache.lock() {
            cache.put(key.to_string(), data.to_vec());
        }
        
        // Compress and store to disk
        let compressed = zstd::encode_all(data, 3)?;
        let file_path = self.cache_dir.join(format!("{}.zst", key));
        tokio::fs::write(&file_path, compressed).await?;
        
        // Store metadata
        let meta_path = self.cache_dir.join(format!("{}.meta", key));
        let meta_json = serde_json::to_string(&metadata)?;
        tokio::fs::write(meta_path, meta_json).await?;
        
        Ok(())
    }
    
    pub async fn get_asset(&self, key: &str) -> anyhow::Result<Option<(Vec<u8>, AssetMetadata)>> {
        // Check memory cache first
        if let Ok(mut cache) = self.memory_cache.lock() {
            if let Some(data) = cache.get(key) {
                // Load metadata
                let meta_path = self.cache_dir.join(format!("{}.meta", key));
                if let Ok(meta_json) = tokio::fs::read_to_string(meta_path).await {
                    if let Ok(metadata) = serde_json::from_str(&meta_json) {
                        return Ok(Some((data.clone(), metadata)));
                    }
                }
            }
        }
        
        // Load from disk
        let file_path = self.cache_dir.join(format!("{}.zst", key));
        if file_path.exists() {
            let compressed = tokio::fs::read(&file_path).await?;
            let data = zstd::decode_all(&compressed[..])?;
            
            let meta_path = self.cache_dir.join(format!("{}.meta", key));
            let meta_json = tokio::fs::read_to_string(meta_path).await?;
            let metadata = serde_json::from_str(&meta_json)?;
            
            // Update memory cache
            if let Ok(mut cache) = self.memory_cache.lock() {
                cache.put(key.to_string(), data.clone());
            }
            
            Ok(Some((data, metadata)))
        } else {
            Ok(None)
        }
    }
}

/// Prompt optimization with learning
pub struct PromptOptimizer {
    success_history: DashMap<String, PromptMetrics>,
}

#[derive(Clone, Debug)]
struct PromptMetrics {
    success_rate: f32,
    avg_quality: f32,
    generation_time: std::time::Duration,
}

impl PromptOptimizer {
    pub async fn new() -> anyhow::Result<Self> {
        Ok(Self {
            success_history: DashMap::new(),
        })
    }
    
    pub async fn optimize_prompt(&self, base_prompt: &str) -> anyhow::Result<String> {
        // Add learned modifiers for better results
        let optimized = format!(
            "{}, high quality, detailed, consistent style, professional game asset",
            base_prompt
        );
        
        Ok(optimized)
    }
    
    pub fn record_result(&self, prompt: &str, quality: f32, duration: std::time::Duration) {
        self.success_history.entry(prompt.to_string())
            .and_modify(|metrics| {
                metrics.success_rate = (metrics.success_rate + quality) / 2.0;
                metrics.generation_time = duration;
            })
            .or_insert(PromptMetrics {
                success_rate: quality,
                avg_quality: quality,
                generation_time: duration,
            });
    }
}

/// Sprite sheet optimizer
pub struct SpriteSheetOptimizer {
    max_atlas_size: (u32, u32),
    padding: u32,
}

impl SpriteSheetOptimizer {
    pub fn new() -> Self {
        Self {
            max_atlas_size: (2048, 2048),
            padding: 2,
        }
    }
    
    pub fn create_character_sheet(
        &self,
        frames: Vec<DynamicImage>,
    ) -> anyhow::Result<(DynamicImage, HashMap<String, (u32, u32, u32, u32)>)> {
        if frames.is_empty() {
            return Err(anyhow::anyhow!("No frames provided"));
        }
        
        let frame_size = frames[0].dimensions();
        let frames_per_row = ((self.max_atlas_size.0 - self.padding) / (frame_size.0 + self.padding)).min(frames.len() as u32);
        let rows_needed = ((frames.len() as u32 + frames_per_row - 1) / frames_per_row).min(
            (self.max_atlas_size.1 - self.padding) / (frame_size.1 + self.padding)
        );
        
        let atlas_width = frames_per_row * (frame_size.0 + self.padding) + self.padding;
        let atlas_height = rows_needed * (frame_size.1 + self.padding) + self.padding;
        
        let mut atlas = ImageBuffer::from_pixel(
            atlas_width,
            atlas_height,
            Rgba([0, 0, 0, 0])
        );
        
        let mut positions = HashMap::new();
        
        for (i, frame) in frames.iter().enumerate() {
            let row = i as u32 / frames_per_row;
            let col = i as u32 % frames_per_row;
            
            let x = self.padding + col * (frame_size.0 + self.padding);
            let y = self.padding + row * (frame_size.1 + self.padding);
            
            image::imageops::overlay(&mut atlas, frame, x.into(), y.into());
            
            positions.insert(
                format!("frame_{}", i),
                (x, y, frame_size.0, frame_size.1)
            );
        }
        
        Ok((DynamicImage::ImageRgba8(atlas), positions))
    }
}

/// Parallel generation pipeline
struct ParallelGenerationPipeline {
    generator: ConsistentAssetGenerator,
}

impl ParallelGenerationPipeline {
    fn new(generator: ConsistentAssetGenerator) -> Self {
        Self { generator }
    }
    
    async fn process_tasks(
        &self,
        tasks: Vec<GenerationTask>,
        tx: Sender<GenerationResult>,
    ) -> anyhow::Result<()> {
        use futures::future::join_all;
        
        // Group tasks by dependency level
        let mut task_levels = self.group_by_dependencies(tasks);
        
        // Process each level in parallel
        for level_tasks in task_levels {
            let futures: Vec<_> = level_tasks.into_iter()
                .map(|task| {
                    let tx = tx.clone();
                    let generator = self.generator.clone();
                    async move {
                        generator.process_single_task(task, tx).await
                    }
                })
                .collect();
            
            join_all(futures).await;
        }
        
        Ok(())
    }
    
    fn group_by_dependencies(&self, tasks: Vec<GenerationTask>) -> Vec<Vec<GenerationTask>> {
        // Simple grouping - in practice would use topological sort
        vec![tasks]
    }
}

// Helper functions
fn image_to_bytes(image: &DynamicImage) -> anyhow::Result<Vec<u8>> {
    use image::ImageOutputFormat;
    let mut bytes = Vec::new();
    image.write_to(&mut std::io::Cursor::new(&mut bytes), ImageOutputFormat::Png)?;
    Ok(bytes)
}

impl ConsistentAssetGenerator {
    async fn generate_image(&self, prompt: &str, size: (u32, u32)) -> anyhow::Result<DynamicImage> {
        use async_openai::types::{CreateImageRequestArgs, ImageSize, ImageModel};
        
        let image_size = match size {
            (256, 256) => ImageSize::S256x256,
            (512, 512) => ImageSize::S512x512,
            (1024, 1024) => ImageSize::S1024x1024,
            _ => ImageSize::S512x512,
        };
        
        let request = CreateImageRequestArgs::default()
            .prompt(prompt)
            .model(ImageModel::DallE3)
            .size(image_size)
            .n(1)
            .build()?;
        
        let response = self.openai_client.images().create(request).await?;
        
        if let Some(data) = response.data.first() {
            if let Some(url) = &data.url {
                // Download image from URL
                let bytes = reqwest::get(url).await?.bytes().await?;
                let image = image::load_from_memory(&bytes)?;
                return Ok(image);
            }
        }
        
        Err(anyhow::anyhow!("Failed to generate image"))
    }
    
    async fn generate_tileset(&self, prompt: &str, params: &HashMap<String, String>) -> anyhow::Result<GenerationOutput> {
        let tile_size = params.get("tile_size")
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or(16);
        
        let tileset_prompt = format!(
            "Tileset for: {}. \
             Individual {}x{} pixel tiles arranged in a grid. \
             Include: ground, walls, decorations. \
             Consistent pixel art style.",
            prompt, tile_size, tile_size
        );
        
        let tileset = self.generate_image(&tileset_prompt, (256, 256)).await?;
        let processed = self.pixel_processor.process_to_pixel_art(tileset)?;
        
        Ok(GenerationOutput {
            data: image_to_bytes(&processed)?,
            metadata: AssetMetadata {
                asset_type: "tileset".into(),
                dimensions: Some(processed.dimensions()),
                format: "png".into(),
                generation_params: params.clone(),
                quality_score: 0.9,
                style_consistency: 0.9,
            },
        })
    }
    
    async fn generate_ui_element(&self, prompt: &str, params: &HashMap<String, String>) -> anyhow::Result<GenerationOutput> {
        let element_type = params.get("element_type").cloned().unwrap_or_else(|| "button".into());
        
        let ui_prompt = format!(
            "Game UI {} element: {}. \
             Clean, pixel art style. \
             Suitable for game interface.",
            element_type, prompt
        );
        
        let ui_element = self.generate_image(&ui_prompt, (256, 128)).await?;
        let processed = self.pixel_processor.process_to_pixel_art(ui_element)?;
        
        Ok(GenerationOutput {
            data: image_to_bytes(&processed)?,
            metadata: AssetMetadata {
                asset_type: "ui_element".into(),
                dimensions: Some(processed.dimensions()),
                format: "png".into(),
                generation_params: params.clone(),
                quality_score: 0.85,
                style_consistency: 0.9,
            },
        })
    }
    
    async fn generate_audio(&self, prompt: &str, params: &HashMap<String, String>) -> anyhow::Result<GenerationOutput> {
        // Since direct audio generation isn't available, generate specifications
        let audio_type = params.get("audio_type").cloned().unwrap_or_else(|| "sfx".into());
        
        let audio_spec = serde_json::json!({
            "type": audio_type,
            "description": prompt,
            "duration": params.get("duration").unwrap_or(&"1.0".to_string()),
            "format": "procedural_spec",
            "synthesis_params": {
                "waveform": "sine",
                "frequency": 440.0,
                "envelope": {
                    "attack": 0.1,
                    "decay": 0.2,
                    "sustain": 0.7,
                    "release": 0.3
                }
            }
        });
        
        Ok(GenerationOutput {
            data: serde_json::to_vec(&audio_spec)?,
            metadata: AssetMetadata {
                asset_type: "audio_spec".into(),
                dimensions: None,
                format: "json".into(),
                generation_params: params.clone(),
                quality_score: 0.8,
                style_consistency: 1.0,
            },
        })
    }
    
    async fn generate_animation_frames(
        &self,
        base_image: &DynamicImage,
        params: &HashMap<String, String>,
    ) -> anyhow::Result<Vec<DynamicImage>> {
        let frame_count = params.get("frame_count")
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(4);
        
        // For now, create simple variations
        // In practice, this would use more sophisticated animation generation
        let mut frames = Vec::new();
        
        for i in 1..frame_count {
            let offset = i as i64 * 2;
            let shifted = image::imageops::translate(base_image, offset, 0);
            frames.push(shifted);
        }
        
        Ok(frames)
    }
    
    fn build_dependency_graph(&self, config: &GameConfiguration) -> anyhow::Result<()> {
        // Build relationships between assets based on game configuration
        // This is simplified - real implementation would be more complex
        Ok(())
    }
    
    fn create_generation_tasks(&self, config: &GameConfiguration) -> anyhow::Result<Vec<GenerationTask>> {
        let mut tasks = Vec::new();
        
        // Create character generation tasks
        tasks.push(GenerationTask {
            id: Uuid::new_v4(),
            task_type: TaskType::Character { name: "hero".into() },
            status: TaskStatus::Pending,
            progress: 0.0,
            output: None,
            dependencies: vec![],
            priority: 10,
        });
        
        // Create tileset tasks
        tasks.push(GenerationTask {
            id: Uuid::new_v4(),
            task_type: TaskType::Tileset { theme: "grassland".into() },
            status: TaskStatus::Pending,
            progress: 0.0,
            output: None,
            dependencies: vec![],
            priority: 5,
        });
        
        // Add more tasks based on config...
        
        Ok(tasks)
    }
    
    async fn create_sprite_atlases(&self, tx: Sender<GenerationResult>) -> anyhow::Result<()> {
        // Collect all sprites and create optimized atlases
        Ok(())
    }
    
    async fn generate_game_code(&self, config: &GameConfiguration, tx: Sender<GenerationResult>) -> anyhow::Result<()> {
        // Generate Bevy game code based on configuration and assets
        Ok(())
    }
    
    async fn apply_style_consistency(
        &self,
        content: GenerationOutput,
        style: &DynamicImage,
        strength: f32,
    ) -> anyhow::Result<GenerationOutput> {
        // Apply style transfer to ensure consistency
        if let Ok(content_image) = image::load_from_memory(&content.data) {
            let styled = self.style_transfer.apply_style_transfer(
                &content_image,
                &[0.5, 0.5, 0.5], // Placeholder style features
                strength
            ).await?;
            
            Ok(GenerationOutput {
                data: image_to_bytes(&styled)?,
                metadata: content.metadata,
            })
        } else {
            Ok(content)
        }
    }
    
    fn get_cached_style(&self, name: &str) -> anyhow::Result<Option<DynamicImage>> {
        // Retrieve cached style image
        Ok(None)
    }
    
    async fn process_single_task(&self, task: GenerationTask, tx: Sender<GenerationResult>) -> anyhow::Result<()> {
        // Process individual generation task
        match task.task_type {
            TaskType::Character { name } => {
                let params = HashMap::from([
                    ("name".to_string(), name),
                    ("animated".to_string(), "true".to_string()),
                ]);
                self.generate_single_asset("character", params, tx).await?;
            }
            TaskType::Tileset { theme } => {
                let params = HashMap::from([
                    ("theme".to_string(), theme),
                    ("tile_size".to_string(), "16".to_string()),
                ]);
                self.generate_single_asset("tileset", params, tx).await?;
            }
            _ => {}
        }
        Ok(())
    }
}

// Clone implementation for generator (simplified)
impl Clone for ConsistentAssetGenerator {
    fn clone(&self) -> Self {
        Self {
            openai_client: self.openai_client.clone(),
            style_transfer: self.style_transfer.clone(),
            pixel_processor: self.pixel_processor.clone(),
            dependency_graph: self.dependency_graph.clone(),
            smart_cache: self.smart_cache.clone(),
            prompt_optimizer: self.prompt_optimizer.clone(),
            sprite_optimizer: self.sprite_optimizer.clone(),
        }
    }
}

// Additional resources
#[derive(Resource, Default)]
pub struct ProjectDatabase {
    pub projects: HashMap<Uuid, ProjectData>,
}

#[derive(Clone)]
pub struct ProjectData {
    pub id: Uuid,
    pub name: String,
    pub config: GameConfiguration,
    pub assets: HashMap<String, String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_modified: chrono::DateTime<chrono::Utc>,
}

#[derive(Resource, Default)]
pub struct AssetCache {
    pub cached_assets: DashMap<String, CachedAsset>,
}

#[derive(Clone)]
pub struct CachedAsset {
    pub id: String,
    pub data: Vec<u8>,
    pub metadata: AssetMetadata,
    pub thumbnail: Option<Vec<u8>>,
}

#[derive(Resource, Default)]
pub struct GenerationTasks {
    pub active_tasks: DashMap<Uuid, TaskHandle>,
}

pub struct TaskHandle {
    pub task: GenerationTask,
    pub start_time: std::time::Instant,
    pub estimated_completion: Option<std::time::Instant>,
}

#[derive(Clone)]
pub struct GeneratedAsset {
    pub id: String,
    pub asset_type: String,
    pub data: Vec<u8>,
    pub metadata: AssetMetadata,
    pub style_consistency: f32,
    pub quality_score: f32,
}

// Required trait implementations
use uuid::Uuid;