//! Web Fetch tool for retrieving content from URLs
//!
//! This tool fetches the content of web pages and converts them to readable markdown format.

use async_trait::async_trait;
use serde_json::{json, Value};
use anyhow::Result;
use html2text;

use crate::provider::MessageContent;
use crate::tool::{ExecuteResult, Tool};
use crate::utils;

/// Web Fetch tool that retrieves webpage content and converts to markdown
pub struct WebFetchTool;

impl WebFetchTool {
    /// Creates a new instance of WebFetchTool
    pub fn new() -> Self {
        Self
    }
}

impl Default for WebFetchTool {
    fn default() -> Self {
        Self::new()
    }
}

impl WebFetchTool {
    /// Converts HTML content to readable Markdown format
    fn html_to_markdown(html: &str) -> String {
        // html2text 0.16+ returns a Result
        html2text::from_read(html.as_bytes(), 80).unwrap_or_else(|_| html.to_string())
    }
}

#[async_trait]
impl Tool for WebFetchTool {
    /// Returns the unique name of the tool
    fn name(&self) -> &str {
        "fetch_webpage"
    }

    /// Returns a description of what the tool does
    fn description(&self) -> &str {
        "Fetches the content of a webpage from a given URL and converts it to readable markdown format for easier consumption."
    }

    /// Returns the JSON Schema for the arguments the tool accepts
    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "url": {
                    "type": "string",
                    "description": "The URL of the webpage to fetch content from. Must be a valid HTTP/HTTPS URL."
                }
            },
            "required": ["url"]
        })
    }

    /// Executes the tool logic with the given arguments
    async fn execute(&self, args: Value) -> Result<ExecuteResult> {
        // Parse arguments
        let url = args["url"].as_str()
            .ok_or_else(|| anyhow::anyhow!("url parameter is required"))?;
        
        // Validate URL format
        if !url.starts_with("http://") && !url.starts_with("https://") {
            return Ok(ExecuteResult::Failure(
                "URL must start with http:// or https://".to_string()
            ));
        }

        // For real URLs, we need to make an actual HTTP request and convert HTML to markdown
        let client = utils::reqwest_client();
        let response = client.get(url)
            .send()
            .await?;

        if !response.status().is_success() {
            return Ok(ExecuteResult::Failure(
                format!("Failed to fetch URL. Status code: {}", response.status())
            ));
        }

        let html_content = response.text().await?;

        // Convert HTML to readable text
        let text_content = WebFetchTool::html_to_markdown(&html_content);

        // Truncate if too long (max 10000 chars) - must respect Unicode char boundaries
        let truncated = if text_content.len() > 10000 {
            // Use char_indices to find the byte position at or before 10000 chars
            let end_pos = text_content
                .char_indices()
                .nth(10000)
                .map(|(i, _)| i)
                .unwrap_or(text_content.len());
            format!("{}...\n[Content truncated]", &text_content[..end_pos])
        } else {
            text_content
        };

        Ok(ExecuteResult::Success(MessageContent::Text(truncated)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_web_fetch_tool_name() {
        let tool = WebFetchTool::new();
        assert_eq!(tool.name(), "fetch_webpage");
    }

    #[tokio::test]
    async fn test_web_fetch_tool_description() {
        let tool = WebFetchTool::new();
        assert_eq!(
            tool.description(),
            "Fetches the content of a webpage from a given URL and converts it to readable markdown format for easier consumption."
        );
    }

    #[tokio::test]
    async fn test_web_fetch_tool_parameters() {
        let tool = WebFetchTool::new();
        let params = tool.parameters();
        
        assert_eq!(params["type"], "object");
        assert!(params["properties"].is_object());
        assert!(params["required"].is_array());
        
        let required = params["required"].as_array().unwrap();
        assert!(required.contains(&Value::String("url".to_string())));
        
        let url_prop = &params["properties"]["url"];
        assert_eq!(url_prop["type"], "string");
    }

    #[tokio::test]
    async fn test_web_fetch_tool_execute_with_valid_url() {
        let tool = WebFetchTool::new();

        let result = tool.execute(json!({
            "url": "https://example.com"
        })).await;

        assert!(result.is_ok());

        let execute_result = result.unwrap();
        match execute_result {
            ExecuteResult::Success(content) => {
                match content {
                    MessageContent::Text(content_str) => {
                        // Check that content contains expected elements
                        assert!(content_str.contains("Example"));
                    }
                    MessageContent::Parts(_) => {
                        panic!("Expected text content, got parts");
                    }
                }
            }
            ExecuteResult::Failure(msg) => {
                panic!("Web fetch tool execution failed with: {}", msg);
            }
        }
    }

    #[tokio::test]
    async fn test_web_fetch_tool_execute_with_invalid_url() {
        let tool = WebFetchTool::new();
        
        let result = tool.execute(json!({
            "url": "invalid-url"
        })).await;
        
        assert!(result.is_ok());
        
        let execute_result = result.unwrap();
        match execute_result {
            ExecuteResult::Failure(msg) => {
                assert!(msg.contains("must start with"));
            }
            ExecuteResult::Success(_) => {
                panic!("Web fetch tool should have failed with invalid URL");
            }
        }
    }

    #[tokio::test]
    async fn test_web_fetch_tool_execute_without_url() {
        let tool = WebFetchTool::new();
        
        let result = tool.execute(json!({})).await;
        
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_web_fetch_tool_execute_with_real_url() {
        let tool = WebFetchTool::new();
        
        let result = tool.execute(json!({
            "url": "https://example.com"
        })).await;
        
        assert!(result.is_ok());
        
        let execute_result = result.unwrap();
        match execute_result {
            ExecuteResult::Success(content) => {
                match content {
                    MessageContent::Text(content_str) => {
                        // Content should contain expected information for example.com
                        assert!(content_str.contains("Example Domain") || content_str.contains("example"));
                    }
                    MessageContent::Parts(_) => {
                        panic!("Expected text content, got parts");
                    }
                }
            }
            ExecuteResult::Failure(msg) => {
                panic!("Web fetch tool execution failed with: {}", msg);
            }
        }
    }

    #[tokio::test]
    async fn test_web_fetch_tool_definition() {
        let tool = WebFetchTool::new();
        let definition = tool.definition();
        
        assert_eq!(definition.name, "fetch_webpage");
        assert!(!definition.description.is_empty());
        assert_eq!(definition.parameters["type"], "object");
    }
}
