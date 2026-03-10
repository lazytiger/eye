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
use tokio::sync::Mutex;

/// CLI interface
pub struct CliInterface {
    /// Configuration
    config: InterfaceConfig,
    arrow: String,
    /// Console terminal for output (protected by mutex for thread-safe writes)
    term: Arc<Mutex<Term>>,
}

impl CliInterface {
    /// Create a new CLI interface
    pub fn new(config: InterfaceConfig) -> Self {
        Self {
            config,
            arrow: format!("{}", style("> ").cyan().bold()),
            term: Arc::new(Mutex::new(Term::buffered_stdout())),
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
        let mut term = self.term.lock().await;
        term.write(format!("{0}{1}\n{0}", self.arrow, message).as_bytes())?;
        term.flush()?;
        Ok(())
    }

    async fn listen(&self, response_tx: Sender<String>) -> Result<()> {
        // Use tokio::task::spawn_blocking for synchronous read_line
        loop {
            // Read line in a blocking task to avoid blocking the async runtime
            let input = tokio::task::spawn_blocking(|| {
                let term = Term::buffered_stdout();
                term.read_line()
            }).await??;

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
