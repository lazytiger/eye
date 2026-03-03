//! Command-line interface implementation
//!
//! CLI implementation of the Interface trait using console I/O

use super::{Interface, MessageRole, Usage};
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

    /// Print colored text
    fn print_colored(&self, text: &str, color_code: &str) {
        if self.config.enable_colors {
            // Map ANSI color codes to console styles
            let styled_text = match color_code {
                "34" => style(text).blue(),    // Blue
                "32" => style(text).green(),   // Green
                "33" => style(text).yellow(),  // Yellow
                "35" => style(text).magenta(), // Purple/Magenta
                "36" => style(text).cyan(),    // Cyan
                "31" => style(text).red(),     // Red
                "1" => style(text).bold(),     // Bold
                _ => style(text),
            };
            print!("{}", styled_text);
        } else {
            print!("{}", text);
        }
        self.term.flush().unwrap();
    }

    /// Print timestamp
    fn print_timestamp(&self) {
        if self.config.show_timestamp {
            use std::time::{SystemTime, UNIX_EPOCH};

            let now = SystemTime::now();
            let duration = now.duration_since(UNIX_EPOCH).unwrap_or_default();
            let seconds = duration.as_secs();
            let hours = (seconds / 3600) % 24;
            let minutes = (seconds / 60) % 60;
            let secs = seconds % 60;

            print!("[{:02}:{:02}:{:02}] ", hours, minutes, secs);
            self.term.flush().unwrap();
        }
    }
}

#[async_trait::async_trait]
impl Interface for CliInterface {
    async fn display_message(&self, message: &str, role: MessageRole) -> Result<()> {
        self.print_timestamp();

        match role {
            MessageRole::User => {
                self.print_colored("User: ", "34"); // Blue
                self.term.write_line(message)?;
            }
            MessageRole::Assistant => {
                self.print_colored("Assistant: ", "32"); // Green
                self.term.write_line(message)?;
            }
            MessageRole::System => {
                self.print_colored("System: ", "33"); // Yellow
                self.term.write_line(message)?;
            }
            MessageRole::Tool => {
                self.print_colored("Tool: ", "35"); // Purple
                self.term.write_line(message)?;
            }
        }

        Ok(())
    }

    async fn display_tool_call(
        &self,
        tool_name: &str,
        arguments: &serde_json::Value,
    ) -> Result<()> {
        self.print_timestamp();
        self.print_colored("Tool call: ", "36"); // Cyan
        self.term.write_line(tool_name)?;

        if let Ok(args_str) = serde_json::to_string_pretty(arguments) {
            self.term.write_line(&format!("Args: {}", args_str))?;
        }

        Ok(())
    }

    async fn display_tool_result(
        &self,
        tool_name: &str,
        result: &serde_json::Value,
        success: bool,
    ) -> Result<()> {
        self.print_timestamp();

        if success {
            self.print_colored("Tool result: ", "32"); // Green
        } else {
            self.print_colored("Tool error: ", "31"); // Red
        }

        self.term.write_line(tool_name)?;

        if let Ok(result_str) = serde_json::to_string_pretty(result) {
            self.term.write_line(&format!("Result: {}", result_str))?;
        }

        Ok(())
    }

    async fn get_user_input(&self) -> Result<String> {
        self.print_timestamp();
        self.print_colored(&self.config.prompt, "1"); // Bold

        self.term.flush()?;

        // Use console's read_line with initial text for better UX
        let input = self.term.read_line_initial_text("")?;

        Ok(input.trim().to_string())
    }

    async fn clear_screen(&self) -> Result<()> {
        self.term.clear_screen()?;
        Ok(())
    }

    async fn display_error(&self, error: &str) -> Result<()> {
        self.print_timestamp();
        self.print_colored("Error: ", "31"); // Red
        self.term.write_line(error)?;

        Ok(())
    }

    async fn display_info(&self, info: &str) -> Result<()> {
        self.print_timestamp();
        self.print_colored("Info: ", "36"); // Cyan
        self.term.write_line(info)?;

        Ok(())
    }

    async fn display_usage(&self, usage: &Usage) -> Result<()> {
        self.print_timestamp();
        self.print_colored("Token usage: ", "33"); // Yellow
        self.term.write_line(&format!(
            "Prompt: {}, Completion: {}, Total: {}",
            usage.prompt_tokens, usage.completion_tokens, usage.total_tokens
        ))?;

        Ok(())
    }

    async fn send(&self, message: String) -> Result<()> {
        // Display as assistant message by default
        self.display_message(&message, MessageRole::Assistant).await
    }

    async fn listen(&self, response_tx: Sender<String>) -> Result<()> {
        // Start listening loop
        loop {
            let input = self.get_user_input().await?;

            // Send input to agent for processing
            if response_tx.send(input).await.is_err() {
                break; // Channel closed, stop listening
            }
        }

        Ok(())
    }
}
