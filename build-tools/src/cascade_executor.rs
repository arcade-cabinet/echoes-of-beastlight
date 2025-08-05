use crate::metaprompt::{MetaPrompt, PromptCascade, OutputType};
use anyhow::{Result, Context};
use async_openai::{
    types::{CreateChatCompletionRequestArgs, ChatCompletionRequestMessage, ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs},
    Client,
};
use minijinja::{Environment, Value};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tokio::fs;
use tracing::{info, debug};

pub struct CascadeExecutor {
    client: Client<async_openai::config::OpenAIConfig>,
    env: Environment<'static>,
    cache_dir: PathBuf,
    dry_run: bool,
}

impl CascadeExecutor {
    pub fn new(cache_dir: PathBuf, dry_run: bool) -> Result<Self> {
        let client = Client::new();
        let mut env = Environment::new();
        
        // Configure minijinja
        env.set_lstrip_blocks(true);
        env.set_trim_blocks(true);
        
        Ok(Self {
            client,
            env,
            cache_dir,
            dry_run,
        })
    }
    
    /// Execute a complete prompt cascade
    pub async fn execute_cascade(&mut self, cascade: &PromptCascade, output_dir: &Path) -> Result<()> {
        info!("Executing cascade: {}", cascade.name);
        
        // Get execution order
        let order = cascade.get_execution_order()
            .context("Failed to determine execution order")?;
        
        info!("Execution order: {:?}", order);
        
        // Create context with global variables
        let mut context = HashMap::new();
        for (key, value) in &cascade.global_variables {
            context.insert(key.clone(), value.clone());
        }
        
        // Execute prompts in order
        for prompt_id in order {
            let prompt = cascade.prompts.get(&prompt_id)
                .context(format!("Prompt {} not found", prompt_id))?;
            
            self.execute_prompt(prompt, cascade, output_dir, &mut context).await?;
        }
        
        Ok(())
    }
    
    /// Execute a single prompt
    async fn execute_prompt(
        &mut self,
        prompt: &MetaPrompt,
        cascade: &PromptCascade,
        output_dir: &Path,
        context: &mut HashMap<String, serde_json::Value>,
    ) -> Result<()> {
        info!("Executing prompt: {}", prompt.id);
        
        // Check cache first if idempotent
        if prompt.idempotent {
            let cache_key = self.render_cache_key(prompt, context)?;
            if self.check_cache(&cache_key).await? {
                info!("Using cached result for {}", prompt.id);
                return Ok(());
            }
        }
        
        // Add prompt variables to context
        for var in &prompt.variables {
            if !context.contains_key(&var.name) {
                if let Some(default) = &var.default {
                    context.insert(var.name.clone(), default.clone());
                }
            }
        }
        
        // Render templates with inheritance
        let system_prompt = self.render_template_with_inheritance(
            &prompt.system_template,
            prompt.inherits.as_ref(),
            cascade,
            context,
            true,
        )?;
        
        let user_prompt = self.render_template_with_inheritance(
            &prompt.user_template,
            prompt.inherits.as_ref(),
            cascade,
            context,
            false,
        )?;
        
        debug!("System prompt: {}", system_prompt);
        debug!("User prompt: {}", user_prompt);
        
        if self.dry_run {
            info!("Dry run - would execute prompt: {}", prompt.id);
            return Ok(());
        }
        
        // Build the full prompt with response format if specified
        let mut full_user_prompt = user_prompt;
        if let Some(response_format) = &prompt.response_format {
            full_user_prompt.push_str("\n\n");
            match &response_format.format_type {
                crate::metaprompt::ResponseFormatType::Structured { schema } => {
                    full_user_prompt.push_str(&format!("Please respond with valid JSON matching this schema:\n{}", schema));
                }
                crate::metaprompt::ResponseFormatType::Json => {
                    full_user_prompt.push_str("Please respond with valid JSON format.");
                }
                crate::metaprompt::ResponseFormatType::Markdown => {
                    full_user_prompt.push_str("Please format your response as valid Markdown.");
                }
                crate::metaprompt::ResponseFormatType::Code { language } => {
                    full_user_prompt.push_str(&format!("Please respond with only valid {} code, no explanations.", language));
                }
                crate::metaprompt::ResponseFormatType::Yaml => {
                    full_user_prompt.push_str("Please respond with valid YAML format.");
                }
                crate::metaprompt::ResponseFormatType::Toml => {
                    full_user_prompt.push_str("Please respond with valid TOML format.");
                }
                crate::metaprompt::ResponseFormatType::PlainText => {
                    // No special formatting needed for plain text
                }
            }
            
            if response_format.validate {
                full_user_prompt.push_str("\nEnsure the response is syntactically valid and can be parsed.");
            }
        }
        
        // Execute with OpenAI
        let response = self.call_openai(&system_prompt, &full_user_prompt).await?;
        
        // Validate response format if requested
        if let Some(response_format) = &prompt.response_format {
            if response_format.validate {
                self.validate_response_format(&response, &response_format.format_type)?;
            }
        }
        
        // Process response based on output type
        self.process_response(prompt, &response, output_dir, context).await?;
        
        // Cache if idempotent
        if prompt.idempotent {
            let cache_key = self.render_cache_key(prompt, context)?;
            self.save_to_cache(&cache_key, &response).await?;
        }
        
        Ok(())
    }
    
