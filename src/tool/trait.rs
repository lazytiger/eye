//! Tool trait definition
//!
//! Defines the Tool trait to abstract different tool capabilities

use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;

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
