//! Interface module
//!
//! Manages user interfaces, including:
//! - Base interface trait definition
//! - Interface trait definition
//! - CLI interface implementation
//! - Interface manager

//! Interface trait definition
//!
//! Defines the Interface trait to abstract different user interfaces

use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;
use tokio::sync::mpsc::Sender;

/// Message role
#[derive(Debug, Clone, PartialEq)]
pub enum MessageRole {
    /// System message
    System,
    /// User message
    User,
    /// Assistant message
    Assistant,
    /// Tool message
    Tool,
}

/// Token usage
#[derive(Debug, Clone)]
pub struct Usage {
    /// Prompt token count
    pub prompt_tokens: u32,
    /// Completion token count
    pub completion_tokens: u32,
    /// Total token count
    pub total_tokens: u32,
}

/// Interface trait
#[async_trait]
pub trait Interface: Send + Sync {
    /// Display a message
    async fn display_message(&self, message: &str, role: MessageRole) -> Result<()>;

    /// Display a tool call
    async fn display_tool_call(&self, tool_name: &str, arguments: &Value) -> Result<()>;

    /// Display a tool result
    async fn display_tool_result(
        &self,
        tool_name: &str,
        result: &Value,
        success: bool,
    ) -> Result<()>;

    /// Get user input
    async fn get_user_input(&self) -> Result<String>;

    /// Clear screen
    async fn clear_screen(&self) -> Result<()>;

    /// Display an error message
    async fn display_error(&self, error: &str) -> Result<()>;

    /// Display an info message
    async fn display_info(&self, info: &str) -> Result<()>;

    /// Display token usage
    async fn display_usage(&self, usage: &Usage) -> Result<()>;

    /// Send a message to the interface
    async fn send(&self, message: String) -> Result<()>;

    /// Listen for input from the interface
    /// Returns a channel sender that will receive processed responses
    async fn listen(&self, response_tx: Sender<String>) -> Result<()>;
}

pub mod cli;

use crate::config::settings::InterfaceConfig;
use crate::interface::cli::CliInterface;

/// Create interface
///
/// Create corresponding interface instance based on configuration
pub fn create_interface(config: &InterfaceConfig) -> Box<dyn Interface> {
    let interface = CliInterface::new(config.clone());
    Box::new(interface)
}
