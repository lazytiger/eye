//! Configuration struct definitions
//!
//! Defines application configuration structures with TOML support

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Application settings
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Settings {
    /// OpenRouter configuration
    pub openrouter: OpenRouterConfig,

    /// Model configuration
    pub model: ModelConfig,

    /// Tools configuration
    pub tools: ToolsConfig,

    /// Interface configuration
    pub interface: InterfaceConfig,
}

/// OpenRouter configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenRouterConfig {
    /// API Key
    pub api_key: String,

    /// API endpoint
    #[serde(default = "default_openrouter_endpoint")]
    pub endpoint: String,

    /// Default model
    #[serde(default = "default_model")]
    pub default_model: String,
}

/// Model configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    /// Temperature
    #[serde(default = "default_temperature")]
    pub temperature: f32,

    /// Max tokens
    #[serde(default = "default_max_tokens")]
    pub max_tokens: u32,

    /// Enable streaming output
    #[serde(default = "default_stream")]
    pub stream: bool,
}

/// Tools configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolsConfig {
    /// Enabled tools
    #[serde(default)]
    pub enabled: Vec<String>,

    /// Shell tool configuration
    #[serde(default)]
    pub shell: ShellConfig,
}

/// Shell tool configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShellConfig {
    /// Allowed command list
    #[serde(default)]
    pub allowed_commands: Vec<String>,

    /// Allow any command
    #[serde(default = "default_allow_any_command")]
    pub allow_any_command: bool,

    /// Timeout in seconds
    #[serde(default = "default_shell_timeout")]
    pub timeout_seconds: u64,
}

/// Interface configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterfaceConfig {
    /// Prompt
    #[serde(default = "default_prompt")]
    pub prompt: String,

    /// Show timestamp
    #[serde(default = "default_show_timestamp")]
    pub show_timestamp: bool,

    /// Enable colors
    #[serde(default = "default_enable_colors")]
    pub enable_colors: bool,
}

impl Settings {
    /// Load configuration
    ///
    /// Load from the specified path, or from the default location if none provided
    pub fn load(config_path: Option<&Path>) -> Result<Self> {
        let config_path = config_path.unwrap_or_else(|| Path::new("eye.toml"));

        if config_path.exists() {
            let config_content = std::fs::read_to_string(config_path).with_context(|| {
                format!("Unable to read config file: {}", config_path.display())
            })?;

            toml::from_str(&config_content)
                .with_context(|| format!("Invalid configuration format: {}", config_path.display()))
        } else {
            // If no config file exists, return defaults
            Ok(Self::default())
        }
    }

    /// Save configuration to file
    pub fn save(&self, config_path: &Path) -> Result<()> {
        let config_content =
            toml::to_string_pretty(self).context("Failed to serialize configuration")?;

        std::fs::write(config_path, config_content)
            .with_context(|| format!("Unable to write config file: {}", config_path.display()))
    }
}

impl Default for OpenRouterConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            endpoint: default_openrouter_endpoint(),
            default_model: default_model(),
        }
    }
}

impl Default for ModelConfig {
    fn default() -> Self {
        Self {
            temperature: default_temperature(),
            max_tokens: default_max_tokens(),
            stream: default_stream(),
        }
    }
}

impl Default for ToolsConfig {
    fn default() -> Self {
        Self {
            enabled: vec!["shell".to_string()],
            shell: ShellConfig::default(),
        }
    }
}

impl Default for ShellConfig {
    fn default() -> Self {
        Self {
            allowed_commands: vec!["ls".to_string(), "pwd".to_string(), "echo".to_string()],
            allow_any_command: default_allow_any_command(),
            timeout_seconds: default_shell_timeout(),
        }
    }
}

impl Default for InterfaceConfig {
    fn default() -> Self {
        Self {
            prompt: default_prompt(),
            show_timestamp: default_show_timestamp(),
            enable_colors: default_enable_colors(),
        }
    }
}

// Default value helpers
fn default_openrouter_endpoint() -> String {
    "https://openrouter.ai/api/v1".to_string()
}

fn default_model() -> String {
    "openai/gpt-4o-mini".to_string()
}

fn default_temperature() -> f32 {
    0.7
}

fn default_max_tokens() -> u32 {
    2048
}

fn default_stream() -> bool {
    true
}

fn default_allow_any_command() -> bool {
    false
}

fn default_shell_timeout() -> u64 {
    30
}

fn default_prompt() -> String {
    "eye> ".to_string()
}

fn default_show_timestamp() -> bool {
    false
}

fn default_enable_colors() -> bool {
    true
}
