//! Interface module
//!
//! Manages user interfaces, including:
//! - Interface trait definition
//! - Console interface implementation
//! - Interface manager

pub mod console;
pub mod r#trait;

pub use self::{console::ConsoleInterface, r#trait::*};

use crate::config::settings::InterfaceConfig;

/// Create interface
///
/// Create corresponding interface instance based on configuration
pub fn create_interface(config: &InterfaceConfig) -> Box<dyn Interface> {
    let interface = ConsoleInterface::new(config.clone());
    Box::new(interface)
}
