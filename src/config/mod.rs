//! Configuration module
//!
//! Manages application configuration, including:
//! - Command-line argument parsing (using clap)
//! - TOML configuration loading
//! - Configuration struct definitions

pub mod cli;
pub mod settings;

use anyhow::Result;

/// Load configuration
///
/// Load from command line arguments first, then from configuration file
pub fn load_config() -> Result<settings::Settings> {
    let cli_args = cli::parse_args();
    let config_path = cli_args.config_path.as_deref();

    settings::Settings::load(config_path)
}
