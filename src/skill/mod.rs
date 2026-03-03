//! Skill module
//!
//! Manages the skill system, including:
//! - Skill trait definition
//! - Skill manager
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

use std::{collections::HashMap, sync::Arc};

/// Skill manager
#[derive(Default)]
pub struct SkillManager {
    /// Skill registry
    skills: HashMap<String, Arc<dyn Skill>>,
}

impl SkillManager {
    /// Register a skill
    pub fn register_skill(&mut self, skill: Arc<dyn Skill>) {
        self.skills.insert(skill.name().to_string(), skill);
    }

    /// List skills
    pub fn list_skills(&self) -> Vec<String> {
        self.skills.keys().cloned().collect()
    }

    /// Check if a skill exists
    pub fn has_skill(&self, skill_name: &str) -> bool {
        self.skills.contains_key(skill_name)
    }

    /// Get a skill
    pub fn get_skill(&self, skill_name: &str) -> Option<Arc<dyn Skill>> {
        self.skills.get(skill_name).cloned()
    }
}
