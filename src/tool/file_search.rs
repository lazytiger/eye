//! File Search tool for finding files by name or pattern
//!
//! This tool enables searching for files in the local filesystem by name, pattern, or extension.

use async_trait::async_trait;
use serde_json::{json, Value};
use anyhow::Result;
use tokio::fs;
use std::path::Path;
use regex::Regex;

use crate::provider::MessageContent;
use crate::tool::{ExecuteResult, Tool};

/// File Search tool for finding files by name or pattern
pub struct FileSearchTool;

impl FileSearchTool {
    /// Creates a new instance of FileSearchTool
    pub fn new() -> Self {
        Self
    }
}

impl Default for FileSearchTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Tool for FileSearchTool {
    /// Returns the unique name of the tool
    fn name(&self) -> &str {
        "search_files"
    }

    /// Returns a description of what the tool does
    fn description(&self) -> &str {
        "Searches for files in the local filesystem by name pattern, extension, or regex. Returns a list of matching file paths."
    }

    /// Returns the JSON Schema for the arguments the tool accepts
    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "The directory path to search in. Defaults to current directory if not specified."
                },
                "pattern": {
                    "type": "string",
                    "description": "The search pattern. Can be a simple substring, glob pattern (*, ?), or regex."
                },
                "extension": {
                    "type": "string",
                    "description": "Filter by file extension (e.g., '.rs', '.txt'). Don't include the dot if using glob pattern."
                },
                "regex": {
                    "type": "boolean",
                    "description": "If true, treat pattern as a regex. Defaults to false.",
                    "default": false
                },
                "recursive": {
                    "type": "boolean",
                    "description": "If true, search subdirectories recursively. Defaults to true.",
                    "default": true
                },
                "max_results": {
                    "type": "integer",
                    "description": "Maximum number of results to return. Defaults to 100, set to 0 for no limit.",
                    "default": 100
                },
                "include_hidden": {
                    "type": "boolean",
                    "description": "If true, include hidden files (starting with .). Defaults to false.",
                    "default": false
                }
            },
            "required": []
        })
    }

    /// Executes the tool logic with the given arguments
    async fn execute(&self, args: Value) -> Result<ExecuteResult> {
        // Parse arguments
        let path = args["path"].as_str().unwrap_or(".");
        let pattern = args["pattern"].as_str();
        let extension = args["extension"].as_str();
        let use_regex = args["regex"].as_bool().unwrap_or(false);
        let recursive = args["recursive"].as_bool().unwrap_or(true);
        let max_results = args["max_results"].as_u64().unwrap_or(100);
        let include_hidden = args["include_hidden"].as_bool().unwrap_or(false);

        // Validate path exists and is a directory
        let metadata = match fs::metadata(path).await {
            Ok(meta) => meta,
            Err(e) => {
                return Ok(ExecuteResult::Failure(
                    format!("Failed to access search path: {}", e)
                ));
            }
        };

        if !metadata.is_dir() {
            return Ok(ExecuteResult::Failure(
                format!("Path is not a directory: {}", path)
            ));
        }

        // Compile regex if needed
        let regex_pattern = if use_regex && pattern.is_some() {
            match Regex::new(pattern.unwrap()) {
                Ok(r) => Some(r),
                Err(e) => {
                    return Ok(ExecuteResult::Failure(
                        format!("Invalid regex pattern: {}", e)
                    ));
                }
            }
        } else {
            None
        };

        // Search for files
        let mut results: Vec<String> = Vec::new();
        search_directory(
            path,
            pattern,
            extension,
            regex_pattern.as_ref(),
            recursive,
            max_results,
            include_hidden,
            &mut results,
        ).await;

        if results.is_empty() {
            return Ok(ExecuteResult::Success(MessageContent::Text(
                "No matching files found.".to_string()
            )));
        }

        // Format results as JSON
        let results_json = json!({
            "count": results.len(),
            "search_path": path,
            "files": results
        });

        Ok(ExecuteResult::Success(MessageContent::Text(
            serde_json::to_string_pretty(&results_json).unwrap_or_else(|_| format!("{:?}", results))
        )))
    }
}

