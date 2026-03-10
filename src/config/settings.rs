//! Configuration struct definitions
//!
//! Defines application configuration structures with TOML support

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Legacy Settings for backwards compatibility
#[derive(Debug, Clone, Deserialize)]
struct LegacySettings {
    #[serde(default)]
    openrouter: Option<LegacyOpenRouterConfig>,
    #[serde(default)]
    model_routes: Vec<ModelRouteConfig>,
    #[serde(default)]
    active_route: String,
    #[serde(default)]
    tools: ToolsConfig,
    #[serde(default)]
    interface: InterfaceConfig,
    #[serde(default)]
    agent: AgentConfig,
}

#[derive(Debug, Clone, Deserialize)]
struct LegacyOpenRouterConfig {
    api_key: String,
    #[serde(default)]
    default_model: String,
}

impl From<LegacySettings> for Settings {
    fn from(legacy: LegacySettings) -> Self {
        let mut settings = Settings {
            model_routes: legacy.model_routes,
            active_route: legacy.active_route,
            tools: legacy.tools,
            interface: legacy.interface,
            agent: legacy.agent,
        };

        // Migrate legacy openrouter config to model_routes
        if let Some(openrouter) = legacy.openrouter {
            if settings.model_routes.is_empty() {
                settings.model_routes.push(ModelRouteConfig {
                    name: "default".to_string(),
                    provider: "openrouter".to_string(),
                    model: if openrouter.default_model.is_empty() {
                        "openai/gpt-4o-mini".to_string()
                    } else {
                        openrouter.default_model
                    },
                    api_key: openrouter.api_key,
                    temperature: None,
                    max_tokens: None,
                    stream: None,
                });

                if settings.active_route.is_empty() {
                    settings.active_route = "default".to_string();
                }
            }
        }

        settings
    }
}

/// Get the default configuration file path
///
/// Returns the path to `.eye/config.toml` in the user's home directory.
/// Supports Windows, Linux, and macOS.
pub fn get_default_config_path() -> Result<PathBuf> {
    let home_dir =
        home::home_dir().ok_or_else(|| anyhow::anyhow!("Unable to determine home directory"))?;

    let config_dir = home_dir.join(".eye");
    let config_file = config_dir.join("config.toml");

    Ok(config_file)
}

/// Get the default configuration directory
///
/// Returns the path to `.eye` directory in the user's home directory.
pub fn get_default_config_dir() -> Result<PathBuf> {
    let home_dir =
        home::home_dir().ok_or_else(|| anyhow::anyhow!("Unable to determine home directory"))?;

    Ok(home_dir.join(".eye"))
}

/// Application settings
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Settings {
    /// Model routes - array of available model configurations
    #[serde(default)]
    pub model_routes: Vec<ModelRouteConfig>,

    /// Default/active route (references name in model_routes)
    #[serde(default)]
    pub active_route: String,

    /// Tools configuration
    #[serde(default)]
    pub tools: ToolsConfig,

    /// Interface configuration
    #[serde(default)]
    pub interface: InterfaceConfig,

    /// Agent configuration
    #[serde(default)]
    pub agent: AgentConfig,
}

