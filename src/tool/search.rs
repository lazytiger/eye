//! Search tool for retrieving real-time information from the web
//!
//! This tool enables the LLM to search the web for current events, specific facts,
//! or topics not covered in the training data.

use async_trait::async_trait;
use serde_json::{json, Value};
use anyhow::Result;
use regex::Regex;
use rand::Rng;

use crate::provider::MessageContent;
use crate::tool::{ExecuteResult, Tool};
use crate::utils;

/// Search providers
#[derive(Clone, PartialEq)]
enum SearchProvider {
    DuckDuckGo,
    Bing,
}

impl SearchProvider {
    /// Get all providers for retry fallback
    fn all() -> Vec<SearchProvider> {
        vec![SearchProvider::DuckDuckGo, SearchProvider::Bing]
    }
}

/// Search tool that retrieves real-time information from the web
pub struct SearchTool;

impl SearchTool {
    /// Creates a new instance of SearchTool
    pub fn new() -> Self {
        Self
    }

    /// Randomly select a search provider
    fn select_provider() -> SearchProvider {
        let mut rng = rand::thread_rng();
        if rng.gen_bool(0.5) {
            SearchProvider::DuckDuckGo
        } else {
            SearchProvider::Bing
        }
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

        // Randomly select initial search provider
        let initial_provider = SearchTool::select_provider();
        tracing::debug!("Starting with {} search provider",
            match initial_provider {
                SearchProvider::DuckDuckGo => "DuckDuckGo",
                SearchProvider::Bing => "Bing",
            }
        );

        // Try providers with fallback - start with random provider, then try others
        let mut providers_tried = Vec::new();
        let providers = SearchProvider::all();

        // Build provider order: start with random, then try others
        let mut provider_order = Vec::new();
        provider_order.push(initial_provider.clone());
        for p in providers {
            if p != initial_provider {
                provider_order.push(p);
            }
        }

        for provider in provider_order {
            let provider_name = match &provider {
                SearchProvider::DuckDuckGo => "DuckDuckGo",
                SearchProvider::Bing => "Bing",
            };

            // Skip if already tried
            if providers_tried.contains(&provider) {
                continue;
            }
            providers_tried.push(provider.clone());

            tracing::debug!("Trying {} search provider", provider_name);

            let results = match provider {
                SearchProvider::DuckDuckGo => search_duckduckgo(query, num_results).await,
                SearchProvider::Bing => search_bing(query, num_results).await,
            };

            match results {
                Ok(results) if !results.is_empty() => {
                    tracing::debug!("{} search succeeded with {} results", provider_name, results.len());
                    // Convert results to JSON string
                    let results_json = serde_json::to_string_pretty(&results)
                        .unwrap_or_else(|_| "[]".to_string());
                    return Ok(ExecuteResult::Success(MessageContent::Text(results_json)));
                }
                Ok(_) => {
                    tracing::debug!("{} search returned no results, trying fallback", provider_name);
                }
                Err(e) => {
                    tracing::debug!("{} search failed: {}, trying fallback", provider_name, e);
                }
            }
        }

        // All providers failed or returned no results
        Ok(ExecuteResult::Success(MessageContent::Text(
            "No search results found.".to_string()
        )))
    }
}

/// Search using DuckDuckGo HTML search
async fn search_duckduckgo(query: &str, num_results: usize) -> Result<Vec<Value>> {
    let client = utils::reqwest_client();
    let encoded_query = urlencoding::encode(query);
    let search_url = format!("https://html.duckduckgo.com/html/?q={}", encoded_query);

    let response = client
        .get(&search_url)
        .send()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to perform search: {}", e))?;

    if !response.status().is_success() {
        return Ok(Vec::new());
    }

    let html_content = response.text().await?;
    Ok(parse_duckduckgo_results(&html_content, num_results))
}

/// Search using Bing HTML search
async fn search_bing(query: &str, num_results: usize) -> Result<Vec<Value>> {
    let client = utils::reqwest_client();
    let encoded_query = urlencoding::encode(query);
    let search_url = format!("https://www.bing.com/search?q={}", encoded_query);

    let response = client
        .get(&search_url)
        .header("User-Agent", utils::user_agent())
        .send()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to perform search: {}", e))?;

    if !response.status().is_success() {
        return Ok(Vec::new());
    }

    let html_content = response.text().await?;
    Ok(parse_bing_results(&html_content, num_results))
}

