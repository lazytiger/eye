//! Shell command execution tool
//!
//! Tool implementation for executing shell commands

use super::r#trait::*;
use crate::config::settings::ShellConfig;
use anyhow::{Context, Result};
use serde_json::{Value, json};
use std::{
    process::{Command, Stdio},
    time::Duration,
};
use tokio::time::timeout;

/// Shell command execution tool
pub struct ShellTool {
    /// Configuration
    config: ShellConfig,
}

impl ShellTool {
    /// Create a new Shell tool
    pub fn new(config: ShellConfig) -> Self {
        Self { config }
    }

    /// Check if a command is allowed to run
    fn is_command_allowed(&self, command: &str) -> bool {
        if self.config.allow_any_command {
            return true;
        }

        // Parse command and arguments
        let parts: Vec<&str> = command.split_whitespace().collect();
        if parts.is_empty() {
            return false;
        }

        let cmd_name = parts[0];
        self.config
            .allowed_commands
            .iter()
            .any(|allowed| allowed == cmd_name)
    }

    /// Execute command
    async fn execute_command(&self, command: &str) -> Result<(String, String, i32)> {
        // Check if command is allowed
        if !self.is_command_allowed(command) {
            return Err(anyhow::anyhow!(
                "Command '{}' is not in the allowed list",
                command
            ));
        }

        // Use PowerShell on Windows
        let shell = if cfg!(windows) { "powershell" } else { "sh" };

        let args = if cfg!(windows) {
            vec!["-Command", command]
        } else {
            vec!["-c", command]
        };

        // Execute command
        let output = timeout(Duration::from_secs(self.config.timeout_seconds), async {
            Command::new(shell)
                .args(&args)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output()
                .context("Failed to execute command")
        })
        .await
        .context("Command execution timed out")??;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let exit_code = output.status.code().unwrap_or(-1);

        Ok((stdout, stderr, exit_code))
    }
}

#[async_trait::async_trait]
impl Tool for ShellTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: "execute_shell_command".to_string(),
            description: "Execute a shell command and return the result. Supports basic commands such as ls, pwd, echo, etc.".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "command": {
                        "type": "string",
                        "description": "The shell command to execute"
                    }
                },
                "required": ["command"]
            }),
        }
    }

    async fn execute(&self, arguments: Value) -> Result<ToolResult> {
        // Parse arguments
        let command = arguments
            .get("command")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing 'command' parameter"))?;

        // Validate arguments
        self.validate_arguments(&arguments)?;

        // Execute command
        let (stdout, stderr, exit_code) = self.execute_command(command).await?;

        // Build result
        let success = exit_code == 0;
        let result = json!({
            "command": command,
            "stdout": stdout,
            "stderr": stderr,
            "exit_code": exit_code,
            "success": success,
        });

        Ok(ToolResult {
            tool_call_id: "".to_string(), // will be set at call time
            tool_name: "execute_shell_command".to_string(),
            result,
            success,
            error: if success {
                None
            } else {
                Some(format!(
                    "Command execution failed, exit code: {}",
                    exit_code
                ))
            },
        })
    }

    fn validate_arguments(&self, arguments: &Value) -> Result<()> {
        // Check if 'command' parameter exists
        if arguments.get("command").and_then(|v| v.as_str()).is_none() {
            return Err(anyhow::anyhow!("Missing 'command' parameter"));
        }

        // Check if command is allowed
        let command = arguments.get("command").unwrap().as_str().unwrap();
        if !self.is_command_allowed(command) {
            return Err(anyhow::anyhow!(
                "Command '{}' is not in the allowed list",
                command
            ));
        }

        Ok(())
    }
}
