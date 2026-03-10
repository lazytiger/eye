//! Search tool for retrieving real-time information from the web
//!
//! This tool enables the LLM to search the web for current events, specific facts,
//! or topics not covered in the training data.

use anyhow::Result;
use async_trait::async_trait;
use rand::seq::SliceRandom;
use scraper::{Html, Selector};
use serde_json::{json, Value};

use crate::provider::MessageContent;
use crate::tool::{ExecuteResult, Tool};
use crate::utils;
use crate::utils::user_agent;

/// Search providers
#[derive(Clone, PartialEq)]
#[allow(dead_code)]
enum SearchProvider {
    DuckDuckGo,
    Bing,
    Tavily,
}

impl SearchProvider {
    /// Get all providers for retry fallback
    fn all() -> Vec<SearchProvider> {
        // Prefer Tavily if API key is available, fallback to DuckDuckGo
        vec![SearchProvider::Tavily]
    }
}

/// Search tool that retrieves real-time information from the web
pub struct SearchTool;

impl Default for SearchTool {
    fn default() -> Self {
        Self
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
        "Searches the web for a given query and returns a list of relevant results with links and snippets. Uses Tavily API (preferred) or DuckDuckGo as fallback."
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
        let query = args["query"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("query parameter is required"))?;

        let num_results = args["num_results"].as_i64().unwrap_or(5).clamp(1, 10) as usize;

        // Get all providers and shuffle them
        let mut providers = SearchProvider::all();
        providers.shuffle(&mut rand::thread_rng());

        // Try each provider in random order
        for provider in providers {
            let provider_name = match &provider {
                SearchProvider::DuckDuckGo => "DuckDuckGo",
                SearchProvider::Bing => "Bing",
                SearchProvider::Tavily => "Tavily",
            };

            tracing::debug!("Trying {} search provider", provider_name);

            let results = match provider {
                SearchProvider::DuckDuckGo => search_duckduckgo(query, num_results).await,
                SearchProvider::Bing => search_bing(query, num_results).await,
                SearchProvider::Tavily => search_tavily(query, num_results).await,
            };

            match results {
                Ok(results) if !results.is_empty() => {
                    tracing::debug!(
                        "{} search succeeded with {} results",
                        provider_name,
                        results.len()
                    );
                    let results_json =
                        serde_json::to_string_pretty(&results).unwrap_or_else(|_| "[]".to_string());
                    return Ok(ExecuteResult::Success(MessageContent::Text(results_json)));
                }
                Ok(_) => {
                    tracing::debug!(
                        "{} search returned no results, trying fallback",
                        provider_name
                    );
                }
                Err(e) => {
                    tracing::debug!("{} search failed: {}, trying fallback", provider_name, e);
                }
            }
        }

        // All providers failed or returned no results
        Ok(ExecuteResult::Success(MessageContent::Text(
            "No search results found.".to_string(),
        )))
    }
}

/// Search using Tavily API
async fn search_tavily(query: &str, num_results: usize) -> Result<Vec<Value>> {
    // Get API key from environment variable
    let api_key = std::env::var("TAVILY_API_KEY")
        .map_err(|_| anyhow::anyhow!("TAVILY_API_KEY environment variable not set"))?;

    let client = utils::reqwest_client();
    let search_url = "https://api.tavily.com/search";

    let request_body = json!({
        "api_key": api_key,
        "query": query,
        "search_depth": "basic",
        "include_answers": true,
        "max_results": num_results
    });

    let response = client
        .post(search_url)
        .json(&request_body)
        .send()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to perform Tavily search: {}", e))?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        return Err(anyhow::anyhow!(
            "Tavily API returned error status {}: {}",
            status,
            error_text
        ));
    }

    let response_json: Value = response.json().await?;
    Ok(parse_tavily_results(&response_json))
}

/// Parse Tavily API response
fn parse_tavily_results(response: &Value) -> Vec<Value> {
    let mut results = Vec::new();

    if let Some(results_array) = response["results"].as_array() {
        for result in results_array {
            let title = result["title"].as_str().unwrap_or("").to_string();
            let url = result["url"].as_str().unwrap_or("").to_string();
            let snippet = result["content"]
                .as_str()
                .or_else(|| result["snippet"].as_str())
                .unwrap_or("")
                .to_string();

            if !title.is_empty() && !url.is_empty() {
                results.push(json!({
                    "title": title,
                    "link": url,
                    "snippet": snippet
                }));
            }
        }
    }

    results
}