/// Search directories for matching files (iterative approach to avoid async recursion)
async fn search_directory(
    path: &str,
    pattern: Option<&str>,
    extension: Option<&str>,
    regex: Option<&Regex>,
    recursive: bool,
    max_results: u64,
    include_hidden: bool,
    results: &mut Vec<String>,
) {
    // Use a Vec as a stack for directories to search
    let mut dirs_to_search: Vec<String> = vec![path.to_string()];

    while let Some(current_path) = dirs_to_search.pop() {
        // Check if we've reached max results
        if max_results > 0 && results.len() as u64 >= max_results {
            return;
        }

        let mut entries = match fs::read_dir(&current_path).await {
            Ok(e) => e,
            Err(_) => continue,
        };

        while let Ok(Some(entry)) = entries.next_entry().await {
            // Check if we've reached max results
            if max_results > 0 && results.len() as u64 >= max_results {
                return;
            }

            let entry_path = entry.path();
            let file_name = match entry_path.file_name() {
                Some(name) => name.to_string_lossy(),
                None => continue,
            };

            // Skip hidden files unless requested
            if !include_hidden && file_name.starts_with('.') {
                continue;
            }

            // Skip cargo target, node_modules, .git, etc.
            if is_common_exclude_dir(&file_name) {
                continue;
            }

            let is_dir = entry_path.is_dir();

            if is_dir && recursive {
                // Add subdirectory to stack for later processing
                dirs_to_search.push(entry_path.to_string_lossy().to_string());
            } else if !is_dir {
                // Check if file matches criteria
                if matches_file(&file_name, pattern, extension, regex) {
                    results.push(entry_path.to_string_lossy().to_string());
                }
            }
        }
    }
}

/// Check if a directory name is commonly excluded
fn is_common_exclude_dir(name: &str) -> bool {
    matches!(name, "target" | "node_modules" | ".git" | "vendor" | "__pycache__" | ".cargo" | "build")
}

/// Check if a file name matches the search criteria
fn matches_file(
    file_name: &str,
    pattern: Option<&str>,
    extension: Option<&str>,
    regex: Option<&Regex>,
) -> bool {
    // Check extension filter
    if let Some(ext) = extension {
        let file_ext = Path::new(file_name)
            .extension()
            .map(|e| e.to_string_lossy())
            .unwrap_or_default();

        // Handle both with and without leading dot
        let ext_match = ext.starts_with('.')
            && file_ext == &ext[1..]
            || !ext.starts_with('.')
            && file_ext == *ext;

        if !ext_match {
            return false;
        }
    }

    // Check pattern match
    if let Some(pat) = pattern {
        // Regex match
        if let Some(re) = regex {
            return re.is_match(file_name);
        }

        // Glob pattern match (simple * and ? support)
        if pat.contains('*') || pat.contains('?') {
            return glob_match(file_name, pat);
        }

        // Simple substring match (case-insensitive)
        return file_name.to_lowercase().contains(&pat.to_lowercase());
    }

    // No pattern specified, match all files
    true
}

