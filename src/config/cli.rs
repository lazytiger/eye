//! Command-line argument definitions
//!
//! Define all CLI arguments using clap

use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// Eye - Personal AI Assistant
#[derive(Parser, Debug)]
#[command(name = "eye")]
#[command(version = "0.1.0")]
#[command(about = "Personal AI Assistant - interacts with the real world via tools", long_about = None)]
pub struct Cli {
    /// Configuration file path
    #[arg(short, long, value_name = "FILE")]
    pub config_path: Option<PathBuf>,

    /// OpenRouter API Key
    #[arg(short, long, env = "OPENROUTER_API_KEY")]
    pub api_key: Option<String>,

    /// Model name
    #[arg(short, long, default_value = "deepseek/deepseek-v3.2")]
    pub model: String,

    /// Interaction mode
    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// Subcommands
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Start an interactive session
    Chat {
        /// System prompt
        #[arg(short, long)]
        system_prompt: Option<String>,
    },
    /// Execute a single query
    Query {
        /// Query text
        query: String,
    },
    /// List available tools
    ListTools,
    /// List available skills
    ListSkills,
    /// List available model routes
    ListRoutes,
}

/// Parse command-line arguments
pub fn parse_args() -> Cli {
    Cli::parse()
}