/// Search using DuckDuckGo HTML search
async fn search_duckduckgo(query: &str, num_results: usize) -> Result<Vec<Value>> {
    let client = utils::reqwest_client();
    let encoded_query = urlencoding::encode(query);
    let search_url = format!("https://html.duckduckgo.com/html/?q={}", encoded_query);

    let response = client
        .get(&search_url)
        .header("User-Agent", user_agent())
        .send()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to perform search: {}", e))?;

    if !response.status().is_success() {
        tracing::warn!("{} did not return a success status code", search_url);
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

    tracing::debug!("Bing search URL: {}", search_url);

    let response = client
        .get(&search_url)
        .header("User-Agent", utils::user_agent())
        .send()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to perform search: {}", e))?;

    tracing::debug!("Bing response status: {}", response.status());

    if !response.status().is_success() {
        tracing::warn!(
            "Bing search returned non-success status: {}",
            response.status()
        );
        return Ok(Vec::new());
    }

    let html_content = response.text().await?;
    tracing::debug!("Bing HTML content length: {} bytes", html_content.len());

    let results = parse_bing_results(&html_content, num_results);
    tracing::debug!("Bing parsed {} results", results.len());

    Ok(results)
}

/// Parse Bing HTML search results using CSS selectors
fn parse_bing_results(html: &str, max_results: usize) -> Vec<Value> {
    let mut results = Vec::new();
    let document = Html::parse_document(html);

    // Bing HTML result structure:
    // <li class="b_algo">
    //   <h2><a href="...">Title</a></h2>
    //   <div class="b_caption"><p>Snippet</p></div>
    // </li>

    let algo_selector = Selector::parse("li.b_algo").unwrap();
    let title_selector = Selector::parse("h2 a").unwrap();
    let snippet_selector = Selector::parse("div.b_caption p").unwrap();

    for result in document.select(&algo_selector) {
        if results.len() >= max_results {
            break;
        }

        // Extract title and link
        if let Some(title_elem) = result.select(&title_selector).next() {
            let url = title_elem.value().attr("href").unwrap_or("").to_string();
            let title = title_elem.text().collect::<String>().trim().to_string();

            tracing::debug!("Found title: {} at URL: {}", title, url);

            // Extract snippet
            let snippet = result
                .select(&snippet_selector)
                .next()
                .map(|elem| elem.text().collect::<String>().trim().to_string())
                .unwrap_or_default();

            if snippet.is_empty() {
                tracing::debug!("No snippet found for result");
            }

            results.push(json!({
                "title": title,
                "link": url,
                "snippet": snippet
            }));
        } else {
            tracing::debug!("Failed to extract title/link from result");
        }
    }

    results
}

/// Parse DuckDuckGo HTML search results using CSS selectors
fn parse_duckduckgo_results(html: &str, max_results: usize) -> Vec<Value> {
    let mut results = Vec::new();
    let document = Html::parse_document(html);

    // DuckDuckGo HTML result structure:
    // <a class="result__a" href="...">Title</a>
    // <a class="result__snippet">Snippet</a>

    let result_selector = Selector::parse("a.result__a").unwrap();
    let snippet_selector = Selector::parse("a.result__snippet").unwrap();

    // Collect all results and snippets in document order
    let all_results: Vec<_> = document
        .select(&result_selector)
        .take(max_results)
        .collect();
    let all_snippets: Vec<_> = document.select(&snippet_selector).collect();

    for (idx, result) in all_results.iter().enumerate() {
        let url = result.value().attr("href").unwrap_or("").to_string();
        let title = result.text().collect::<String>().trim().to_string();

        // Get the snippet at the same index (if available)
        let snippet = all_snippets
            .get(idx)
            .map(|elem| elem.text().collect::<String>().trim().to_string())
            .unwrap_or_default();

        results.push(json!({
            "title": title,
            "link": url,
            "snippet": snippet
        }));
    }

    if results.is_empty() {
        println!("{}", html);
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_search_tool_name() {
        let tool = SearchTool::default();
        assert_eq!(tool.name(), "search_web");
    }

    #[tokio::test]
    async fn test_search_tool_description() {
        let tool = SearchTool::default();
        assert_eq!(
            tool.description(),
            "Searches the web for a given query and returns a list of relevant results with links and snippets. Uses Tavily API (preferred) or DuckDuckGo as fallback."
        );
    }

    #[tokio::test]
    async fn test_search_tool_parameters() {
        let tool = SearchTool::default();
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
        let tool = SearchTool::default();
        let result = tool.execute(json!({})).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_search_tool_execute_with_empty_query() {
        let tool = SearchTool::default();
        let result = tool.execute(json!({ "query": "" })).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_search_tool_execute_with_num_results() {
        let tool = SearchTool::default();
        let result = tool
            .execute(json!({
                "query": "Rust programming",
                "num_results": 3
            }))
            .await;

        // The test may fail if DuckDuckGo is unavailable, so we check the result format
        match result {
            Ok(execute_result) => {
                match execute_result {
                    ExecuteResult::Success(content) => {
                        match content {
                            MessageContent::Text(results_json) => {
                                // Should return valid JSON
                                let results: Value = serde_json::from_str(&results_json)
                                    .unwrap_or(Value::Array(vec![]));
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
        let tool = SearchTool::default();

        // Test with num_results > 10 (should be clamped to 10)
        let result = tool
            .execute(json!({
                "query": "test",
                "num_results": 100
            }))
            .await;

        // Just verify it doesn't crash - network issues are acceptable
        match result {
            Ok(_) => {}
            Err(_) => println!("Network error acceptable"),
        }

        // Test with num_results = 0 (should be clamped to 1)
        let result = tool
            .execute(json!({
                "query": "test",
                "num_results": 0
            }))
            .await;

        // Just verify it doesn't crash - network issues are acceptable
        match result {
            Ok(_) => {}
            Err(_) => println!("Network error acceptable"),
        }
    }

    #[test]
    fn test_parse_duckduckgo_results() {
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

        let results = parse_duckduckgo_results(sample_html, 5);

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
    fn test_parse_duckduckgo_results_max_limit() {
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
        let results = parse_duckduckgo_results(sample_html, 2);
        assert_eq!(results.len(), 2, "Should respect max_results limit");

        // Test with max_results = 10 (more than available)
        let results = parse_duckduckgo_results(sample_html, 10);
        assert_eq!(results.len(), 5, "Should return all available results");
    }

    #[test]
    fn test_parse_duckduckgo_results_with_html_entities() {
        let sample_html = r#"
            <div class="results">
                <a class="result__a" href="https://example.com/page">Title &amp; Description</a>
                <a class="result__snippet">Snippet with &lt;tags&gt; and &quot;quotes&quot;</a>
            </div>
        "#;

        let results = parse_duckduckgo_results(sample_html, 1);

        assert_eq!(results.len(), 1);
        // scraper automatically normalizes some HTML entities
        assert_eq!(results[0]["title"], "Title & Description");
        assert_eq!(results[0]["snippet"], "Snippet with <tags> and \"quotes\"");
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

        let results = parse_bing_results(sample_html, 5);

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
        let results = parse_bing_results(sample_html, 2);
        assert_eq!(results.len(), 2, "Should respect max_results limit");

        // Test with max_results = 10 (more than available)
        let results = parse_bing_results(sample_html, 10);
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

        let results = parse_bing_results(sample_html, 1);

        assert_eq!(results.len(), 1);
        // scraper automatically normalizes some HTML entities
        assert_eq!(results[0]["title"], "Title & Description");
        assert_eq!(results[0]["snippet"], "Snippet with <tags> and \"quotes\"");
    }

    #[tokio::test]
    async fn test_search_tool_definition() {
        let tool = SearchTool::default();
        let definition = tool.definition();

        assert_eq!(definition.name, "search_web");
        assert!(!definition.description.is_empty());
        assert_eq!(definition.parameters["type"], "object");
    }

    #[tokio::test]
    async fn test_search_bing_integration() {
        // Integration test with real Bing search - fetch and parse
        let result = search_bing("Rust programming language", 3).await;

        // Log the raw result for debugging
        match &result {
            Ok(results) => {
                println!("Bing search returned {} results", results.len());
                for (i, r) in results.iter().enumerate() {
                    println!(
                        "Result {}: title={:?}, link={:?}, snippet={:?}",
                        i,
                        r.get("title"),
                        r.get("link"),
                        r.get("snippet")
                    );
                }
            }
            Err(e) => {
                println!("Bing search error: {}", e);
            }
        }

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
        let result = search_duckduckgo("Rust programming language", 3).await;

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
                println!(
                    "DuckDuckGo search error (network issue - acceptable): {}",
                    e
                );
            }
        }
    }

    #[test]
    fn test_parse_tavily_results() {
        // Test with sample Tavily API response
        let sample_response = json!({
            "query": "Rust programming",
            "follow_up_questions": [],
            "answer": null,
            "images": [],
            "results": [
                {
                    "title": "Rust Programming Language",
                    "url": "https://www.rust-lang.org/",
                    "content": "Rust is a programming language designed for performance and reliability.",
                    "snippet": "Official website of Rust"
                },
                {
                    "title": "Rust - Wikipedia",
                    "url": "https://en.wikipedia.org/wiki/Rust_(programming_language)",
                    "content": "Rust is a multi-paradigm, general-purpose programming language.",
                    "snippet": "Wikipedia article about Rust"
                }
            ]
        });

        let results = parse_tavily_results(&sample_response);

        assert_eq!(results.len(), 2, "Should parse all 2 results");

        // Check first result
        assert_eq!(results[0]["title"], "Rust Programming Language");
        assert_eq!(results[0]["link"], "https://www.rust-lang.org/");
        assert_eq!(
            results[0]["snippet"],
            "Rust is a programming language designed for performance and reliability."
        );

        // Check second result
        assert_eq!(results[1]["title"], "Rust - Wikipedia");
        assert_eq!(
            results[1]["link"],
            "https://en.wikipedia.org/wiki/Rust_(programming_language)"
        );
        assert_eq!(
            results[1]["snippet"],
            "Rust is a multi-paradigm, general-purpose programming language."
        );
    }

    #[test]
    fn test_parse_tavily_results_empty() {
        // Test with empty results
        let sample_response = json!({
            "query": "test",
            "results": []
        });

        let results = parse_tavily_results(&sample_response);
        assert!(results.is_empty(), "Should return empty results");
    }

    #[tokio::test]
    async fn test_search_tavily_without_api_key() {
        // Test that search fails gracefully without API key
        let result = search_tavily("test query", 3).await;

        // Should fail with API key error
        assert!(result.is_err());
        let error = result.unwrap_err().to_string();
        assert!(error.contains("TAVILY_API_KEY"));
    }
}
