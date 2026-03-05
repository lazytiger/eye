//! Time tool for getting current local time

use async_trait::async_trait;
use chrono::Local;
use serde_json::{json, Value};
use anyhow::Result;

use crate::tool::{ExecuteResult, Tool};

/// Time tool that returns the current local time
pub struct TimeTool;

impl TimeTool {
    /// Creates a new instance of TimeTool
    pub fn new() -> Self {
        Self
    }
}

impl Default for TimeTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Tool for TimeTool {
    /// Returns the unique name of the tool
    fn name(&self) -> &str {
        "get_current_time"
    }

    /// Returns a description of what the tool does
    fn description(&self) -> &str {
        "Retrieves the current local date and time."
    }

    /// Returns the JSON Schema for the arguments the tool accepts
    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {},
            "required": []
        })
    }

    /// Executes the tool logic with the given arguments
    async fn execute(&self, _args: Value) -> Result<ExecuteResult> {
        // Get current local time
        let current_time = Local::now();
        
        // Convert to string using chrono's default format
        let time_str = current_time.to_string();
        
        Ok(ExecuteResult::Success(Value::String(time_str)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_time_tool_name() {
        let tool = TimeTool::new();
        assert_eq!(tool.name(), "get_current_time");
    }

    #[tokio::test]
    async fn test_time_tool_description() {
        let tool = TimeTool::new();
        assert_eq!(tool.description(), "Retrieves the current local date and time.");
    }

    #[tokio::test]
    async fn test_time_tool_parameters() {
        let tool = TimeTool::new();
        let params = tool.parameters();
        
        // Verify the JSON schema structure
        assert_eq!(params["type"], "object");
        assert!(params["properties"].is_object());
        assert!(params["required"].is_array());
        assert_eq!(params["required"].as_array().unwrap().len(), 0);
    }

    #[tokio::test]
    async fn test_time_tool_execute() {
        let tool = TimeTool::new();
        
        // Execute with empty arguments
        let result = tool.execute(json!({})).await;
        
        // Should succeed
        assert!(result.is_ok());
        
        let execute_result = result.unwrap();
        
        // Should be a Success variant
        match execute_result {
            ExecuteResult::Success(value) => {
                // Value should be a string
                assert!(value.is_string());
                
                // String should not be empty
                let time_str = value.as_str().unwrap();
                assert!(!time_str.is_empty());
                
                // Should contain date and time components
                // (basic validation that it looks like a timestamp)
                assert!(time_str.contains('-') || time_str.contains(':'));
            }
            ExecuteResult::Failure(_) => {
                panic!("Time tool execution should not fail");
            }
        }
    }

    #[tokio::test]
    async fn test_time_tool_definition() {
        let tool = TimeTool::new();
        let definition = tool.definition();
        
        assert_eq!(definition.name, "get_current_time");
        assert_eq!(definition.description, "Retrieves the current local date and time.");
        assert_eq!(definition.parameters["type"], "object");
    }
}
