//! Command-line interface implementation
//!
//! Console implementation of the Interface trait

use super::r#trait::*;
use crate::config::settings::InterfaceConfig;
use anyhow::Result;
use std::io::{self, Write};

/// Console interface
pub struct ConsoleInterface {
    /// Configuration
    config: InterfaceConfig,
}

impl ConsoleInterface {
    /// Create a new console interface
    pub fn new(config: InterfaceConfig) -> Self {
        Self { config }
    }

    /// Print colored text
    fn print_colored(&self, text: &str, color_code: &str) {
        if self.config.enable_colors {
            print!("\x1b[{}m{}\x1b[0m", color_code, text);
        } else {
            print!("{}", text);
        }
        io::stdout().flush().unwrap();
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
        }
    }
}

#[async_trait::async_trait]
impl Interface for ConsoleInterface {
    async fn display_message(&self, message: &str, role: MessageRole) -> Result<()> {
        self.print_timestamp();

        match role {
            MessageRole::User => {
                self.print_colored("User: ", "34"); // Blue
                println!("{}", message);
            }
            MessageRole::Assistant => {
                self.print_colored("Assistant: ", "32"); // Green
                println!("{}", message);
            }
            MessageRole::System => {
                self.print_colored("System: ", "33"); // Yellow
                println!("{}", message);
            }
            MessageRole::Tool => {
                self.print_colored("Tool: ", "35"); // Purple
                println!("{}", message);
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
        println!("{}", tool_name);

        if let Ok(args_str) = serde_json::to_string_pretty(arguments) {
            println!("Args: {}", args_str);
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

        println!("{}", tool_name);

        if let Ok(result_str) = serde_json::to_string_pretty(result) {
            println!("Result: {}", result_str);
        }

        Ok(())
    }

    async fn get_user_input(&self) -> Result<String> {
        self.print_timestamp();
        self.print_colored(&self.config.prompt, "1"); // Bold

        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        Ok(input.trim().to_string())
    }

    async fn clear_screen(&self) -> Result<()> {
        if cfg!(windows) {
            std::process::Command::new("cmd")
                .args(["/c", "cls"])
                .status()?;
        } else {
            std::process::Command::new("clear").status()?;
        }

        Ok(())
    }

    async fn display_error(&self, error: &str) -> Result<()> {
        self.print_timestamp();
        self.print_colored("Error: ", "31"); // Red
        println!("{}", error);

        Ok(())
    }

    async fn display_info(&self, info: &str) -> Result<()> {
        self.print_timestamp();
        self.print_colored("Info: ", "36"); // Cyan
        println!("{}", info);

        Ok(())
    }

    async fn display_usage(&self, usage: &Usage) -> Result<()> {
        self.print_timestamp();
        self.print_colored("Token usage: ", "33"); // Yellow
        println!(
            "Prompt: {}, Completion: {}, Total: {}",
            usage.prompt_tokens, usage.completion_tokens, usage.total_tokens
        );

        Ok(())
    }
}
