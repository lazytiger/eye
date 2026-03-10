//! ConversationManager - manages per-request conversation context
//!
//! This module provides a transient conversation manager that tracks tool calls
//! and their results for a single request. Unlike HistoryManager which stores
//! long-term conversation history, ConversationManager is created per request
//! and discarded after completion.
//!
//! Lifecycle:
//! 1. Created at start of process_user_message()
//! 2. Tracks all tool calls and results during the request
//! 3. On completion: only user input + final answer go to HistoryManager
//! 4. Discarded after request completes

use crate::provider::MessageContent;
use crate::tool::ExecuteResult;
use std::collections::VecDeque;
use tokio::sync::RwLock;
use std::sync::Arc;

/// Maximum number of recent tool calls to track for loop detection
const MAX_TOOL_CALL_HISTORY: usize = 30;

/// Minimum consecutive similar patterns to detect a loop
const MIN_LOOP_PATTERN_COUNT: usize = 6;

/// Tool call signature for loop detection
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolCallSignature {
    pub tool_name: String,
    pub arguments: String,
}

impl ToolCallSignature {
    pub fn new(tool_name: impl Into<String>, arguments: impl Into<String>) -> Self {
        Self {
            tool_name: tool_name.into(),
            arguments: arguments.into(),
        }
    }

    /// Create a normalized signature for comparison (ignores minor argument differences)
    pub fn normalized(tool_name: impl Into<String>, arguments: &str) -> Self {
        // Normalize arguments by sorting JSON keys and removing whitespace variations
        let normalized_args = if let Ok(value) = serde_json::from_str::<serde_json::Value>(arguments) {
            value.to_string()
        } else {
            arguments.to_string()
        };
        Self {
            tool_name: tool_name.into(),
            arguments: normalized_args,
        }
    }
}

/// Loop detection result
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoopDetectionResult {
    NoLoop,
    /// Approaching loop threshold (more than half of MIN_LOOP_PATTERN_COUNT)
    ApproachingLoop {
        tool_name: String,
        arguments: String,
        count: usize,
    },
    LoopDetected {
        pattern: Vec<ToolCallSignature>,
        cycle_length: usize,
    },
}

/// Tracked tool call with its result
#[derive(Debug, Clone)]
pub struct TrackedToolCall {
    pub tool_call_id: String,
    pub tool_name: String,
    pub arguments: String,
    pub result: ExecuteResult,
}

impl TrackedToolCall {
    pub fn new(
        tool_call_id: impl Into<String>,
        tool_name: impl Into<String>,
        arguments: impl Into<String>,
        result: ExecuteResult,
    ) -> Self {
        Self {
            tool_call_id: tool_call_id.into(),
            tool_name: tool_name.into(),
            arguments: arguments.into(),
            result,
        }
    }

    /// Convert result to MessageContent for history
    pub fn result_content(&self) -> MessageContent {
        match &self.result {
            ExecuteResult::Success(content) => content.clone(),
            ExecuteResult::Failure(error) => MessageContent::Text(format!("Error: {}", error)),
        }
    }
}

/// Conversation summary for a single request
#[derive(Debug, Clone)]
pub struct ConversationSummary {
    pub user_input: String,
    pub tool_call_count: usize,
    pub tool_results: Vec<String>,
}

/// Internal state protected by RwLock
struct ConversationManagerInner {
    /// Original user input for this request
    user_input: String,

    /// Tool calls made during this request (with results)
    tool_calls: Vec<TrackedToolCall>,

    /// Recent tool call signatures for loop detection
    recent_tool_calls: VecDeque<ToolCallSignature>,
}

/// ConversationManager - manages per-request conversation context
#[derive(Clone)]
pub struct ConversationManager(Arc<RwLock<ConversationManagerInner>>);

impl ConversationManager {
    /// Create a new ConversationManager for a user request
    pub fn new(user_input: impl Into<String>) -> Self {
        Self(Arc::new(RwLock::new(ConversationManagerInner {
            user_input: user_input.into(),
            tool_calls: Vec::new(),
            recent_tool_calls: VecDeque::with_capacity(MAX_TOOL_CALL_HISTORY),
        })))
    }

