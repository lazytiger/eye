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
use tokio::sync::mpsc::Sender;

/// Message role
/// Interface trait
#[async_trait]
pub trait Interface: Send + Sync {
    /// Get the name of the interface
    fn name(&self) -> &str;
    /// Send a response to the interface
    async fn send(&self, message: String) -> Result<()>;

    /// Listen for input from the interface infinitely.
    /// When got a message from the interface, send it to the response_tx channel.
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
