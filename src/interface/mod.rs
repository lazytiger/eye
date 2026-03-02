//! Interface module
//!
//! Manages user interfaces, including:
//! - Base interface trait definition
//! - Interface trait definition
//! - CLI interface implementation
//! - Interface manager

pub mod base;
pub mod cli;
pub mod r#trait;

pub use self::{base::*, cli::CliInterface, r#trait::*};

use crate::config::settings::InterfaceConfig;

/// Create interface
///
/// Create corresponding interface instance based on configuration
pub fn create_interface(config: &InterfaceConfig) -> Box<dyn Interface> {
    let interface = CliInterface::new(config.clone());
    Box::new(interface)
}