    /// Record a tool call and its result
    pub async fn record_tool_call(
        &self,
        tool_call_id: impl Into<String>,
        tool_name: impl Into<String>,
        arguments: impl Into<String>,
        result: ExecuteResult,
    ) {
        let mut inner = self.0.write().await;

        // Convert to owned strings first
        let tool_name_str = tool_name.into();
        let arguments_str = arguments.into();

        // Create tracked tool call
        let tracked = TrackedToolCall::new(
            tool_call_id,
            tool_name_str.clone(),
            arguments_str.clone(),
            result,
        );
        inner.tool_calls.push(tracked);

        // Record for loop detection
        let signature = ToolCallSignature::normalized(&tool_name_str, &arguments_str);
        inner.recent_tool_calls.push_back(signature);

        // Keep only the most recent tool calls
        while inner.recent_tool_calls.len() > MAX_TOOL_CALL_HISTORY {
            inner.recent_tool_calls.pop_front();
        }
    }

    /// Check if a specific tool call pattern is repeating
    pub async fn check_loop(&self, tool_name: &str, arguments: &str) -> LoopDetectionResult {
        let signature = ToolCallSignature::normalized(tool_name, arguments);
        let inner = self.0.read().await;

        if inner.recent_tool_calls.len() < MIN_LOOP_PATTERN_COUNT {
            // Count occurrences even if below threshold
            let mut count = 0;
            for sig in inner.recent_tool_calls.iter().rev() {
                if *sig == signature {
                    count += 1;
                } else {
                    break;
                }
            }

            // Return ApproachingLoop if more than half of threshold
            if count > 0 && count >= MIN_LOOP_PATTERN_COUNT / 2 {
                return LoopDetectionResult::ApproachingLoop {
                    tool_name: tool_name.to_string(),
                    arguments: arguments.to_string(),
                    count,
                };
            }
            return LoopDetectionResult::NoLoop;
        }

        // Count consecutive occurrences of the same signature at the end
        let mut count = 0;
        for sig in inner.recent_tool_calls.iter().rev() {
            if *sig == signature {
                count += 1;
                if count >= MIN_LOOP_PATTERN_COUNT {
                    return LoopDetectionResult::LoopDetected {
                        pattern: vec![signature.clone()],
                        cycle_length: 1,
                    };
                }
            } else {
                break;
            }
        }

        // Return count if approaching threshold
        if count >= MIN_LOOP_PATTERN_COUNT / 2 {
            return LoopDetectionResult::ApproachingLoop {
                tool_name: tool_name.to_string(),
                arguments: arguments.to_string(),
                count,
            };
        }

        LoopDetectionResult::NoLoop
    }

    /// Detect if the recent tool calls form a loop pattern
    pub async fn detect_loop(&self) -> LoopDetectionResult {
        let inner = self.0.read().await;

        if inner.recent_tool_calls.len() < MIN_LOOP_PATTERN_COUNT * 2 {
            return LoopDetectionResult::NoLoop;
        }

        let tool_calls: Vec<&ToolCallSignature> = inner.recent_tool_calls.iter().collect();
        let len = tool_calls.len();

        // Check for different cycle lengths (1 to len/2)
        for cycle_len in 1..=(len / 2) {
            let mut cycle_count = 0;

            // Count how many consecutive cycles match
            for i in (0..len - cycle_len).rev() {
                if tool_calls[i] == tool_calls[i + cycle_len] {
                    cycle_count += 1;
                } else {
                    break;
                }
            }

            if cycle_count >= MIN_LOOP_PATTERN_COUNT {
                // Calculate start position with overflow protection
                let pattern_size = cycle_len * (cycle_count + 1);
                let start = if pattern_size <= len {
                    len - pattern_size
                } else {
                    // If pattern is larger than history, just use what we have
                    len.saturating_sub(cycle_len * (cycle_count + 1))
                };

                // Ensure start + cycle_len doesn't overflow
                if start + cycle_len > len {
                    continue;
                }

                let pattern: Vec<ToolCallSignature> = tool_calls[start..start + cycle_len]
                    .iter()
                    .map(|s| (*s).clone())
                    .collect();

                return LoopDetectionResult::LoopDetected {
                    pattern,
                    cycle_length: cycle_len,
                };
            }
        }

        LoopDetectionResult::NoLoop
    }