/// Parse Bing HTML search results using regex
fn parse_bing_results(html: &str, max_results: usize) -> Vec<Value> {
    let mut results = Vec::new();

    // Bing HTML result structure:
    // <li class="b_algo">
    //   <h2><a href="...">Title</a></h2>
    //   <div class="b_caption"><p>Snippet</p></div>
    // </li>

    // Match b_algo list items - find starting positions
    let algo_regex = Regex::new(r#"<li[^>]*class="b_algo""#).unwrap();
    let link_regex = Regex::new(r#"<h2[^>]*><a href="([^"]+)">([^<]+)</a></h2>"#).unwrap();
    let snippet_regex = Regex::new(r#"<div class="b_caption"><p>([^<]*)</p></div>"#).unwrap();

    for algo_match in algo_regex.captures_iter(html) {
        if results.len() >= max_results {
            break;
        }

        let start = algo_match.get(0).unwrap().start();

        // Find the next <li tag to get the boundary
        let remaining = &html[start..];
        let next_li = remaining[1..].find("<li").map(|i| start + 1 + i).unwrap_or(html.len());
        let result_html = &html[start..next_li.min(start + 2000)];

        // Extract title and link
        if let Some(link_cap) = link_regex.captures(result_html) {
            let url = link_cap.get(1).map_or("", |m| m.as_str()).to_string();
            let title = link_cap.get(2).map_or("", |m| m.as_str())
                .replace("&amp;", "&")
                .replace("&lt;", "<")
                .replace("&gt;", ">")
                .replace("&quot;", "\"")
                .replace("&#39;", "'")
                .trim()
                .to_string();

            // Extract snippet
            let snippet = snippet_regex.captures(result_html)
                .and_then(|cap| cap.get(1))
                .map_or(String::new(), |m| {
                    m.as_str()
                        .replace("&amp;", "&")
                        .replace("&lt;", "<")
                        .replace("&gt;", ">")
                        .replace("&quot;", "\"")
                        .replace("&#39;", "'")
                });

            results.push(json!({
                "title": title,
                "link": url,
                "snippet": snippet.trim().to_string()
            }));
        }
    }

    results
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

        let num_results = &params["properties"]["num_results"];
        assert_eq!(num_results["type"], "integer");
        assert_eq!(num_results["minimum"], 1);
        assert_eq!(num_results["maximum"], 10);
    }

    #[tokio::test]
    async fn test_search_tool_execute_without_query() {
        let tool = SearchTool::new();
        let result = tool.execute(json!({})).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_search_tool_execute_with_empty_query() {
        let tool = SearchTool::new();
        let result = tool.execute(json!({ "query": "" })).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_search_tool_execute_with_num_results() {
        let tool = SearchTool::new();
        let result = tool.execute(json!({
            "query": "Rust programming",
            "num_results": 3
        })).await;

        // The test may fail if DuckDuckGo is unavailable, so we check the result format
        match result {
            Ok(execute_result) => {
                match execute_result {
                    ExecuteResult::Success(content) => {
                        match content {
                            MessageContent::Text(results_json) => {
                                // Should return valid JSON
                                let results: Value = serde_json::from_str(&results_json).unwrap_or(Value::Array(vec![]));
                                // Results should be an array
                                assert!(results.is_array());
                            }
                            MessageContent::Parts(_) => {
                                panic!("Expected text content, got parts");
                            }
                        }
                    }
                    ExecuteResult::Failure(_) => {
                        // DuckDuckGo might be unavailable, which is acceptable for this test
                        println!("Search returned failure (network issue - acceptable)");
                    }
                }
            }
            Err(e) => {
                // Network errors are acceptable
                println!("Search error (network issue - acceptable): {}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_search_tool_execute_with_invalid_num_results() {
        let tool = SearchTool::new();

        // Test with num_results > 10 (should be clamped to 10)
        let result = tool.execute(json!({
            "query": "test",
            "num_results": 100
        })).await;

        // Just verify it doesn't crash - network issues are acceptable
        match result {
            Ok(_) => {},
            Err(_) => println!("Network error acceptable"),
        }

        // Test with num_results = 0 (should be clamped to 1)
        let result = tool.execute(json!({
            "query": "test",
            "num_results": 0
        })).await;

        // Just verify it doesn't crash - network issues are acceptable
        match result {
            Ok(_) => {},
            Err(_) => println!("Network error acceptable"),
        }
    }

    #[tokio::test]
    async fn test_parse_duckduckgo_results() {
        // Test with sample DuckDuckGo HTML
        let sample_html = r#"
            <div class="results">
                <a class="result__a" href="https://example.com/page1">First Result Title</a>
                <a class="result__snippet">This is the first snippet.</a>

                <a class="result__a" href="https://example.com/page2">Second Result Title</a>
                <a class="result__snippet">This is the second snippet.</a>

                <a class="result__a" href="https://example.com/page3">Third Result Title</a>
                <a class="result__snippet">This is the third snippet.</a>
            </div>
        "#;

        let results = super::parse_duckduckgo_results(sample_html, 5);

        assert_eq!(results.len(), 3, "Should parse all 3 results");

        // Check first result
        assert_eq!(results[0]["title"], "First Result Title");
        assert_eq!(results[0]["link"], "https://example.com/page1");
        assert_eq!(results[0]["snippet"], "This is the first snippet.");

        // Check second result
        assert_eq!(results[1]["title"], "Second Result Title");
        assert_eq!(results[1]["link"], "https://example.com/page2");
        assert_eq!(results[1]["snippet"], "This is the second snippet.");

        // Check third result
        assert_eq!(results[2]["title"], "Third Result Title");
        assert_eq!(results[2]["link"], "https://example.com/page3");
        assert_eq!(results[2]["snippet"], "This is the third snippet.");
    }

    #[tokio::test]
    async fn test_parse_duckduckgo_results_max_limit() {
        let sample_html = r#"
            <div class="results">
                <a class="result__a" href="https://example.com/page1">Result 1</a>
                <a class="result__a" href="https://example.com/page2">Result 2</a>
                <a class="result__a" href="https://example.com/page3">Result 3</a>
                <a class="result__a" href="https://example.com/page4">Result 4</a>
                <a class="result__a" href="https://example.com/page5">Result 5</a>
            </div>
        "#;

        // Test with max_results = 2
        let results = super::parse_duckduckgo_results(sample_html, 2);
        assert_eq!(results.len(), 2, "Should respect max_results limit");

        // Test with max_results = 10 (more than available)
        let results = super::parse_duckduckgo_results(sample_html, 10);
        assert_eq!(results.len(), 5, "Should return all available results");
    }

    #[tokio::test]
    async fn test_parse_duckduckgo_results_with_html_entities() {
        let sample_html = r#"
            <div class="results">
                <a class="result__a" href="https://example.com/page">Title &amp; Description</a>
                <a class="result__snippet">Snippet with &lt;tags&gt; and &quot;quotes&quot;</a>
            </div>
        "#;

        let results = super::parse_duckduckgo_results(sample_html, 1);

        assert_eq!(results.len(), 1);
        // HTML entities should be decoded
        assert_eq!(results[0]["title"], "Title & Description");
        assert_eq!(results[0]["snippet"], "Snippet with <tags> and \"quotes\"");
    }

    #[tokio::test]
    async fn test_extract_snippet_after() {
        let html = r#"
            <a class="result__a" href="https://example.com">Title</a>
            <a class="result__snippet">First snippet</a>
            <a class="result__snippet">Second snippet</a>
        "#;

        let pattern = Regex::new(r#"<a[^>]*class="result__snippet"[^>]*>([^<]+)</a>"#).unwrap();

        // Find the position after the first link
        let url_end = html.find("https://example.com").unwrap() + "https://example.com".len();

        let snippet = super::extract_snippet_after(html, &pattern, url_end);

        // Should extract the first snippet after the link
        assert!(snippet.contains("First snippet") || snippet.contains("Second snippet"));
    }

    #[tokio::test]
    async fn test_search_tool_definition() {
        let tool = SearchTool::new();
        let definition = tool.definition();

        assert_eq!(definition.name, "search_web");
        assert!(!definition.description.is_empty());
        assert_eq!(definition.parameters["type"], "object");
    }

    #[test]
    fn test_parse_bing_results() {
        // Test with sample Bing HTML
        let sample_html = r#"
            <ol id="b_results">
                <li class="b_algo">
                    <h2><a href="https://example.com/page1">First Result Title</a></h2>
                    <div class="b_caption"><p>This is the first snippet.</p></div>
                </li>
                <li class="b_algo">
                    <h2><a href="https://example.com/page2">Second Result Title</a></h2>
                    <div class="b_caption"><p>This is the second snippet.</p></div>
                </li>
                <li class="b_algo">
                    <h2><a href="https://example.com/page3">Third Result Title</a></h2>
                    <div class="b_caption"><p>This is the third snippet.</p></div>
                </li>
            </ol>
        "#;

        let results = super::parse_bing_results(sample_html, 5);

        assert_eq!(results.len(), 3, "Should parse all 3 results");

        // Check first result
        assert_eq!(results[0]["title"], "First Result Title");
        assert_eq!(results[0]["link"], "https://example.com/page1");
        assert_eq!(results[0]["snippet"], "This is the first snippet.");

        // Check second result
        assert_eq!(results[1]["title"], "Second Result Title");
        assert_eq!(results[1]["link"], "https://example.com/page2");
        assert_eq!(results[1]["snippet"], "This is the second snippet.");

        // Check third result
        assert_eq!(results[2]["title"], "Third Result Title");
        assert_eq!(results[2]["link"], "https://example.com/page3");
        assert_eq!(results[2]["snippet"], "This is the third snippet.");
    }

    #[test]
    fn test_parse_bing_results_max_limit() {
        let sample_html = r#"
            <ol id="b_results">
                <li class="b_algo"><h2><a href="https://example.com/1">Result 1</a></h2><div class="b_caption"><p>Snippet 1</p></div></li>
                <li class="b_algo"><h2><a href="https://example.com/2">Result 2</a></h2><div class="b_caption"><p>Snippet 2</p></div></li>
                <li class="b_algo"><h2><a href="https://example.com/3">Result 3</a></h2><div class="b_caption"><p>Snippet 3</p></div></li>
                <li class="b_algo"><h2><a href="https://example.com/4">Result 4</a></h2><div class="b_caption"><p>Snippet 4</p></div></li>
                <li class="b_algo"><h2><a href="https://example.com/5">Result 5</a></h2><div class="b_caption"><p>Snippet 5</p></div></li>
            </ol>
        "#;

        // Test with max_results = 2
        let results = super::parse_bing_results(sample_html, 2);
        assert_eq!(results.len(), 2, "Should respect max_results limit");

        // Test with max_results = 10 (more than available)
        let results = super::parse_bing_results(sample_html, 10);
        assert_eq!(results.len(), 5, "Should return all available results");
    }

    #[test]
    fn test_parse_bing_results_with_html_entities() {
        let sample_html = r#"
            <ol id="b_results">
                <li class="b_algo">
                    <h2><a href="https://example.com/page">Title &amp; Description</a></h2>
                    <div class="b_caption"><p>Snippet with &lt;tags&gt; and &quot;quotes&quot;</p></div>
                </li>
            </ol>
        "#;

        let results = super::parse_bing_results(sample_html, 1);

        assert_eq!(results.len(), 1);
        // HTML entities should be decoded
        assert_eq!(results[0]["title"], "Title & Description");
        assert_eq!(results[0]["snippet"], "Snippet with <tags> and \"quotes\"");
    }

    #[tokio::test]
    async fn test_search_bing_integration() {
        // Integration test with real Bing search
        let result = super::search_bing("Rust programming language", 3).await;

        // Test should not crash - network issues are acceptable
        match result {
            Ok(results) => {
                // If successful, should return a vector of results
                // Results may be empty if no matches found
                assert!(results.len() <= 3, "Should respect num_results limit");

                // If we got results, verify structure
                if !results.is_empty() {
                    for r in &results {
                        assert!(r.get("title").is_some(), "Result should have title");
                        assert!(r.get("link").is_some(), "Result should have link");
                        assert!(r.get("snippet").is_some(), "Result should have snippet");
                    }
                }
            }
            Err(e) => {
                // Network errors are acceptable for integration test
                println!("Bing search error (network issue - acceptable): {}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_search_duckduckgo_integration() {
        // Integration test with real DuckDuckGo search
        let result = super::search_duckduckgo("Rust programming language", 3).await;

        match result {
            Ok(results) => {
                assert!(results.len() <= 3, "Should respect num_results limit");

                if !results.is_empty() {
                    for r in &results {
                        assert!(r.get("title").is_some(), "Result should have title");
                        assert!(r.get("link").is_some(), "Result should have link");
                        assert!(r.get("snippet").is_some(), "Result should have snippet");
                    }
                }
            }
            Err(e) => {
                println!("DuckDuckGo search error (network issue - acceptable): {}", e);
            }
        }
    }
}
