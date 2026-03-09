//! File Read tool for reading file contents
//!
//! This tool enables reading the contents of a file from the local filesystem.

use async_trait::async_trait;
use serde_json::{json, Value};
use anyhow::Result;
use tokio::fs;

use crate::provider::MessageContent;
use crate::tool::{ExecuteResult, Tool};

/// File Read tool for reading file contents
pub struct FileReadTool;

impl FileReadTool {
    /// Creates a new instance of FileReadTool
    pub fn new() -> Self {
        Self
    }
}

impl Default for FileReadTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Tool for FileReadTool {
    /// Returns the unique name of the tool
    fn name(&self) -> &str {
        "read_file"
    }

    /// Returns a description of what the tool does
    fn description(&self) -> &str {
        "Reads the contents of a file from the local filesystem and returns it as text."
    }

    /// Returns the JSON Schema for the arguments the tool accepts
    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "The path to the file to read. Can be absolute or relative to the current working directory."
                },
                "max_size": {
                    "type": "integer",
                    "description": "Maximum file size in bytes to read. Defaults to 100KB (102400). Set to 0 for no limit.",
                    "default": 102400
                }
            },
            "required": ["path"]
        })
    }

    /// Executes the tool logic with the given arguments
    async fn execute(&self, args: Value) -> Result<ExecuteResult> {
        // Parse arguments
        let path = args["path"].as_str()
            .ok_or_else(|| anyhow::anyhow!("path parameter is required"))?;

        let max_size = args["max_size"].as_u64().unwrap_or(102400);

        // Validate path is not empty
        if path.is_empty() {
            return Ok(ExecuteResult::Failure(
                "File path cannot be empty".to_string()
            ));
        }

        // Check if file exists and get metadata
        let metadata = match fs::metadata(path).await {
            Ok(meta) => meta,
            Err(e) => {
                return Ok(ExecuteResult::Failure(
                    format!("Failed to access file: {}", e)
                ));
            }
        };

        // Check if it's actually a file
        if !metadata.is_file() {
            return Ok(ExecuteResult::Failure(
                format!("Path is not a file: {}", path)
            ));
        }

        // Check file size
        let file_size = metadata.len();
        if max_size > 0 && file_size > max_size {
            return Ok(ExecuteResult::Failure(
                format!(
                    "File size ({:.2} KB) exceeds maximum allowed size ({:.2} KB). Use max_size parameter to increase limit.",
                    file_size as f64 / 1024.0,
                    max_size as f64 / 1024.0
                )
            ));
        }

        // Read file contents
        let content = match fs::read_to_string(path).await {
            Ok(c) => c,
            Err(e) => {
                return Ok(ExecuteResult::Failure(
                    format!("Failed to read file: {}", e)
                ));
            }
        };

        Ok(ExecuteResult::Success(MessageContent::Text(content)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use tokio::fs::File;
    use tokio::io::AsyncWriteExt;

    #[tokio::test]
    async fn test_file_read_tool_name() {
        let tool = FileReadTool::new();
        assert_eq!(tool.name(), "read_file");
    }

    #[tokio::test]
    async fn test_file_read_tool_description() {
        let tool = FileReadTool::new();
        assert_eq!(
            tool.description(),
            "Reads the contents of a file from the local filesystem and returns it as text."
        );
    }

    #[tokio::test]
    async fn test_file_read_tool_parameters() {
        let tool = FileReadTool::new();
        let params = tool.parameters();

        assert_eq!(params["type"], "object");
        assert!(params["properties"].is_object());
        assert!(params["required"].is_array());

        let required = params["required"].as_array().unwrap();
        assert!(required.contains(&Value::String("path".to_string())));

        let path_prop = &params["properties"]["path"];
        assert_eq!(path_prop["type"], "string");

        let max_size_prop = &params["properties"]["max_size"];
        assert_eq!(max_size_prop["type"], "integer");
        assert_eq!(max_size_prop["default"], 102400);
    }

    #[tokio::test]
    async fn test_file_read_tool_execute_without_path() {
        let tool = FileReadTool::new();
        let result = tool.execute(json!({})).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_file_read_tool_execute_with_empty_path() {
        let tool = FileReadTool::new();
        let result = tool.execute(json!({ "path": "" })).await;

        assert!(result.is_ok());
        match result.unwrap() {
            ExecuteResult::Failure(msg) => {
                assert!(msg.contains("empty"));
            }
            _ => panic!("Expected failure for empty path"),
        }
    }

    #[tokio::test]
    async fn test_file_read_tool_execute_with_nonexistent_file() {
        let tool = FileReadTool::new();
        let result = tool.execute(json!({ "path": "/nonexistent/file.txt" })).await;

        assert!(result.is_ok());
        match result.unwrap() {
            ExecuteResult::Failure(msg) => {
                assert!(msg.contains("Failed to access"));
            }
            _ => panic!("Expected failure for nonexistent file"),
        }
    }

    #[tokio::test]
    async fn test_file_read_tool_execute_with_directory() {
        let tool = FileReadTool::new();
        // Use current directory as test
        let result = tool.execute(json!({ "path": "." })).await;

        assert!(result.is_ok());
        match result.unwrap() {
            ExecuteResult::Failure(msg) => {
                assert!(msg.contains("not a file"));
            }
            _ => panic!("Expected failure for directory"),
        }
    }

    #[tokio::test]
    async fn test_file_read_tool_execute_success() {
        let tool = FileReadTool::new();
        let test_file = "test_file_read_temp.txt";
        let test_content = "Hello, this is a test file content!";

        // Create test file
        let mut file = File::create(test_file).await.unwrap();
        file.write_all(test_content.as_bytes()).await.unwrap();
        file.flush().await.unwrap();
        drop(file);

        // Read the file
        let result = tool.execute(json!({ "path": test_file })).await;

        assert!(result.is_ok());
        match result.unwrap() {
            ExecuteResult::Success(MessageContent::Text(content)) => {
                assert_eq!(content, test_content);
            }
            _ => panic!("Expected success with text content"),
        }

        // Clean up
        fs::remove_file(test_file).await.unwrap();
    }

    #[tokio::test]
    async fn test_file_read_tool_execute_file_too_large() {
        let tool = FileReadTool::new();
        let test_file = "test_file_large_temp.txt";
        let test_content = "This is test content";

        // Create test file
        let mut file = File::create(test_file).await.unwrap();
        file.write_all(test_content.as_bytes()).await.unwrap();
        file.flush().await.unwrap();
        drop(file);

        // Try to read with very small max_size
        let result = tool.execute(json!({
            "path": test_file,
            "max_size": 5
        })).await;

        assert!(result.is_ok());
        match result.unwrap() {
            ExecuteResult::Failure(msg) => {
                assert!(msg.contains("exceeds maximum"));
            }
            _ => panic!("Expected failure for file too large"),
        }

        // Clean up
        fs::remove_file(test_file).await.unwrap();
    }

    #[tokio::test]
    async fn test_file_read_tool_definition() {
        let tool = FileReadTool::new();
        let definition = tool.definition();

        assert_eq!(definition.name, "read_file");
        assert!(!definition.description.is_empty());
        assert_eq!(definition.parameters["type"], "object");
    }
}