impl Settings {
    /// Load configuration
    ///
    /// Load from the specified path, or from the default location (~/.eye/config.toml) if none provided.
    /// If the config file doesn't exist, it will be created with default values.
    ///
    /// Supports backwards compatibility with legacy [openrouter] config format.
    pub fn load(config_path: Option<&Path>) -> Result<Self> {
        let config_path = config_path
            .map(|p| p.to_path_buf())
            .or_else(|| get_default_config_path().ok())
            .context("Unable to determine configuration file path")?;

        // Check if parent directory exists, create if not
        if let Some(parent) = config_path.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent).with_context(|| {
                    format!("Failed to create config directory: {}", parent.display())
                })?;
            }
        }

        if config_path.exists() {
            let config_content = std::fs::read_to_string(&config_path).with_context(|| {
                format!("Unable to read config file: {}", config_path.display())
            })?;

            // Check if config has legacy [openrouter] section
            let has_openrouter = config_content.contains("[openrouter]");

            // Try to parse as legacy format first if it might have [openrouter]
            if has_openrouter {
                if let Ok(legacy) = toml::from_str::<LegacySettings>(&config_content) {
                    if legacy.openrouter.is_some() {
                        tracing::info!("Loaded configuration (legacy format with openrouter section)");
                        return Ok(legacy.into());
                    }
                }
            }

            // Parse as new format
            let settings: Settings = toml::from_str(&config_content)
                .with_context(|| format!("Invalid configuration format: {}", config_path.display()))?;
            tracing::info!("Loaded configuration (new format with model_routes)");
            Ok(settings)
        } else {
            // Create default configuration and save it
            let settings = Self::default();
            settings.save(&config_path)?;
            tracing::info!(
                "Created default configuration at: {}",
                config_path.display()
            );
            Ok(settings)
        }
    }

    /// Save configuration to file
    pub fn save(&self, config_path: &Path) -> Result<()> {
        let config_content =
            toml::to_string_pretty(self).context("Failed to serialize configuration")?;

        std::fs::write(config_path, config_content)
            .with_context(|| format!("Unable to write config file: {}", config_path.display()))
    }

    /// Get the active route configuration
    pub fn get_active_route(&self) -> Result<&ModelRouteConfig> {
        if self.model_routes.is_empty() {
            return Err(anyhow::anyhow!("No model routes configured. Add at least one [[model_routes]] entry to your config."));
        }

        if self.active_route.is_empty() {
            return Err(anyhow::anyhow!("No active_route specified. Set active_route to one of: {}",
                self.model_routes.iter().map(|r| r.name.as_str()).collect::<Vec<_>>().join(", ")));
        }

        self.model_routes
            .iter()
            .find(|r| r.name == self.active_route)
            .ok_or_else(|| anyhow::anyhow!("Active route '{}' not found in model_routes", self.active_route))
    }

    /// List all available route names
    pub fn list_route_names(&self) -> Vec<&str> {
        self.model_routes.iter().map(|r| r.name.as_str()).collect()
    }

    /// Create provider from active route
    pub fn create_provider_from_active_route(&self) -> Result<Box<dyn crate::provider::Provider>> {
        let route = self.get_active_route()?;
        route.create_provider()
    }
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

/// Agent configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    /// System prompt
    #[serde(default = "default_system_prompt")]
    pub system_prompt: String,
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            system_prompt: default_system_prompt(),
        }
    }
}

fn default_system_prompt() -> String {
    "You are Eye, a helpful personal intelligent assistant.".to_string()
}

/// Model route configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelRouteConfig {
    /// Unique identifier for this route (e.g., "fast", "smart", "budget")
    pub name: String,

    /// Provider name: "openai", "openrouter", "deepseek", or custom "name:endpoint"
    pub provider: String,

    /// Model identifier (e.g., "gpt-4o", "claude-3-opus")
    pub model: String,

    /// Optional API key (can also use env var PROVIDER_API_KEY)
    #[serde(default)]
    pub api_key: String,

    /// Temperature (0-2). Higher values = more random
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    /// Max tokens to generate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,

    /// Enable streaming output
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
}

impl ModelRouteConfig {
    /// Create a provider instance from this route configuration
    pub fn create_provider(&self) -> anyhow::Result<Box<dyn crate::provider::Provider>> {
        crate::provider::create_provider(&self.provider, &self.model, &self.api_key)
    }
}

