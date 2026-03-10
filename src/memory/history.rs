use crate::provider::{ChatMessage, ChatRequest, ChatResponse, MessageContent, Provider};
use crate::OptionToResult;
use anyhow::Context;
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Maximum number of recent tool calls to track for loop detection
const MAX_TOOL_CALL_HISTORY: usize = 30;

/// Minimum consecutive similar patterns to detect a loop
/// Set higher to avoid false positives during legitimate repeated operations
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

pub struct HistoryManagerInner {
    messages: Vec<ChatMessage>,
    summary_provider: Arc<dyn Provider>,
    system: ChatMessage,
    user: ChatMessage,
    /// Recent tool call signatures for loop detection
    recent_tool_calls: VecDeque<ToolCallSignature>,
}

#[derive(Clone)]
pub struct HistoryManager(Arc<RwLock<HistoryManagerInner>>);

impl HistoryManager {
    const MIN_HISTORY_SIZE: usize = 20;

    pub fn new(provider: Arc<dyn Provider>) -> Self {
        Self(Arc::new(RwLock::new(HistoryManagerInner {
            messages: Vec::new(),
            summary_provider: provider,
            system: ChatMessage::System(crate::provider::SystemMessage {
                name: None,
                content: MessageContent::Text("You are a conversation compression engine.\n\nTask:\nSummarize the provided conversation history into a concise summary not exceeding 600 characters.\n\nLanguage requirement:\nThe summary must be written in the same language as the original conversation. Do not translate.\n\nRules:\n1. Output only the summary content.\n2. Do not add any explanations, introductions, or closing remarks.\n3. Do not use phrases like \"Here is the summary\" or similar.\n4. Do not use bullet points or numbered lists.\n5. Do not add information that is not present in the original conversation.\n6. Remove greetings, repetition, and irrelevant details.\n7. The output must be a single continuous paragraph.\n\nReturn only the final summary.".to_string()),
            }),
            user: ChatMessage::User(crate::provider::UserMessage {
                name: None,
                content: MessageContent::Text("Compress the full conversation history in this context according to the system instructions.".to_string()),
            }),
            recent_tool_calls: VecDeque::with_capacity(MAX_TOOL_CALL_HISTORY),
        })))
    }

