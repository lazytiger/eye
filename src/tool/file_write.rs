//! File Write tool for writing content to files
//!
//! This tool enables writing text content to a file on the local filesystem.

use async_trait::async_trait;
use serde_json::{json, Value};
use anyhow::Result;
use tokio::fs;
use tokio::io::AsyncWriteExt;

use crate::provider::MessageContent;
use crate::tool::{ExecuteResult, Tool};

/// File Write tool for writing content to files
pub struct FileWriteTool;

impl FileWriteTool {
    /// Creates a new instance of FileWriteTool
    pub fn new() -> Self {
        Self
    }
}

impl Default for FileWriteTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Tool for FileWriteTool {
    /// Returns the unique name of the tool
    fn name(&self) -> &str {
        "write_file"
    }

    /// Returns a description of what the tool does
    fn description(&self) -> &str {
        "Writes text content to a file on the local filesystem. Creates the file if it doesn't exist, or overwrites if it does."
    }

    /// Returns the JSON Schema for the arguments the tool accepts
    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "The path to the file to write. Can be absolute or relative to the current working directory."
                },
                "content": {
                    "type": "string",
                    "description": "The text content to write to the file."
                },
                "append": {
                    "type": "boolean",
                    "description": "If true, append to the file instead of overwriting. Defaults to false.",
                    "default": false
                },
                "create_dirs": {
                    "type": "boolean",
                    "description": "If true, create parent directories if they don't exist. Defaults to false.",
                    "default": false
                }
            },
            "required": ["path", "content"]
        })
    }

    /// Executes the tool logic with the given arguments
    async fn execute(&self, args: Value) -> Result<ExecuteResult> {
        // Parse arguments
        let path = args["path"].as_str()
            .ok_or_else(|| anyhow::anyhow!("path parameter is required"))?;

        let content = args["content"].as_str()
            .ok_or_else(|| anyhow::anyhow!("content parameter is required"))?;

        let append = args["append"].as_bool().unwrap_or(false);
        let create_dirs = args["create_dirs"].as_bool().unwrap_or(false);

        // Validate path is not empty
        if path.is_empty() {
            return Ok(ExecuteResult::Failure(
                "File path cannot be empty".to_string()
            ));
        }

        // Create parent directories if requested
        if create_dirs {
            if let Some(parent) = std::path::Path::new(path).parent() {
                if !parent.as_os_str().is_empty() {
                    if let Err(e) = fs::create_dir_all(parent).await {
                        return Ok(ExecuteResult::Failure(
                            format!("Failed to create parent directories: {}", e)
                        ));
                    }
                }
            }
        }

        // Write to file
        let result = if append {
            // Append mode
            let mut file = match fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(path)
                .await
            {
                Ok(f) => f,
                Err(e) => {
                    return Ok(ExecuteResult::Failure(
                        format!("Failed to open file for appending: {}", e)
                    ));
                }
            };

            match file.write_all(content.as_bytes()).await {
                Ok(_) => Ok(()),
                Err(e) => Err(format!("Failed to write to file: {}", e)),
            }
        } else {
            // Overwrite mode
            match fs::write(path, content).await {
                Ok(_) => Ok(()),
                Err(e) => Err(format!("Failed to write to file: {}", e)),
            }
        };

        match result {
            Ok(_) => {
                let mode = if append { "appended to" } else { "written to" };
                Ok(ExecuteResult::Success(MessageContent::Text(
                    format!("Successfully {} file: {}", mode, path)
                )))
            }
            Err(msg) => Ok(ExecuteResult::Failure(msg)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use tokio::fs;

    #[tokio::test]
    async fn test_file_write_tool_name() {
        let tool = FileWriteTool::new();
        assert_eq!(tool.name(), "write_file");
    }

    #[tokio::test]
    async fn test_file_write_tool_description() {
        let tool = FileWriteTool::new();
        assert_eq!(
            tool.description(),
            "Writes text content to a file on the local filesystem. Creates the file if it doesn't exist, or overwrites if it does."
        );
    }

    #[tokio::test]
    async fn test_file_write_tool_parameters() {
        let tool = FileWriteTool::new();
        let params = tool.parameters();

        assert_eq!(params["type"], "object");
        assert!(params["properties"].is_object());
        assert!(params["required"].is_array());

        let required = params["required"].as_array().unwrap();
        assert!(required.contains(&Value::String("path".to_string())));
        assert!(required.contains(&Value::String("content".to_string())));

        let path_prop = &params["properties"]["path"];
        assert_eq!(path_prop["type"], "string");

        let content_prop = &params["properties"]["content"];
        assert_eq!(content_prop["type"], "string");

        let append_prop = &params["properties"]["append"];
        assert_eq!(append_prop["type"], "boolean");
        assert_eq!(append_prop["default"], false);

        let create_dirs_prop = &params["properties"]["create_dirs"];
        assert_eq!(create_dirs_prop["type"], "boolean");
        assert_eq!(create_dirs_prop["default"], false);
    }

    #[tokio::test]
    async fn test_file_write_tool_execute_without_path() {
        let tool = FileWriteTool::new();
        let result = tool.execute(json!({ "content": "test" })).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_file_write_tool_execute_without_content() {
        let tool = FileWriteTool::new();
        let result = tool.execute(json!({ "path": "test.txt" })).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_file_write_tool_execute_with_empty_path() {
        let tool = FileWriteTool::new();
        let result = tool.execute(json!({ "path": "", "content": "test" })).await;

        assert!(result.is_ok());
        match result.unwrap() {
            ExecuteResult::Failure(msg) => {
                assert!(msg.contains("empty"));
            }
            _ => panic!("Expected failure for empty path"),
        }
    }

    #[tokio::test]
    async fn test_file_write_tool_execute_success() {
        let tool = FileWriteTool::new();
        let test_file = "test_file_write_temp.txt";
        let test_content = "Hello, this is test content to write!";

        // Write to file
        let result = tool.execute(json!({
            "path": test_file,
            "content": test_content
        })).await;

        assert!(result.is_ok());
        match result.unwrap() {
            ExecuteResult::Success(MessageContent::Text(msg)) => {
                assert!(msg.contains("Successfully"));
                assert!(msg.contains(test_file));
            }
            _ => panic!("Expected success message"),
        }

        // Verify content
        let content = fs::read_to_string(test_file).await.unwrap();
        assert_eq!(content, test_content);

        // Clean up
        fs::remove_file(test_file).await.unwrap();
    }

    #[tokio::test]
    async fn test_file_write_tool_execute_append() {
        let tool = FileWriteTool::new();
        let test_file = "test_file_append_temp.txt";
        let content1 = "First line\n";
        let content2 = "Second line\n";

        // Write initial content
        fs::write(test_file, content1).await.unwrap();

        // Append more content
        let result = tool.execute(json!({
            "path": test_file,
            "content": content2,
            "append": true
        })).await;

        assert!(result.is_ok());

        // Verify content was appended
        let content = fs::read_to_string(test_file).await.unwrap();
        assert_eq!(content, format!("{}{}", content1, content2));

        // Clean up
        fs::remove_file(test_file).await.unwrap();
    }

    #[tokio::test]
    async fn test_file_write_tool_execute_overwrite() {
        let tool = FileWriteTool::new();
        let test_file = "test_file_overwrite_temp.txt";
        let content1 = "Original content";
        let content2 = "New content";

        // Write initial content
        fs::write(test_file, content1).await.unwrap();

        // Overwrite with new content
        let result = tool.execute(json!({
            "path": test_file,
            "content": content2,
            "append": false
        })).await;

        assert!(result.is_ok());

        // Verify content was overwritten
        let content = fs::read_to_string(test_file).await.unwrap();
        assert_eq!(content, content2);

        // Clean up
        fs::remove_file(test_file).await.unwrap();
    }

    #[tokio::test]
    async fn test_file_write_tool_execute_create_dirs() {
        let tool = FileWriteTool::new();
        let test_file = "temp_test_dir/subdir/test_file.txt";
        let test_content = "Content with created dirs";

        // Write with create_dirs = true
        let result = tool.execute(json!({
            "path": test_file,
            "content": test_content,
            "create_dirs": true
        })).await;

        assert!(result.is_ok());

        // Verify file was created
        let content = fs::read_to_string(test_file).await.unwrap();
        assert_eq!(content, test_content);

        // Clean up
        fs::remove_file(test_file).await.unwrap();
        fs::remove_dir_all("temp_test_dir").await.unwrap();
    }

    #[tokio::test]
    async fn test_file_write_tool_definition() {
        let tool = FileWriteTool::new();
        let definition = tool.definition();

        assert_eq!(definition.name, "write_file");
        assert!(!definition.description.is_empty());
        assert_eq!(definition.parameters["type"], "object");
    }
}
