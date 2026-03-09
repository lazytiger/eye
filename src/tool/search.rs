//! Search tool for retrieving real-time information from the web
//!
//! This tool enables the LLM to search the web for current events, specific facts,
//! or topics not covered in the training data.

use async_trait::async_trait;
use serde_json::{json, Value};
use anyhow::Result;
use regex::Regex;

use crate::provider::MessageContent;
use crate::tool::{ExecuteResult, Tool};
use crate::utils;

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
        let query = args["query"].as_str()
            .ok_or_else(|| anyhow::anyhow!("query parameter is required"))?;

        let num_results = args["num_results"].as_i64().unwrap_or(5).clamp(1, 10) as usize;

        // Use DuckDuckGo HTML search (no API key required)
        let client = utils::reqwest_client();

        let encoded_query = urlencoding::encode(query);
        let search_url = format!("https://html.duckduckgo.com/html/?q={}", encoded_query);

        let response = client
            .get(&search_url)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to perform search: {}", e))?;

        if !response.status().is_success() {
            return Ok(ExecuteResult::Failure(
                format!("Search failed with status: {}", response.status())
            ));
        }

        let html_content = response.text().await?;

        // Parse HTML to extract results
        let results = parse_duckduckgo_results(&html_content, num_results);

        if results.is_empty() {
            return Ok(ExecuteResult::Success(MessageContent::Text(
                "No search results found.".to_string()
            )));
        }

        // Convert results to JSON string
        let results_json = serde_json::to_string_pretty(&results)
            .unwrap_or_else(|_| "[]".to_string());

        Ok(ExecuteResult::Success(MessageContent::Text(results_json)))
    }
}

/// Parse DuckDuckGo HTML search results using regex
fn parse_duckduckgo_results(html: &str, max_results: usize) -> Vec<Value> {
    let mut results = Vec::new();

    // DuckDuckGo HTML result structure:
    // <a class="result__a" href="...">Title</a>
    // <a class="result__snippet">Snippet</a>

    // Pattern to match result blocks
    let result_pattern = Regex::new(
        r#"<a[^>]*class="result__a"[^>]*href="([^"]+)"[^>]*>([^<]+)</a>"#
    ).unwrap();

    let snippet_pattern = Regex::new(
        r#"<a[^>]*class="result__snippet"[^>]*>([^<]+)</a>"#
    ).unwrap();

    for (idx, cap) in result_pattern.captures_iter(html).enumerate() {
        if idx >= max_results {
            break;
        }

        let url = cap.get(1).map_or("", |m| m.as_str()).to_string();
        let title = cap.get(2).map_or("", |m| m.as_str())
            .replace("&amp;", "&")
            .replace("&lt;", "<")
            .replace("&gt;", ">")
            .replace("&quot;", "\"")
            .replace("&#39;", "'");

        // Try to find a snippet after this result
        let snippet = extract_snippet_after(html, &snippet_pattern, cap.get(1).map_or(0, |m| m.end()));

        results.push(json!({
            "title": title.trim().to_string(),
            "link": url,
            "snippet": snippet.trim().to_string()
        }));
    }

    results
}

/// Extract snippet text from HTML after a given position
fn extract_snippet_after(html: &str, pattern: &Regex, start_pos: usize) -> String {
    if start_pos >= html.len() {
        return String::new();
    }

    let remaining = &html[start_pos..];

    // Find the first snippet in the remaining HTML
    if let Some(cap) = pattern.captures(remaining) {
        return cap.get(1).map_or("", |m| m.as_str())
            .replace("&amp;", "&")
            .replace("&lt;", "<")
            .replace("&gt;", ">")
            .replace("&quot;", "\"")
            .replace("&#39;", "'");
    }

    String::new()
}