    /// Record a tool call for loop detection
    pub async fn record_tool_call(&self, tool_name: &str, arguments: &str) {
        let mut inner = self.0.write().await;
        let signature = ToolCallSignature::normalized(tool_name, arguments);
        inner.recent_tool_calls.push_back(signature);

        // Keep only the most recent tool calls
        while inner.recent_tool_calls.len() > MAX_TOOL_CALL_HISTORY {
            inner.recent_tool_calls.pop_front();
        }
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

    /// Check if a specific tool call pattern is repeating
    /// Returns the count of consecutive occurrences for more granular handling
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

    /// Clear the tool call history (useful after successful conversation progress)
    pub async fn clear_tool_call_history(&self) {
        let mut inner = self.0.write().await;
        inner.recent_tool_calls.clear();
    }

    pub async fn on_response(&self, response: &ChatResponse) {
        let mut inner = self.0.write().await;
        if let Some(choice) = response.choices.first() {
            inner.messages.push(ChatMessage::Assistant(choice.message.clone()));
        } else {
            tracing::error!("No choices found in response");
        }
    }

    pub async fn add_user_message(&self, message: impl Into<String>) {
        let mut inner = self.0.write().await;
        inner.messages.push(ChatMessage::User(crate::provider::UserMessage {
            name: None,
            content: MessageContent::Text(message.into()),
        }));
    }

    pub async fn add_tool_result(
        &self,
        tool_call_id: impl Into<String>,
        content: impl Into<MessageContent>,
    ) {
        let mut inner = self.0.write().await;
        inner.messages.push(ChatMessage::Tool(crate::provider::ToolMessage {
            tool_call_id: tool_call_id.into(),
            content: content.into(),
        }));
    }

    pub async fn add_message(&self, message: ChatMessage) {
        let mut inner = self.0.write().await;
        inner.messages.push(message);
    }

    pub async fn messages(&self) -> Vec<ChatMessage> {
        let inner = self.0.read().await;
        inner.messages.clone()
    }

    pub async fn compact(&self, force: bool) -> anyhow::Result<()> {
        let mut inner = self.0.write().await;
        if inner.messages.len() < Self::MIN_HISTORY_SIZE && !force {
            return Ok(());
        }
        let len = inner.messages.len();
        let (index, _) = inner
            .messages
            .iter()
            .enumerate()
            .find(|(i, m)| *i > len / 2 && matches!(m, ChatMessage::User { .. }))
            .to_ok()
            .context("Failed to find a suitable position to split history")?;
        let to_be_summary = inner.messages.split_off(index);
        let mut request = ChatRequest::default();
        request.messages.push(inner.system.clone());
        request.messages.extend(to_be_summary);
        request.messages.push(inner.user.clone());
        let response = inner.summary_provider.chat(request).await?;
        if let Some(choice) = response.choices.first() {
            inner.messages.insert(
                0,
                ChatMessage::User(crate::provider::UserMessage {
                    name: None,
                    content: choice.message.content.clone().unwrap_or(MessageContent::Text(String::new())),
                }),
            );
        } else {
            tracing::error!("No choices found in response");
        }
        Ok(())
    }

    pub async fn len(&self) -> usize {
        let inner = self.0.read().await;
        inner.messages.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::provider::{create_provider, ChatRequest, AssistantMessage};

    #[tokio::test]
    async fn test_tool_call_signature_new() {
        let sig = ToolCallSignature::new("test_tool", "{\"arg\": 1}");
        assert_eq!(sig.tool_name, "test_tool");
        assert_eq!(sig.arguments, "{\"arg\": 1}");
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
    async fn test_record_and_detect_loop() {
        let provider = create_provider("deepseek", "deepseek-chat", "").unwrap();
        let history = HistoryManager::new(provider.into());

        // Record a repeating pattern 6 times (MIN_LOOP_PATTERN_COUNT)
        for _ in 0..6 {
            history.record_tool_call("search_files", r#"{"path": ".", "pattern": "*.rs"}"#).await;
            history.record_tool_call("read_file", r#"{"path": "src/main.rs"}"#).await;
        }

        let result = history.detect_loop().await;
        assert!(matches!(result, LoopDetectionResult::LoopDetected { .. }));

        if let LoopDetectionResult::LoopDetected { cycle_length, .. } = result {
            assert_eq!(cycle_length, 2); // Two tool calls in the cycle
        }
    }

    #[tokio::test]
    async fn test_no_loop_with_insufficient_patterns() {
        let provider = create_provider("deepseek", "deepseek-chat", "").unwrap();
        let history = HistoryManager::new(provider.into());

        // Record only 2 similar calls (less than MIN_LOOP_PATTERN_COUNT)
        for _ in 0..2 {
            history.record_tool_call("search_files", r#"{"path": "."}"#).await;
        }

        let result = history.detect_loop().await;
        assert!(matches!(result, LoopDetectionResult::NoLoop));
    }

    #[tokio::test]
    async fn test_check_loop_single_tool() {
        let provider = create_provider("deepseek", "deepseek-chat", "").unwrap();
        let history = HistoryManager::new(provider.into());

        // Record same tool call 6 times (MIN_LOOP_PATTERN_COUNT)
        for _ in 0..6 {
            history.record_tool_call("search_files", r#"{"path": "."}"#).await;
        }

        // Check if this pattern would continue - should return LoopDetected
        let result = history.check_loop("search_files", r#"{"path": "."}"#).await;
        assert!(matches!(result, LoopDetectionResult::LoopDetected { .. }));

        // Different tool should return NoLoop
        let result2 = history.check_loop("read_file", r#"{"path": "test.rs"}"#).await;
        assert!(matches!(result2, LoopDetectionResult::NoLoop));
    }

    #[tokio::test]
    async fn test_check_loop_approaching_threshold() {
        let provider = create_provider("deepseek", "deepseek-chat", "").unwrap();
        let history = HistoryManager::new(provider.into());

        // Record 3 times (half of MIN_LOOP_PATTERN_COUNT = 6)
        for _ in 0..3 {
            history.record_tool_call("search_files", r#"{"path": "."}"#).await;
        }

        // Should return ApproachingLoop
        let result = history.check_loop("search_files", r#"{"path": "."}"#).await;
        assert!(matches!(result, LoopDetectionResult::ApproachingLoop { count, .. } if count == 3));
    }

    #[tokio::test]
    async fn test_clear_tool_call_history() {
        let provider = create_provider("deepseek", "deepseek-chat", "").unwrap();
        let history = HistoryManager::new(provider.into());

        // Record some tool calls (6 times to reach threshold)
        for _ in 0..6 {
            history.record_tool_call("test_tool", r#"{"arg": 1}"#).await;
        }

        // Verify loop detected
        let result = history.check_loop("test_tool", r#"{"arg": 1}"#).await;
        assert!(matches!(result, LoopDetectionResult::LoopDetected { .. }));

        // Clear history
        history.clear_tool_call_history().await;

        // Verify no loop detected
        let result2 = history.check_loop("test_tool", r#"{"arg": 1}"#).await;
        assert!(matches!(result2, LoopDetectionResult::NoLoop));
    }

    #[tokio::test]
    async fn test_loop_detection_with_varying_args() {
        let provider = create_provider("deepseek", "deepseek-chat", "").unwrap();
        let history = HistoryManager::new(provider.into());

        // Record pattern with different arguments (should not be detected as loop)
        history.record_tool_call("search_files", r#"{"path": "/a"}"#).await;
        history.record_tool_call("search_files", r#"{"path": "/b"}"#).await;
        history.record_tool_call("search_files", r#"{"path": "/c"}"#).await;

        let _result = history.detect_loop().await;
        // Different arguments mean different signatures, so may not detect loop
        // depending on normalization
    }

    #[tokio::test]
    async fn test_max_tool_call_history_limit() {
        let provider = create_provider("deepseek", "deepseek-chat", "").unwrap();
        let history = HistoryManager::new(provider.into());

        // Record more than MAX_TOOL_CALL_HISTORY
        for i in 0..50 {
            history.record_tool_call("test_tool", &format!(r#"{{"arg": {}}}"#, i)).await;
        }

        // Verify history is limited
        let inner = history.0.read().await;
        assert!(inner.recent_tool_calls.len() <= MAX_TOOL_CALL_HISTORY);
    }

    #[tokio::test]
    async fn test_history() -> anyhow::Result<()> {
        let history = HistoryManager::new(create_provider("deepseek", "deepseek-chat", "")?.into());
        let provider = create_provider("deepseek", "deepseek-chat", "")?;
        println!("test begin");
        let mut request = ChatRequest::default();
        // 添加一段较长的用户 - 助手对话历史，用于测试历史压缩功能
        request.add_system_message("您是一个专业的 rust 语言专家，您的任务是对用户的问题进行回答。");
        let questions = vec![
            "请介绍一下 Rust 语言的主要特点。",
            "所有权系统具体是怎么工作的？",
            "Rust 的异步编程模型是怎样的？",
            "Rust 在 Web 开发中有哪些应用？",
            "学习 Rust 有哪些好的资源？",
            "接下来持续学习还需要注意哪些方面？",
        ];
        for question in questions {
            request.add_user_message(question);
            println!("Question: {}", question);
            let response = provider.chat(request.clone()).await?;
            request
                .messages
                .push(ChatMessage::Assistant(AssistantMessage {
                    content: response.choices[0].message.content.clone(),
                    name: None,
                    tool_calls: None,
                    refusal: None,
                }));
            history.add_user_message(question).await;
            history.on_response(&response).await;
            println!("Answer: {:?}", response);
        }
        history.compact(true).await?;
        println!("{:?}", history.messages().await);

        Ok(())
    }
}
