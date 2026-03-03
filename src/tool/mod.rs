//! Tool module
//!
//! Manages the tool system, including:
//! - Tool trait definition
//! - Tool implementations (e.g., Shell command execution)
//! - Tool manager
//! Tool trait definition
//!
//! Defines the Tool trait to abstract different tool capabilities

use async_trait::async_trait;
use serde_json::Value;

use crate::config::settings::ToolsConfig;
use crate::tool::shell::ShellTool;
use anyhow::Result;
use std::{collections::HashMap, sync::Arc};

/// Tool call result
#[derive(Debug, Clone)]
pub struct ToolResult {
    /// Tool call ID
    pub tool_call_id: String,
    /// Tool name
    pub tool_name: String,
    /// Execution result
    pub result: Value,
    /// Whether execution succeeded
    pub success: bool,
    /// Error message (if any)
    pub error: Option<String>,
}

/// Tool definition
#[derive(Debug, Clone)]
pub struct ToolDefinition {
    /// Tool name
    pub name: String,
    /// Tool description
    pub description: String,
    /// Tool parameter schema (JSON Schema)
    pub parameters: Value,
}

/// Tool trait
#[async_trait]
pub trait Tool: Send + Sync {
    /// Get tool definition
    fn definition(&self) -> ToolDefinition;

    /// Execute a tool call
    async fn execute(&self, arguments: Value) -> Result<ToolResult>;

    /// Validate arguments
    fn validate_arguments(&self, arguments: &Value) -> Result<()>;
}

pub mod shell;

/// Tool manager
pub struct ToolManager {
    /// Tool registry
    tools: HashMap<String, Arc<dyn Tool>>,
}

impl ToolManager {
    /// Create new tool manager
    pub fn new(config: &ToolsConfig) -> Self {
        let mut tools: HashMap<String, Arc<dyn Tool>> = HashMap::new();

        // Add tools based on configuration
        for tool_name in &config.enabled {
            match tool_name.as_str() {
                "shell" => {
                    let shell_tool = ShellTool::new(config.shell.clone());
                    tools.insert(
                        "execute_shell_command".to_string(),
                        Arc::new(shell_tool) as Arc<dyn Tool>,
                    );
                }
                _ => {
                    // Ignore unknown tools
                }
            }
        }

        Self { tools }
    }

    /// Get all tool definitions
    pub fn get_tool_definitions(&self) -> Vec<ToolDefinition> {
        self.tools.values().map(|tool| tool.definition()).collect()
    }

    /// Execute a tool call
    pub async fn execute_tool(
        &self,
        tool_name: &str,
        tool_call_id: &str,
        arguments: serde_json::Value,
    ) -> Result<ToolResult> {
        let tool = self
            .tools
            .get(tool_name)
            .ok_or_else(|| anyhow::anyhow!("Tool '{}' not found", tool_name))?;

        let mut result = tool.execute(arguments).await?;
        result.tool_call_id = tool_call_id.to_string();

        Ok(result)
    }

    /// Get tool list
    pub fn list_tools(&self) -> Vec<String> {
        self.tools.keys().cloned().collect()
    }

    /// Check if a tool exists
    pub fn has_tool(&self, tool_name: &str) -> bool {
        self.tools.contains_key(tool_name)
    }
}