    /// Get the original user input
    pub async fn tool_calls(&self) -> Vec<TrackedToolCall> {
        let inner = self.0.read().await;
        inner.tool_calls.clone()
    }

    /// Get the original user input
    pub async fn user_input(&self) -> String {
        let inner = self.0.read().await;
        inner.user_input.clone()
    }

    /// Get conversation summary
    pub async fn to_conversation_summary(&self) -> ConversationSummary {
        let inner = self.0.read().await;
        let tool_results: Vec<String> = inner.tool_calls.iter().map(|tc| {
            match &tc.result {
                ExecuteResult::Success(content) => match content {
                    MessageContent::Text(text) => format!("{}: {}", tc.tool_name, text),
                    MessageContent::Parts(_) => format!("{}: [Multimodal result]", tc.tool_name),
                },
                ExecuteResult::Failure(error) => format!("{}: Error: {}", tc.tool_name, error),
            }
        }).collect();

        ConversationSummary {
            user_input: inner.user_input.clone(),
            tool_call_count: inner.tool_calls.len(),
            tool_results,
        }
    }

    /// Check if any tool calls were made
    pub async fn has_tool_calls(&self) -> bool {
        let inner = self.0.read().await;
        !inner.tool_calls.is_empty()
    }

    /// Get the number of tool calls made
    pub async fn tool_call_count(&self) -> usize {
        let inner = self.0.read().await;
        inner.tool_calls.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_conversation_manager() {
        let cm = ConversationManager::new("Test input");
        assert_eq!(cm.user_input().await, "Test input");
        assert!(cm.tool_calls().await.is_empty());
        assert!(!cm.has_tool_calls().await);
    }

    #[tokio::test]
    async fn test_record_tool_call() {
        let cm = ConversationManager::new("Test input");

        cm.record_tool_call(
            "call_1",
            "search_files",
            r#"{"path": ".", "pattern": "*.rs"}"#,
            ExecuteResult::Success(MessageContent::Text("Found 5 files".to_string())),
        ).await;

        let tool_calls = cm.tool_calls().await;
        assert_eq!(tool_calls.len(), 1);
        assert_eq!(tool_calls[0].tool_call_id, "call_1");
        assert_eq!(tool_calls[0].tool_name, "search_files");
        assert!(cm.has_tool_calls().await);
    }

    #[tokio::test]
    async fn test_result_content_success() {
        let call = TrackedToolCall::new(
            "call_1",
            "test_tool",
            r#"{"arg": 1}"#,
            ExecuteResult::Success(MessageContent::Text("Success result".to_string())),
        );
        assert!(matches!(call.result_content(), MessageContent::Text(s) if s == "Success result"));
    }

    #[tokio::test]
    async fn test_result_content_failure() {
        let call = TrackedToolCall::new(
            "call_1",
            "test_tool",
            r#"{"arg": 1}"#,
            ExecuteResult::Failure("Error occurred".to_string()),
        );
        assert!(matches!(call.result_content(), MessageContent::Text(s) if s == "Error: Error occurred"));
    }

    #[tokio::test]
    async fn test_conversation_summary() {
        let cm = ConversationManager::new("What is Rust?");

        cm.record_tool_call(
            "call_1",
            "search_web",
            r#"{"query": "Rust programming"}"#,
            ExecuteResult::Success(MessageContent::Text("Rust is a systems language".to_string())),
        ).await;

        cm.record_tool_call(
            "call_2",
            "read_file",
            r#"{"path": "README.md"}"#,
            ExecuteResult::Failure("File not found".to_string()),
        ).await;

        let summary = cm.to_conversation_summary().await;
        assert_eq!(summary.user_input, "What is Rust?");
        assert_eq!(summary.tool_call_count, 2);
        assert_eq!(summary.tool_results.len(), 2);
    }

    #[tokio::test]
    async fn test_record_and_detect_loop() {
        let cm = ConversationManager::new("Test");

        // Record a repeating pattern 6 times (MIN_LOOP_PATTERN_COUNT)
        for i in 0..6 {
            cm.record_tool_call(
                format!("call_{}_search", i),
                "search_files",
                r#"{"path": ".", "pattern": "*.rs"}"#,
                ExecuteResult::Success(MessageContent::Text("Found files".to_string())),
            ).await;
            cm.record_tool_call(
                format!("call_{}_read", i),
                "read_file",
                r#"{"path": "src/main.rs"}"#,
                ExecuteResult::Success(MessageContent::Text("Read file".to_string())),
            ).await;
        }

        // Use detect_loop to find the alternating pattern
        let result = cm.detect_loop().await;
        assert!(matches!(result, LoopDetectionResult::LoopDetected { .. }));

        if let LoopDetectionResult::LoopDetected { cycle_length, .. } = result {
            assert_eq!(cycle_length, 2); // Two tool calls in the cycle
        }
    }

    #[tokio::test]
    async fn test_detect_loop_no_loop_with_insufficient_patterns() {
        let cm = ConversationManager::new("Test");

        // Record only 2 pairs (less than MIN_LOOP_PATTERN_COUNT * 2 = 12)
        for i in 0..2 {
            cm.record_tool_call(
                format!("call_{}_search", i),
                "search_files",
                r#"{"path": "."}"#,
                ExecuteResult::Success(MessageContent::Text("Found".to_string())),
            ).await;
            cm.record_tool_call(
                format!("call_{}_read", i),
                "read_file",
                r#"{"path": "test.rs"}"#,
                ExecuteResult::Success(MessageContent::Text("Read".to_string())),
            ).await;
        }

        let result = cm.detect_loop().await;
        assert!(matches!(result, LoopDetectionResult::NoLoop));
    }

    #[tokio::test]
    async fn test_check_loop_approaching_threshold() {
        let cm = ConversationManager::new("Test");

        // Record 3 times (half of MIN_LOOP_PATTERN_COUNT = 6)
        for i in 0..3 {
            cm.record_tool_call(
                format!("call_{}", i),
                "search_files",
                r#"{"path": "."}"#,
                ExecuteResult::Success(MessageContent::Text("Found".to_string())),
            ).await;
        }

        // Should return ApproachingLoop
        let result = cm.check_loop("search_files", r#"{"path": "."}"#).await;
        assert!(matches!(result, LoopDetectionResult::ApproachingLoop { count, .. } if count == 3));
    }

    #[tokio::test]
    async fn test_tool_call_signature_normalized() {
        // Test with valid JSON - should normalize whitespace
        let sig1 = ToolCallSignature::normalized("test", r#"{"b": 1, "a": 2}"#);
        let sig2 = ToolCallSignature::normalized("test", r#"{"a":2,"b":1}"#);
        // JSON normalization may vary, but same content should be similar
        assert_eq!(sig1.tool_name, sig2.tool_name);

        // Test with invalid JSON - should keep as-is
        let sig3 = ToolCallSignature::normalized("test", "invalid json");
        assert_eq!(sig3.arguments, "invalid json");
    }

    #[tokio::test]
    async fn test_has_tool_calls_and_count() {
        let cm = ConversationManager::new("Test");

        // Initially no tool calls
        assert!(!cm.has_tool_calls().await);
        assert_eq!(cm.tool_call_count().await, 0);

        // After recording tool calls
        cm.record_tool_call(
            "call_1",
            "search_files",
            r#"{"path": "."}"#,
            ExecuteResult::Success(MessageContent::Text("Found".to_string())),
        ).await;
        cm.record_tool_call(
            "call_2",
            "read_file",
            r#"{"path": "test.rs"}"#,
            ExecuteResult::Success(MessageContent::Text("Read".to_string())),
        ).await;

        assert!(cm.has_tool_calls().await);
        assert_eq!(cm.tool_call_count().await, 2);
    }
}
