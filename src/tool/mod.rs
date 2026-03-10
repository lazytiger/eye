//! Tool abstraction module
//!
//! This module provides the core abstractions for tools that can be invoked by LLMs.
//! Tools are capabilities that models can use to perform actions in the real world.

use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Represents the outcome of a tool execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExecuteResult {
    /// The execution was successful. Contains the result data.
    Success(MessageContent),
    /// The execution failed. Contains the error message or details.
    Failure(String),
}

/// Represents a tool independently of any specific model provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    /// Unique name of the tool
    pub name: String,
    /// Description of what the tool does
    pub description: String,
    /// JSON Schema for the arguments the tool accepts
    pub parameters: Value,
}

/// Core trait for tools that can be invoked by LLMs
#[async_trait]
pub trait Tool: Send + Sync {
    /// Returns the unique name of the tool (e.g., "get_weather", "search_web").
    /// This name is what the LLM uses to invoke the tool.
    fn name(&self) -> &str;

    /// Returns a description of what the tool does.
    /// This helps the LLM understand when to use the tool.
    fn description(&self) -> &str;

    /// Returns the JSON Schema for the arguments the tool accepts.
    /// This defines the structure of the input the LLM must provide.
    fn parameters(&self) -> Value;

    /// Executes the tool logic with the given arguments.
    /// Returns an ExecuteResult which can be Success or Failure.
    async fn execute(&self, args: Value) -> Result<ExecuteResult>;

    /// Returns the independent definition of the tool.
    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: self.name().to_string(),
            description: self.description().to_string(),
            parameters: self.parameters(),
        }
    }
}

/// Shell tool for executing commands
pub mod shell;

/// Time tool for getting current local time
pub mod time;

/// Search tool for web searching
pub mod search;

/// Web Fetch tool for retrieving webpage content
pub mod web_fetch;

/// File Read tool for reading file contents
pub mod file_read;

/// File Write tool for writing content to files
pub mod file_write;

/// File Search tool for finding files by pattern
pub mod file_search;

/// Re-export shell tool
pub use shell::ShellTool;


/// Re-export time tool
pub use time::TimeTool;


/// Re-export search tool
pub use search::SearchTool;


use crate::provider::MessageContent;
/// Re-export web fetch tool
pub use web_fetch::WebFetchTool;

/// Re-export file read tool
pub use file_read::FileReadTool;

/// Re-export file write tool
pub use file_write::FileWriteTool;

/// Re-export file search tool
pub use file_search::FileSearchTool;

use std::collections::HashMap;
use std::sync::Arc;

/// ToolManager - manages registered tools and provides execution capabilities
#[derive(Default)]
pub struct ToolManager {
    tools: HashMap<String, Arc<dyn Tool>>,
}

impl ToolManager {
    /// Create a new ToolManager with all built-in tools registered
    pub fn new() -> Self {
        let mut manager = Self {
            tools: HashMap::new(),
        };
        // Register all built-in tools
        manager.register_tool(Arc::new(ShellTool::new()));
        manager.register_tool(Arc::new(TimeTool::new()));
        manager.register_tool(Arc::new(SearchTool::default()));
        manager.register_tool(Arc::new(WebFetchTool::new()));
        manager.register_tool(Arc::new(FileReadTool::new()));
        manager.register_tool(Arc::new(FileWriteTool::new()));
        manager.register_tool(Arc::new(FileSearchTool::new()));
        manager
    }

    /// Register a tool
    pub fn register_tool(&mut self, tool: Arc<dyn Tool>) {
        self.tools.insert(tool.name().to_string(), tool);
    }

    /// Get a tool by name
    pub fn get_tool(&self, name: &str) -> Option<Arc<dyn Tool>> {
        self.tools.get(name).cloned()
    }

    /// List all registered tool names
    pub fn list_tools(&self) -> Vec<String> {
        self.tools.keys().cloned().collect()
    }

    /// Check if a tool is registered
    pub fn has_tool(&self, name: &str) -> bool {
        self.tools.contains_key(name)
    }

    /// Get all tool definitions
    pub fn get_tool_definitions(&self) -> Vec<ToolDefinition> {
        self.tools.values().map(|t| t.definition()).collect()
    }

    /// Execute a tool by name with the given arguments
    pub async fn execute_tool(&self, name: &str, args: Value) -> Result<ExecuteResult> {
        let tool = self
            .get_tool(name)
            .ok_or_else(|| anyhow::anyhow!("Tool not found: {}", name))?;
        tool.execute(args).await
    }
}