impl Default for ToolsConfig {
    fn default() -> Self {
        Self {
            enabled: vec![
                "shell".to_string(),
                "time".to_string(),
                "search_web".to_string(),
                "fetch_webpage".to_string(),
            ],
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

impl Default for ModelRouteConfig {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            provider: "openrouter".to_string(),
            model: "openai/gpt-4o-mini".to_string(),
            api_key: String::new(),
            temperature: None,
            max_tokens: None,
            stream: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_route_config_serialization() {
        let route = ModelRouteConfig {
            name: "test".to_string(),
            provider: "openrouter".to_string(),
            model: "gpt-4o".to_string(),
            api_key: "test-key".to_string(),
            temperature: Some(0.8),
            max_tokens: Some(4096),
            stream: Some(false),
        };

        let serialized = toml::to_string(&route).unwrap();
        let deserialized: ModelRouteConfig = toml::from_str(&serialized).unwrap();

        assert_eq!(route.name, deserialized.name);
        assert_eq!(route.provider, deserialized.provider);
        assert_eq!(route.model, deserialized.model);
        assert_eq!(route.api_key, deserialized.api_key);
        assert_eq!(route.temperature, deserialized.temperature);
        assert_eq!(route.max_tokens, deserialized.max_tokens);
        assert_eq!(route.stream, deserialized.stream);
    }

    #[test]
    fn test_settings_with_model_routes() {
        let toml_str = r#"
            active_route = "fast"

            [[model_routes]]
            name = "fast"
            provider = "openrouter"
            model = "gpt-4o-mini"

            [[model_routes]]
            name = "smart"
            provider = "openrouter"
            model = "claude-3-opus"
        "#;

        let settings: Settings = toml::from_str(toml_str).unwrap();

        assert_eq!(settings.model_routes.len(), 2);
        assert_eq!(settings.model_routes[0].name, "fast");
        assert_eq!(settings.model_routes[0].provider, "openrouter");
        assert_eq!(settings.model_routes[0].model, "gpt-4o-mini");
        assert_eq!(settings.active_route, "fast");

        let active = settings.get_active_route().unwrap();
        assert_eq!(active.name, "fast");
        assert_eq!(active.model, "gpt-4o-mini");
    }

    #[test]
    fn test_get_active_route_not_found() {
        let mut settings = Settings::default();
        settings.active_route = "nonexistent".to_string();
        let result = settings.get_active_route();
        assert!(result.is_err());
    }

    #[test]
    fn test_get_active_route_no_active_route_set() {
        let mut settings = Settings::default();
        settings.model_routes = vec![ModelRouteConfig {
            name: "test".to_string(),
            provider: "openrouter".to_string(),
            model: "gpt-4o".to_string(),
            api_key: String::new(),
            temperature: None,
            max_tokens: None,
            stream: None,
        }];
        settings.active_route = String::new();
        let result = settings.get_active_route();
        assert!(result.is_err());
    }

    #[test]
    fn test_legacy_settings_migration() {
        let legacy_toml = r#"
            [openrouter]
            api_key = "legacy-key"
            default_model = "gpt-4"
        "#;

        let legacy: LegacySettings = toml::from_str(legacy_toml).unwrap();
        let settings: Settings = legacy.into();

        assert_eq!(settings.model_routes.len(), 1);
        assert_eq!(settings.model_routes[0].name, "default");
        assert_eq!(settings.model_routes[0].provider, "openrouter");
        assert_eq!(settings.model_routes[0].model, "gpt-4");
        assert_eq!(settings.model_routes[0].api_key, "legacy-key");
        assert_eq!(settings.active_route, "default");
    }

    #[test]
    fn test_legacy_settings_migration_with_existing_routes() {
        // Note: In TOML, keys under [table] belong to that table until the next [table]
        // So active_route must come BEFORE [openrouter] or after [[model_routes]] with explicit table
        let legacy_toml = r#"
            active_route = "custom"

            [openrouter]
            api_key = "legacy-key"

            [[model_routes]]
            name = "custom"
            provider = "deepseek"
            model = "deepseek-chat"
        "#;

        let legacy: LegacySettings = toml::from_str(legacy_toml).unwrap();
        let settings: Settings = legacy.into();

        // Should not migrate if model_routes already exists
        assert_eq!(settings.model_routes.len(), 1);
        assert_eq!(settings.model_routes[0].name, "custom");
        assert_eq!(settings.active_route, "custom");
    }
}