    /// Render template with inheritance support
    fn render_template_with_inheritance(
        &mut self,
        template: &str,
        parent_id: Option<&String>,
        cascade: &PromptCascade,
        context: &HashMap<String, serde_json::Value>,
        is_system: bool,
    ) -> Result<String> {
        let mut full_template = template.to_string();
        
        // Handle inheritance
        if let Some(parent_id) = parent_id {
            if let Some(parent) = cascade.prompts.get(parent_id) {
                let parent_template = if is_system {
                    &parent.system_template
                } else {
                    &parent.user_template
                };
                
                // Replace {{ super() }} with parent template
                full_template = full_template.replace("{{ super() }}", parent_template);
            }
        }
        
        // Render with minijinja
        let tmpl = self.env.template_from_str(&full_template)?;
        let ctx = Value::from_serialize(context);
        Ok(tmpl.render(&ctx)?)
    }
    
    /// Call OpenAI API
    async fn call_openai(&self, system_prompt: &str, user_prompt: &str) -> Result<String> {
        let request = CreateChatCompletionRequestArgs::default()
            .model("gpt-4-turbo-preview")
            .messages([
                ChatCompletionRequestMessage::System(
                    ChatCompletionRequestSystemMessageArgs::default()
                        .content(system_prompt)
                        .build()?
                ),
                ChatCompletionRequestMessage::User(
                    ChatCompletionRequestUserMessageArgs::default()
                        .content(user_prompt)
                        .build()?
                ),
            ])
            .temperature(0.7)
            .build()?;
        
        let response = self.client.chat().create(request).await?;
        
        Ok(response.choices[0].message.content.clone().unwrap_or_default())
    }
    
