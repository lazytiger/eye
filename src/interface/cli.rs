//! Command-line interface implementation
//!
//! CLI implementation of the Interface trait using console I/O

use super::Interface;
use crate::config::settings::InterfaceConfig;
use anyhow::Result;
use console::{style, Term};
use tokio::sync::mpsc::Sender;

/// CLI interface
pub struct CliInterface {
    /// Configuration
    config: InterfaceConfig,
    /// Console terminal
    term: Term,
}

impl CliInterface {
    /// Create a new CLI interface
    pub fn new(config: InterfaceConfig) -> Self {
        Self {
            config,
            term: Term::buffered_stdout(),
        }
    }
}

#[async_trait::async_trait]
impl Interface for CliInterface {
    fn name(&self) -> &str {
        "cli"
    }

    async fn send(&self, message: String) -> Result<()> {
        // Use console style prefix similar to Claude Code
        // ">" character with cyan color for assistant messages
        self.term
            .write_line(&format!("{} {}", style(">").cyan().bold(), message))?;
        self.term.flush()?;
        Ok(())
    }

    async fn listen(&self, response_tx: Sender<String>) -> Result<()> {
        loop {
            let input = self.term.read_line_initial_text("> ")?;
            if input.trim().is_empty() {
                continue;
            }
            response_tx.send(input).await?;
        }
    }
}
