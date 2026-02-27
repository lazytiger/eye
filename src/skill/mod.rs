//! Skill module
//!
//! Manages the skill system, including:
//! - Skill trait definition
//! - Skill manager

pub mod r#trait;

pub use self::r#trait::*;

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
