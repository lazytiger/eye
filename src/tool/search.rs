//! Search tool for retrieving real-time information from the web
//!
//! This tool enables the LLM to search the web for current events, specific facts,
//! or topics not covered in the training data.

use async_trait::async_trait;
use serde_json::{json, Value};
use anyhow::Result;

use crate::provider::MessageContent;
use crate::tool::{ExecuteResult, Tool};

/// Search tool that retrieves real-time information from the web
pub struct SearchTool;

impl SearchTool {
    /// Creates a new instance of SearchTool
    pub fn new() -> Self {
        Self
    }
}

impl Default for SearchTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Tool for SearchTool {
    /// Returns the unique name of the tool
    fn name(&self) -> &str {
        "search_web"
    }

    /// Returns a description of what the tool does
    fn description(&self) -> &str {
        "Searches the web for a given query and returns a list of relevant results with links and snippets."
    }

    /// Returns the JSON Schema for the arguments the tool accepts
    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "query": {
                    "type": "string",
                    "description": "The search query to use for finding relevant information."
                },
                "num_results": {
                    "type": "integer",
                    "description": "The number of search results to return. Defaults to 5. Minimum 1, maximum 10.",
                    "minimum": 1,
                    "maximum": 10
                }
            },
            "required": ["query"]
        })
    }

    /// Executes the tool logic with the given arguments
    async fn execute(&self, args: Value) -> Result<ExecuteResult> {
        // Parse arguments
        let _query = args["query"].as_str()
            .ok_or_else(|| anyhow::anyhow!("query parameter is required"))?;
        
        let num_results = args["num_results"].as_i64().unwrap_or(5).clamp(1, 10) as usize;

        // For now, we'll use a mock implementation since real search requires API keys
        // In production, you would integrate with Google Custom Search, Bing Search, or similar APIs
        let mock_results = vec![
            serde_json::json!({
                "title": "Search Result 1",
                "link": "https://example.com/result1",
                "snippet": "This is a mock search result for testing purposes. It represents a relevant webpage about your query."
            }),
            serde_json::json!({
                "title": "Search Result 2",
                "link": "https://example.com/result2",
                "snippet": "Another mock result that provides additional information. This would typically come from a different website."
            }),
            serde_json::json!({
                "title": "Search Result 3",
                "link": "https://example.com/result3",
                "snippet": "A third mock result offering more context. In real usage, these would be actual search results from the web."
            })
        ];

        let results_to_return = &mock_results[..num_results.min(mock_results.len())];

        // Convert results to JSON string for MessageContent
        let results_json = serde_json::to_string(results_to_return)
            .unwrap_or_else(|_| "[]".to_string());

        Ok(ExecuteResult::Success(MessageContent::Text(results_json)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_search_tool_name() {
        let tool = SearchTool::new();
        assert_eq!(tool.name(), "search_web");
    }

    #[tokio::test]
    async fn test_search_tool_description() {
        let tool = SearchTool::new();
        assert_eq!(
            tool.description(),
            "Searches the web for a given query and returns a list of relevant results with links and snippets."
        );
    }

    #[tokio::test]
    async fn test_search_tool_parameters() {
        let tool = SearchTool::new();
        let params = tool.parameters();
        
        assert_eq!(params["type"], "object");
        assert!(params["properties"].is_object());
        assert!(params["required"].is_array());
        
        let required = params["required"].as_array().unwrap();
        assert!(required.contains(&Value::String("query".to_string())));
        
        let query_prop = &params["properties"]["query"];
        assert_eq!(query_prop["type"], "string");
        
        let num_results_prop = &params["properties"]["num_results"];
        assert_eq!(num_results_prop["type"], "integer");
        assert_eq!(num_results_prop["minimum"], 1);
        assert_eq!(num_results_prop["maximum"], 10);
    }

    #[tokio::test]
    async fn test_search_tool_execute_with_query() {
        let tool = SearchTool::new();
        
        let result = tool.execute(json!({
            "query": "Rust programming",
            "num_results": 3
        })).await;
        
        assert!(result.is_ok());
        
        let execute_result = result.unwrap();
        match execute_result {
            ExecuteResult::Success(value) => {
                assert!(value.is_array());
                let results = value.as_array().unwrap();
                assert_eq!(results.len(), 3);
                
                for result in results {
                    assert!(result["title"].is_string());
                    assert!(result["link"].is_string());
                    assert!(result["snippet"].is_string());
                }
            }
            ExecuteResult::Failure(msg) => {
                panic!("Search tool execution failed with: {}", msg);
            }
        }
    }

    #[tokio::test]
    async fn test_search_tool_execute_without_query() {
        let tool = SearchTool::new();
        
        let result = tool.execute(json!({
            "num_results": 3
        })).await;
        
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_search_tool_execute_with_default_results() {
        let tool = SearchTool::new();
        
        let result = tool.execute(json!({
            "query": "Rust programming"
        })).await;
        
        assert!(result.is_ok());
        
        let execute_result = result.unwrap();
        match execute_result {
            ExecuteResult::Success(value) => {
                assert!(value.is_array());
                let results = value.as_array().unwrap();
                assert!(results.len() > 0);
            }
            ExecuteResult::Failure(msg) => {
                panic!("Search tool execution failed with: {}", msg);
            }
        }
    }

    #[tokio::test]
    async fn test_search_tool_execute_with_min_max_results() {
        let tool = SearchTool::new();
        
        // Test minimum results
        let min_result = tool.execute(json!({
            "query": "Rust programming",
            "num_results": 1
        })).await;
        assert!(min_result.is_ok());
        if let ExecuteResult::Success(value) = min_result.unwrap() {
            assert_eq!(value.as_array().unwrap().len(), 1);
        }

        // Test maximum results (mock only has 3 results)
        let max_result = tool.execute(json!({
            "query": "Rust programming",
            "num_results": 10
        })).await;
        assert!(max_result.is_ok());
        if let ExecuteResult::Success(value) = max_result.unwrap() {
            assert_eq!(value.as_array().unwrap().len(), 3);
        }
    }

    #[tokio::test]
    async fn test_search_tool_definition() {
        let tool = SearchTool::new();
        let definition = tool.definition();
        
        assert_eq!(definition.name, "search_web");
        assert!(!definition.description.is_empty());
        assert_eq!(definition.parameters["type"], "object");
    }
}
