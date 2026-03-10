//! Command-line interface implementation
//!
//! CLI implementation of the Interface trait using console I/O

use super::Interface;
use crate::config::settings::InterfaceConfig;
use anyhow::Result;
use console::{style, Term};
use std::io::Write;
use tokio::sync::mpsc::Sender;

/// CLI interface
pub struct CliInterface {
    /// Configuration
    config: InterfaceConfig,
    term: Term,
    arrow: String,
}

impl CliInterface {
    /// Create a new CLI interface
    pub fn new(config: InterfaceConfig) -> Self {
        Self {
            config,
            term: Term::stdout(),
            arrow: format!("{}", style("> ").cyan().bold()),
        }
    }
}

#[async_trait::async_trait]
impl Interface for CliInterface {
    fn name(&self) -> &str {
        "cli"
    }

    async fn send(&self, message: String) -> Result<()> {
        let mut term = self.term.clone();
        let message = termimad::term_text(&message);
        term.write(format!("{0}{1}\n{0}", self.arrow, message).as_bytes())?;
        term.flush()?;
        Ok(())
    }

    async fn listen(&self, response_tx: Sender<String>) -> Result<()> {
        // Use tokio::task::spawn_blocking for synchronous read_line
        loop {
            // Read line in a blocking task to avoid blocking the async runtime
            let term_clone = self.term.clone();
            let input = tokio::task::spawn_blocking(move || term_clone.read_line()).await??;

            match input.trim() {
                "" => continue,
                "/exit" => break,
                _ => {
                    response_tx.send(input).await?;
                }
            }
        }
        Ok(())
    }
}
