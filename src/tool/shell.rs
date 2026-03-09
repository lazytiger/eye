//! Shell tool implementation
//!
//! This module provides shell command execution capabilities.

use async_trait::async_trait;
use serde_json::{json, Value};
use anyhow::Result;
use super::{ExecuteResult, Tool};
use crate::provider::MessageContent;
use tokio::process::Command;
use std::time::Duration;

/// Shell tool for executing commands
pub struct ShellTool;

impl ShellTool {
    /// Create a new shell tool
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Tool for ShellTool {
    fn name(&self) -> &str {
        "shell"
    }

    fn description(&self) -> &str {
        "Execute shell commands on the system and return the output"
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "command": {
                    "type": "string",
                    "description": "The shell command to execute"
                }
            },
            "required": ["command"]
        })
    }

    async fn execute(&self, args: Value) -> Result<ExecuteResult> {
        // Extract command from arguments
        let command = args.get("command")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing 'command' parameter"))?;

        // Determine shell command based on platform
        let (shell_cmd, shell_arg) = if cfg!(windows) {
            ("cmd", "/C")
        } else {
            ("sh", "-c")
        };

        // Execute command with timeout
        let output = tokio::time::timeout(
            Duration::from_secs(30),
            Command::new(shell_cmd).arg(shell_arg).arg(command).output()
        )
        .await
        .map_err(|_| anyhow::anyhow!("Command execution timed out"))??;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        let result = if output.status.success() {
            if stdout.is_empty() {
                "Command executed successfully (no output)".to_string()
            } else {
                format!("Exit code: {}\nOutput:\n{}", output.status, stdout)
            }
        } else {
            format!(
                "Exit code: {}\nStdout: {}\nStderr: {}",
                output.status, stdout, stderr
            )
        };

        Ok(ExecuteResult::Success(MessageContent::Text(result)))
    }
}