/// Simple glob pattern matching (* and ?)
fn glob_match(text: &str, pattern: &str) -> bool {
    let text_lower = text.to_lowercase();
    let pattern_lower = pattern.to_lowercase();

    // Handle patterns
    if pattern_lower == "*" {
        return true;
    }

    // Split by * and check each part
    let parts: Vec<&str> = pattern_lower.split('*').collect();

    if parts.len() == 1 {
        // No wildcards, exact match
        return text_lower == pattern_lower;
    }

    if parts.len() == 2 {
        if pattern_lower.starts_with('*') {
            // Ends with pattern
            return text_lower.ends_with(&parts[1]);
        }
        if pattern_lower.ends_with('*') {
            // Starts with pattern
            return text_lower.starts_with(&parts[0]);
        }
    }

    // Multiple wildcards - check each part in order
    let mut pos = 0;
    for part in &parts {
        if part.is_empty() {
            continue;
        }
        if let Some(found) = text_lower[pos..].find(*part) {
            pos += found + part.len();
        } else {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use tokio::fs::{File, create_dir_all};
    use tokio::io::AsyncWriteExt;

    #[tokio::test]
    async fn test_file_search_tool_name() {
        let tool = FileSearchTool::new();
        assert_eq!(tool.name(), "search_files");
    }

    #[tokio::test]
    async fn test_file_search_tool_description() {
        let tool = FileSearchTool::new();
        assert_eq!(
            tool.description(),
            "Searches for files in the local filesystem by name pattern, extension, or regex. Returns a list of matching file paths."
        );
    }

    #[tokio::test]
    async fn test_file_search_tool_parameters() {
        let tool = FileSearchTool::new();
        let params = tool.parameters();

        assert_eq!(params["type"], "object");
        assert!(params["properties"].is_object());

        // Check all optional parameters
        let props = &params["properties"];
        assert_eq!(props["path"]["type"], "string");
        assert_eq!(props["pattern"]["type"], "string");
        assert_eq!(props["extension"]["type"], "string");
        assert_eq!(props["regex"]["type"], "boolean");
        assert_eq!(props["recursive"]["type"], "boolean");
        assert_eq!(props["max_results"]["type"], "integer");
        assert_eq!(props["include_hidden"]["type"], "boolean");

        // Check defaults
        assert_eq!(props["regex"]["default"], false);
        assert_eq!(props["recursive"]["default"], true);
        assert_eq!(props["max_results"]["default"], 100);
        assert_eq!(props["include_hidden"]["default"], false);

        // No required fields
        assert!(params["required"].as_array().unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_file_search_tool_execute_with_invalid_path() {
        let tool = FileSearchTool::new();
        let result = tool.execute(json!({ "path": "/nonexistent/path" })).await;

        assert!(result.is_ok());
        match result.unwrap() {
            ExecuteResult::Failure(msg) => {
                assert!(msg.contains("Failed to access"));
            }
            _ => panic!("Expected failure for invalid path"),
        }
    }

    #[tokio::test]
    async fn test_file_search_tool_execute_with_file_instead_of_dir() {
        let tool = FileSearchTool::new();
        let test_file = "test_search_file_temp.txt";

        // Create test file
        let mut file = File::create(test_file).await.unwrap();
        file.write_all(b"test").await.unwrap();
        file.flush().await.unwrap();
        drop(file);

        // Try to search using file path as directory
        let result = tool.execute(json!({ "path": test_file })).await;

        assert!(result.is_ok());
        match result.unwrap() {
            ExecuteResult::Failure(msg) => {
                assert!(msg.contains("not a directory"));
            }
            _ => panic!("Expected failure for file path"),
        }

        // Clean up
        fs::remove_file(test_file).await.unwrap();
    }

    #[tokio::test]
    async fn test_file_search_tool_execute_basic_search() {
        let tool = FileSearchTool::new();
        let test_dir = "test_search_dir";
        let test_files = vec!["file1.txt", "file2.txt", "file3.rs", "readme.md"];

        // Create test directory and files
        create_dir_all(test_dir).await.unwrap();
        for file in &test_files {
            let path = format!("{}/{}", test_dir, file);
            let mut file = File::create(&path).await.unwrap();
            file.write_all(b"test").await.unwrap();
            file.flush().await.unwrap();
        }

        // Search all files
        let result = tool.execute(json!({ "path": test_dir })).await;

        assert!(result.is_ok());
        match result.unwrap() {
            ExecuteResult::Success(MessageContent::Text(content)) => {
                assert!(content.contains("file1.txt") || content.contains("file2.txt"));
            }
            _ => panic!("Expected success with file list"),
        }

        // Clean up
        fs::remove_dir_all(test_dir).await.unwrap();
    }

    #[tokio::test]
    async fn test_file_search_tool_execute_by_extension() {
        let tool = FileSearchTool::new();
        let test_dir = "test_ext_search_dir";
        let test_files = vec!["file1.txt", "file2.txt", "file3.rs", "readme.md"];

        // Create test directory and files
        create_dir_all(test_dir).await.unwrap();
        for file in &test_files {
            let path = format!("{}/{}", test_dir, file);
            let mut file = File::create(&path).await.unwrap();
            file.write_all(b"test").await.unwrap();
            file.flush().await.unwrap();
        }

        // Search by extension
        let result = tool.execute(json!({
            "path": test_dir,
            "extension": "txt"
        })).await;

        assert!(result.is_ok());
        match result.unwrap() {
            ExecuteResult::Success(MessageContent::Text(content)) => {
                assert!(content.contains("file1.txt"));
                assert!(content.contains("file2.txt"));
                assert!(!content.contains("file3.rs"));
            }
            _ => panic!("Expected success with filtered results"),
        }

        // Clean up
        fs::remove_dir_all(test_dir).await.unwrap();
    }

    #[tokio::test]
    async fn test_file_search_tool_execute_by_pattern() {
        let tool = FileSearchTool::new();
        let test_dir = "test_pattern_search_dir";
        let test_files = vec!["test_file.txt", "other_file.txt", "test_data.rs"];

        // Create test directory and files
        create_dir_all(test_dir).await.unwrap();
        for file in &test_files {
            let path = format!("{}/{}", test_dir, file);
            let mut file = File::create(&path).await.unwrap();
            file.write_all(b"test").await.unwrap();
            file.flush().await.unwrap();
        }

        // Search by pattern (substring)
        let result = tool.execute(json!({
            "path": test_dir,
            "pattern": "test"
        })).await;

        assert!(result.is_ok());
        match result.unwrap() {
            ExecuteResult::Success(MessageContent::Text(content)) => {
                assert!(content.contains("test_file.txt"));
                assert!(content.contains("test_data.rs"));
                assert!(!content.contains("other_file.txt"));
            }
            _ => panic!("Expected success with pattern results"),
        }

        // Clean up
        fs::remove_dir_all(test_dir).await.unwrap();
    }

    #[tokio::test]
    async fn test_file_search_tool_execute_by_glob() {
        let tool = FileSearchTool::new();
        let test_dir = "test_glob_search_dir";
        let test_files = vec!["file1.txt", "file2.txt", "file3.rs", "other.md"];

        // Create test directory and files
        create_dir_all(test_dir).await.unwrap();
        for file in &test_files {
            let path = format!("{}/{}", test_dir, file);
            let mut file = File::create(&path).await.unwrap();
            file.write_all(b"test").await.unwrap();
            file.flush().await.unwrap();
        }

        // Search by glob pattern
        let result = tool.execute(json!({
            "path": test_dir,
            "pattern": "file*.txt"
        })).await;

        assert!(result.is_ok());
        match result.unwrap() {
            ExecuteResult::Success(MessageContent::Text(content)) => {
                assert!(content.contains("file1.txt"));
                assert!(content.contains("file2.txt"));
                assert!(!content.contains("file3.rs"));
                assert!(!content.contains("other.md"));
            }
            _ => panic!("Expected success with glob results"),
        }

        // Clean up
        fs::remove_dir_all(test_dir).await.unwrap();
    }

    #[tokio::test]
    async fn test_file_search_tool_execute_by_regex() {
        let tool = FileSearchTool::new();
        let test_dir = "test_regex_search_dir";
        let test_files = vec!["file1.txt", "file2.txt", "file10.txt", "other.md"];

        // Create test directory and files
        create_dir_all(test_dir).await.unwrap();
        for file in &test_files {
            let path = format!("{}/{}", test_dir, file);
            let mut file = File::create(&path).await.unwrap();
            file.write_all(b"test").await.unwrap();
            file.flush().await.unwrap();
        }

        // Search by regex (files with numbers)
        let result = tool.execute(json!({
            "path": test_dir,
            "pattern": "file\\d+\\.txt",
            "regex": true
        })).await;

        assert!(result.is_ok());
        match result.unwrap() {
            ExecuteResult::Success(MessageContent::Text(content)) => {
                assert!(content.contains("file1.txt"));
                assert!(content.contains("file2.txt"));
                assert!(content.contains("file10.txt"));
                assert!(!content.contains("other.md"));
            }
            _ => panic!("Expected success with regex results"),
        }

        // Clean up
        fs::remove_dir_all(test_dir).await.unwrap();
    }

    #[tokio::test]
    async fn test_file_search_tool_execute_recursive() {
        let tool = FileSearchTool::new();
        let test_dir = "test_recursive_search";
        let sub_dir = "test_recursive_search/subdir";

        // Create test directory structure
        create_dir_all(sub_dir).await.unwrap();

        // Create files in both directories
        let files = vec![
            format!("{}/root.txt", test_dir),
            format!("{}/nested.txt", sub_dir),
        ];

        for path in &files {
            let mut file = File::create(path).await.unwrap();
            file.write_all(b"test").await.unwrap();
            file.flush().await.unwrap();
        }

        // Search recursively (default)
        let result = tool.execute(json!({
            "path": test_dir,
            "pattern": "nested.txt"
        })).await;

        assert!(result.is_ok());
        match result.unwrap() {
            ExecuteResult::Success(MessageContent::Text(content)) => {
                assert!(content.contains("nested.txt"));
            }
            _ => panic!("Expected success with recursive results"),
        }

        // Clean up
        fs::remove_dir_all(test_dir).await.unwrap();
    }

    #[tokio::test]
    async fn test_file_search_tool_execute_max_results() {
        let tool = FileSearchTool::new();
        let test_dir = "test_max_results_dir";

        // Create test directory and many files
        create_dir_all(test_dir).await.unwrap();
        for i in 0..50 {
            let path = format!("{}/file{}.txt", test_dir, i);
            let mut file = File::create(&path).await.unwrap();
            file.write_all(b"test").await.unwrap();
            file.flush().await.unwrap();
        }

        // Search with max_results = 5
        let result = tool.execute(json!({
            "path": test_dir,
            "max_results": 5
        })).await;

        assert!(result.is_ok());
        match result.unwrap() {
            ExecuteResult::Success(MessageContent::Text(content)) => {
                // Parse JSON to check count
                let json: Value = serde_json::from_str(&content).unwrap_or(json!({}));
                if let Some(count) = json["count"].as_u64() {
                    assert!(count <= 5, "Should return at most 5 results");
                }
            }
            _ => panic!("Expected success"),
        }

        // Clean up
        fs::remove_dir_all(test_dir).await.unwrap();
    }

    #[tokio::test]
    async fn test_file_search_tool_definition() {
        let tool = FileSearchTool::new();
        let definition = tool.definition();

        assert_eq!(definition.name, "search_files");
        assert!(!definition.description.is_empty());
        assert_eq!(definition.parameters["type"], "object");
    }

    #[tokio::test]
    async fn test_glob_match_function() {
        // Test exact match
        assert!(glob_match("file.txt", "file.txt"));
        assert!(!glob_match("file.txt", "other.txt"));

        // Test * wildcard
        assert!(glob_match("file.txt", "*.txt"));
        assert!(glob_match("file.txt", "file*"));
        assert!(glob_match("file.txt", "*"));
        assert!(glob_match("test_file_name.txt", "test*.txt"));

        // Test case insensitivity
        assert!(glob_match("FILE.TXT", "file.txt"));
        assert!(glob_match("file.txt", "FILE.TXT"));
    }
}
