//! Tool abstraction module
//!
//! This module provides the core abstractions for tools that can be invoked by LLMs.
//! Tools are capabilities that models can use to perform actions in the real world.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use anyhow::Result;

/// Represents the outcome of a tool execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExecuteResult {
    /// The execution was successful. Contains the result data.
    Success(Value),
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

/// Represents a request from the LLM to execute a tool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    /// Unique identifier for this tool call
    pub id: String,
    /// Name of the tool to execute
    pub name: String,
    /// Arguments for the tool (JSON object)
    pub arguments: Value,
}

/// Represents the result of a tool execution to be sent back to the LLM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolOutput {
    /// Matches the ToolCall id
    pub id: String,
    /// Stringified JSON or text result
    pub content: String,
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

/// Re-export shell tool
pub use shell::ShellTool;