    /// Process response based on output type
    async fn process_response(
        &self,
        prompt: &MetaPrompt,
        response: &str,
        output_dir: &Path,
        context: &mut HashMap<String, serde_json::Value>,
    ) -> Result<()> {
        match &prompt.output_type {
            OutputType::MetaPrompt => {
                // Parse and save as TOML
                let path = output_dir.join(format!("{}.toml", prompt.id));
                fs::create_dir_all(path.parent().unwrap()).await?;
                fs::write(&path, response).await?;
                info!("Saved meta-prompt to {:?}", path);
            }
            OutputType::Prompt => {
                // Save as TOML prompt file
                let path = output_dir.join(format!("{}.toml", prompt.id));
                fs::create_dir_all(path.parent().unwrap()).await?;
                fs::write(&path, response).await?;
                info!("Saved prompt to {:?}", path);
            }
            OutputType::Code { language } => {
                // Save as code file
                let ext = match language.as_str() {
                    "rust" => "rs",
                    "toml" => "toml",
                    _ => language.as_str(),
                };
                let path = output_dir.join(format!("{}.{}", prompt.id, ext));
                fs::create_dir_all(path.parent().unwrap()).await?;
                fs::write(&path, response).await?;
                info!("Saved code to {:?}", path);
            }
            OutputType::Data { format } => {
                // Save as data file
                let ext = match format {
                    crate::metaprompt::DataFormat::Json => "json",
                    crate::metaprompt::DataFormat::Yaml => "yaml",
                    crate::metaprompt::DataFormat::Toml => "toml",
                    crate::metaprompt::DataFormat::Ron => "ron",
                };
                let path = output_dir.join(format!("{}.{}", prompt.id, ext));
                fs::create_dir_all(path.parent().unwrap()).await?;
                fs::write(&path, response).await?;
                info!("Saved data to {:?}", path);
            }
            OutputType::Asset { asset_type } => {
                // Save asset specification
                let dir = match asset_type {
                    crate::metaprompt::AssetType::Sprite => "sprites",
                    crate::metaprompt::AssetType::Audio => "audio",
                    crate::metaprompt::AssetType::Level => "levels",
                    crate::metaprompt::AssetType::Tilemap => "tilemaps",
                };
                let path = output_dir.join(dir).join(format!("{}.yaml", prompt.id));
                fs::create_dir_all(path.parent().unwrap()).await?;
                fs::write(&path, response).await?;
                info!("Saved asset spec to {:?}", path);
            }
            OutputType::Documentation => {
                // Save as markdown
                let path = output_dir.join(format!("{}.md", prompt.id));
                fs::write(&path, response).await?;
                info!("Saved documentation to {:?}", path);
            }
        }
        
        // Store response in context for child prompts
        context.insert(
            format!("{}_response", prompt.id),
            serde_json::Value::String(response.to_string()),
        );
        
        Ok(())
    }
    
    /// Render cache key
    fn render_cache_key(
        &mut self,
        prompt: &MetaPrompt,
        context: &HashMap<String, serde_json::Value>,
    ) -> Result<String> {
        if let Some(template) = &prompt.cache_key_template {
            let tmpl = self.env.template_from_str(template)?;
            let ctx = Value::from_serialize(context);
            Ok(tmpl.render(&ctx)?)
        } else {
            // Default cache key
            Ok(format!("{}_v{}", prompt.id, context.get("version").and_then(|v| v.as_str()).unwrap_or("1")))
        }
    }
    
    /// Check if result is cached
    async fn check_cache(&self, cache_key: &str) -> Result<bool> {
        let path = self.cache_dir.join(format!("{}.cache", cache_key));
        Ok(path.exists())
    }
    
    /// Save to cache
    async fn save_to_cache(&self, cache_key: &str, content: &str) -> Result<()> {
        let path = self.cache_dir.join(format!("{}.cache", cache_key));
        fs::create_dir_all(path.parent().unwrap()).await?;
        fs::write(&path, content).await?;
        Ok(())
    }

    /// Validate response format
    fn validate_response_format(&self, response: &str, format_type: &crate::metaprompt::ResponseFormatType) -> Result<()> {
        match format_type {
            crate::metaprompt::ResponseFormatType::Structured { .. } |
            crate::metaprompt::ResponseFormatType::Json => {
                serde_json::from_str::<serde_json::Value>(response)
                    .context("Response is not valid JSON")?;
            }
            crate::metaprompt::ResponseFormatType::Yaml => {
                serde_yaml::from_str::<serde_yaml::Value>(response)
                    .context("Response is not valid YAML")?;
            }
            crate::metaprompt::ResponseFormatType::Toml => {
                toml::from_str::<toml::Value>(response)
                    .context("Response is not valid TOML")?;
            }
            _ => {} // Other formats don't need validation
        }
        Ok(())
    }
}