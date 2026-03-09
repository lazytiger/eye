//! Command-line interface implementation
//!
//! CLI implementation of the Interface trait using console I/O

use super::Interface;
use crate::config::settings::InterfaceConfig;
use anyhow::Result;
use console::{style, Term};
use std::io::Write;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;
use tokio::sync::RwLock;

/// CLI interface
pub struct CliInterface {
    /// Configuration
    config: InterfaceConfig,
    arrow: String,
    /// Console terminal
    term: Arc<RwLock<Term>>,
}

impl CliInterface {
    /// Create a new CLI interface
    pub fn new(config: InterfaceConfig) -> Self {
        Self {
            config,
            arrow: format!("{}", style("> ").cyan().bold()),
            term: Arc::new(RwLock::new(Term::buffered_stdout())),
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
        let term = self.term.read().await;
        term.write_line(&format!("{0}{1}", self.arrow, message))?;
        term.flush()?;
        Ok(())
    }

    async fn listen(&self, response_tx: Sender<String>) -> Result<()> {
        loop {
            {
                let mut term = self.term.write().await;
                term.write(self.arrow.as_bytes())?;
                term.flush()?;
            }
            let term = self.term.read().await;
            let input = term.read_line()?;
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
