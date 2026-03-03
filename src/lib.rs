//! Eye - Personal Intelligent Assistant
//!
//! A personal assistant that uses large language models with tool calling to interact with the real world.
//!
//! Key features:
//! - Supports models with tool calling via OpenRouter
//! - Command-line interface
//! - All components abstracted as Traits for easy extension
//! - Uses clap for command-line argument parsing

use crate::config::{cli, settings};
use anyhow::Context;
use derive_more::{Display, Error};
use tracing_appender::non_blocking::WorkerGuard;

#[derive(Error, Debug, Display)]
pub struct OptionIsNone;

pub trait OptionToResult<T> {
    fn to_ok(self) -> anyhow::Result<T>;
}

impl<T> OptionToResult<T> for Option<T> {
    fn to_ok(self) -> anyhow::Result<T> {
        self.ok_or_else(|| OptionIsNone.into())
    }
}

// Export primary modules
pub mod agent;
pub mod config;
pub mod interface;
pub mod provider;
pub mod skill;
pub mod tool;

pub mod memory;

#[cfg(any(target_os = "ios", target_os = "android"))]
const LOG_LEVEL: tracing::metadata::LevelFilter = tracing::metadata::LevelFilter::INFO;

#[cfg(target_os = "android")]
pub fn init_tracing(_: Option<std::path::PathBuf>) -> anyhow::Result<Option<WorkerGuard>> {
    fn tracing_level_filter(level: tracing::metadata::LevelFilter) -> tracing::log::LevelFilter {
        match level {
            tracing::metadata::LevelFilter::DEBUG => tracing::log::LevelFilter::Debug,
            tracing::metadata::LevelFilter::TRACE => tracing::log::LevelFilter::Trace,
            tracing::metadata::LevelFilter::INFO => tracing::log::LevelFilter::Info,
            tracing::metadata::LevelFilter::WARN => tracing::log::LevelFilter::Warn,
            tracing::metadata::LevelFilter::ERROR => tracing::log::LevelFilter::Error,
            tracing::metadata::LevelFilter::OFF => tracing::log::LevelFilter::Off,
        }
    }

    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(tracing_level_filter(LOG_LEVEL))
            .with_tag("eye"),
    );
    Ok(None)
}

#[cfg(not(target_os = "android"))]
pub fn init_tracing(log_path: Option<std::path::PathBuf>) -> anyhow::Result<Option<WorkerGuard>> {
    let (writer, guard) = if let Some(log_path) = log_path {
        let path = if log_path.is_dir() {
            log_path.as_path()
        } else {
            log_path.parent().to_ok()?
        };
        if !path.exists() {
            std::fs::create_dir_all(path)?;
        }
        tracing_appender::non_blocking(tracing_appender::rolling::daily(path, "eye.log"))
    } else {
        tracing_appender::non_blocking(std::io::stdout())
    };
    let builder = tracing_subscriber::fmt::Subscriber::builder();
    #[cfg(target_os = "ios")]
    let builder = builder.with_max_level(LOG_LEVEL);
    #[cfg(not(target_os = "ios"))]
    let builder = builder.with_env_filter(
        tracing_subscriber::EnvFilter::builder()
            .with_default_directive(tracing::level_filters::LevelFilter::INFO.into())
            .from_env_lossy(),
    );
    let subscriber = builder
        .with_ansi(false)
        .with_file(true)
        .with_line_number(true)
        .with_timer(tracing_subscriber::fmt::time::ChronoLocal::default())
        .with_writer(writer)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;
    Ok(Some(guard))
}

/// Run the Eye application
///
/// This is the main entry point for the application logic
pub async fn run() -> anyhow::Result<()> {
    // Initialize logging
    let _guard = init_tracing(None)?;
    tracing::info!("Starting Eye Personal Intelligent Assistant");

    // Parse command line arguments
    let cli_args = cli::parse_args();

    // Load configuration
    let config = settings::Settings::load(cli_args.config_path.as_deref())
        .context("Failed to load configuration")?;

    // Update API Key in configuration (if provided via command line)
    let mut config = config;
    if let Some(api_key) = cli_args.api_key {
        config.openrouter.api_key = api_key;
    }

    tracing::info!("Eye Personal Intelligent Assistant has exited");
    Ok(())
}
