//! Skill trait definition
//!
//! Defines the Skill trait to abstract different skill capabilities

use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;

/// Skill trait
#[async_trait]
pub trait Skill: Send + Sync {
    /// Get skill name
    fn name(&self) -> &str;

    /// Get skill description
    fn description(&self) -> &str;

    /// Get skill version
    fn version(&self) -> &str;

    /// Execute skill
    async fn execute(&self, input: &str, context: &Value) -> Result<Value>;

    /// Validate input
    fn validate_input(&self, input: &str) -> Result<()>;
}
