use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A meta-prompt that can generate other prompts or content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaPrompt {
    /// Unique identifier for this prompt
    pub id: String,
    
    /// Parent prompt ID for inheritance
    pub inherits: Option<String>,
    
    /// Type of output this prompt generates
    pub output_type: OutputType,
    
    /// Variables that can be passed to this prompt
    pub variables: Vec<Variable>,
    
    /// The system prompt template (supports Jinja2)
    pub system_template: String,
    
    /// The user prompt template (supports Jinja2)
    pub user_template: String,
    
    /// Expected response format/schema
    pub response_format: Option<ResponseFormat>,
    
    /// Child prompts this can trigger
    pub children: Vec<String>,
    
    /// Dependencies on other prompts
    pub depends_on: Vec<String>,
    
    /// Tags for categorization
    pub tags: Vec<String>,
    
    /// Whether this prompt is idempotent
    pub idempotent: bool,
    
    /// Cache key template for idempotency
    pub cache_key_template: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutputType {
    /// Generates another meta-prompt
    MetaPrompt,
    /// Generates a regular prompt
    Prompt,
    /// Generates code
    Code { language: String },
    /// Generates data/configuration
    Data { format: DataFormat },
    /// Generates assets
    Asset { asset_type: AssetType },
    /// Generates documentation
    Documentation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DataFormat {
    Json,
    Yaml,
    Toml,
    Ron,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssetType {
    Sprite,
    Audio,
    Level,
    Tilemap,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Variable {
    pub name: String,
    pub var_type: VariableType,
    pub required: bool,
    pub default: Option<serde_json::Value>,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VariableType {
    String,
    Number,
    Boolean,
    Array,
    Object,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseFormat {
    pub format_type: ResponseFormatType,
    #[serde(default)]
    pub validate: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResponseFormatType {
    Json,
    Yaml,
    Toml,
    Markdown,
    Code { language: String },
    Structured { schema: serde_json::Value },
    PlainText,
}

/// A cascade of meta-prompts forming a DAG
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptCascade {
    pub name: String,
    pub description: String,
    pub version: String,
    pub prompts: HashMap<String, MetaPrompt>,
    pub root_prompt: String,
    pub global_variables: HashMap<String, serde_json::Value>,
}

impl PromptCascade {
    /// Build a DAG from the prompt definitions
    pub fn build_dag(&self) -> Result<petgraph::graph::DiGraph<String, ()>, anyhow::Error> {
        use petgraph::graph::DiGraph;
        use std::collections::HashMap;
        
        let mut graph = DiGraph::new();
        let mut node_map = HashMap::new();
        
        // Add all nodes
        for (id, _) in &self.prompts {
            let node = graph.add_node(id.clone());
            node_map.insert(id.clone(), node);
        }
        
        // Add edges based on dependencies and children
        for (id, prompt) in &self.prompts {
            let from_node = node_map[id];
            
            // Add edges for children
            for child_id in &prompt.children {
                if let Some(&to_node) = node_map.get(child_id) {
                    graph.add_edge(from_node, to_node, ());
                }
            }
            
            // Add edges for dependencies (reversed direction)
            for dep_id in &prompt.depends_on {
                if let Some(&dep_node) = node_map.get(dep_id) {
                    graph.add_edge(dep_node, from_node, ());
                }
            }
        }
        
        // Check for cycles
        if petgraph::algo::is_cyclic_directed(&graph) {
            anyhow::bail!("Prompt cascade contains cycles!");
        }
        
        Ok(graph)
    }
    
    /// Get execution order using topological sort
    pub fn get_execution_order(&self) -> Result<Vec<String>, anyhow::Error> {
        let dag = self.build_dag()?;
        let mut order = Vec::new();
        
        // Use topological sort
        use petgraph::visit::Topo;
        let mut topo = Topo::new(&dag);
        
        while let Some(node) = topo.next(&dag) {
            order.push(dag[node].clone());
        }
        
        Ok(order)
    }
}