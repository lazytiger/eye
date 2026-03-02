//! Base interface trait definition
//!
//! Defines the base Interface trait with minimal send/listen methods

use anyhow::Result;
use async_trait::async_trait;
use tokio::sync::mpsc::Sender;

/// Base interface trait
#[async_trait]
pub trait BaseInterface: Send + Sync {
    /// Send a message to the interface
    async fn send(&self, message: String) -> Result<()>;
    
    /// Listen for input from the interface
    /// Returns a channel sender that will receive processed responses
    async fn listen(&self, response_tx: Sender<String>) -> Result<()>;
}