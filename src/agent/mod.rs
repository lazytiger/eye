//! Agent module
//!
//! The core agent that orchestrates interactions between:
//! - Provider (LLM API)
//! - HistoryManager (conversation history)
//! - ToolManager (tool execution)
//! - SkillManager (skill execution)
//! - Interface (user I/O)

use crate::interface::Interface;
use crate::memory::conversation::ConversationManager;
use crate::memory::history::HistoryManager;
use crate::provider::{
    ChatMessage, ChatRequest, MessageContent, Provider, Tool, ToolCall as ProviderToolCall,
    UserMessage,
};
use crate::skill::SkillManager;
use crate::tool::{ExecuteResult, ToolManager};
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::mpsc;

/// Agent that manages conversation flow and tool execution
pub struct Agent {
    provider: Arc<dyn Provider>,
    history: HistoryManager,
    tool_manager: Arc<ToolManager>,
    skill_manager: Arc<SkillManager>,
    interface: Arc<dyn Interface>,
    system_prompt: String,
}

impl Agent {
    /// Create a new Agent instance
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        provider: Arc<dyn Provider>,
        history: HistoryManager,
        tool_manager: Arc<ToolManager>,
        skill_manager: Arc<SkillManager>,
        interface: Arc<dyn Interface>,
        system_prompt: String,
    ) -> Self {
        Self {
            provider,
            history,
            tool_manager,
            skill_manager,
            interface,
            system_prompt,
        }
    }

    /// Run the agent's main loop
    pub async fn run(&self) -> Result<()> {
        // Send welcome message
        self.interface
            .send(
                "Welcome! I'm Eye, your personal AI assistant. How can I help you today?"
                    .to_string(),
            )
            .await?;

        // Create channel for receiving user input
        let (tx, mut rx) = mpsc::channel::<String>(32);

        // Spawn listener task
        let interface_clone = self.interface.clone();
        let _listener_handle = tokio::spawn(async move {
            if let Err(e) = interface_clone.listen(tx).await {
                tracing::error!("Listener error: {}", e);
            }
        });

        // Main loop
        while let Some(user_input) = rx.recv().await {
            if let Err(e) = self.process_user_message(&user_input).await {
                tracing::error!("Error processing message: {}", e);
                self.interface
                    .send(format!("Error: {}", e))
                    .await
                    .unwrap_or_default();
            }
        }

        Ok(())
    }

    /// Process a single user message through the full agent loop
    async fn process_user_message(&self, user_input: &str) -> Result<()> {
        tracing::info!("Processing user message: {}", user_input);

        // Create per-request conversation manager
        let conversation = ConversationManager::new(user_input);

        // 1. Build initial request with long-term history + current user input
        let mut request = self.build_request_with_conversation(&conversation).await?;

        tracing::debug!(
            "Initial request built with {} messages",
            request.messages.len()
        );

        // 2. Main conversation loop (handles tool calls)
        let mut loop_count = 0;
        loop {
            loop_count += 1;
            tracing::info!("Conversation loop iteration {}", loop_count);

            // 3. Send request to LLM
            tracing::debug!("Sending request to LLM...");
            let chat_start = std::time::Instant::now();
            let response = self.provider.chat(request).await?;
            tracing::info!("LLM response received in {:?}", chat_start.elapsed());

            // 4. Get assistant message
            let assistant_message = &response.choices.first().unwrap().message;

            match &assistant_message.tool_calls {
                Some(tool_calls) if !tool_calls.is_empty() => {
                    tracing::info!("Received {} tool call(s)", tool_calls.len());

                    // Execute all tool calls (tracked in conversation manager)
                    let exec_start = std::time::Instant::now();
                    self.execute_tool_calls_with_conversation(tool_calls, &conversation)
                        .await?;
                    tracing::info!("Tool execution completed in {:?}", exec_start.elapsed());

                    // Build new request with updated tool results from conversation
                    request = self.build_request_with_conversation(&conversation).await?;
                    tracing::debug!("New request built with {} messages", request.messages.len());

                    // Continue loop to send back to LLM
                }
                _ => {
                    tracing::info!("No tool calls, sending final response to user");

                    if let Some(content) = &assistant_message.content {
                        // Convert MessageContent to string for display
                        let content_str = match content {
                            MessageContent::Text(text) => text.clone(),
                            MessageContent::Parts(_) => "[Multimodal content]".to_string(),
                        };
                        self.interface.send(content_str).await?;
                    }

                    // Request completed: only add user input and final answer to long-term history
                    self.history.add_user_message(user_input).await;
                    self.history
                        .add_message(ChatMessage::Assistant(assistant_message.clone()))
                        .await;

                    tracing::info!(
                        "Request completed: added to history (user input + final answer)"
                    );
                    break; // Exit the loop
                }
            }
        }

        tracing::info!("Message processing completed");
        Ok(())
    }

    /// Build a chat request from current history and conversation context
    async fn build_request_with_conversation(
        &self,
        conversation: &ConversationManager,
    ) -> Result<ChatRequest> {
        let mut messages = self.history.messages().await;

        // Add system prompt at the beginning
        messages.insert(0, ChatMessage::System(crate::provider::SystemMessage {
            name: None,
            content: MessageContent::Text(self.system_prompt.clone()),
        }));

        // Add current user input from conversation
        let user_input = conversation.user_input().await;
        messages.push(ChatMessage::User(UserMessage {
            name: None,
            content: MessageContent::Text(user_input),
        }));

        // Add tool calls and results from current conversation
        let tool_calls = conversation.tool_calls().await;
        if !tool_calls.is_empty() {
            tracing::debug!(
                "Adding {} tool call(s) from conversation to request",
                tool_calls.len()
            );

            // Group tool calls and results into the request messages
            // The LLM expects: assistant message with tool_calls, then tool result messages
            for tracked_call in tool_calls {
                // Add assistant message with tool call
                let provider_tool_call = ProviderToolCall {
                    id: tracked_call.tool_call_id.clone(),
                    type_: crate::provider::ToolType::Function,
                    function: crate::provider::FunctionCall {
                        name: tracked_call.tool_name.clone(),
                        arguments: tracked_call.arguments.clone(),
                    },
                };

                messages.push(ChatMessage::Assistant(crate::provider::AssistantMessage {
                    content: None,
                    name: None,
                    tool_calls: Some(vec![provider_tool_call]),
                    refusal: None,
                }));

                // Add tool result
                messages.push(ChatMessage::Tool(crate::provider::ToolMessage {
                    tool_call_id: tracked_call.tool_call_id.clone(),
                    content: tracked_call.result_content(),
                }));
            }
        }

        // Get tool definitions if tools are available
        let tools = if !self.tool_manager.list_tools().is_empty() {
            Some(
                self.tool_manager
                    .get_tool_definitions()
                    .into_iter()
                    .map(|def| Tool {
                        type_: crate::provider::ToolType::Function,
                        function: crate::provider::FunctionDefinition {
                            name: def.name,
                            description: Some(def.description),
                            parameters: def.parameters,
                            strict: None,
                        },
                    })
                    .collect(),
            )
        } else {
            None
        };

        Ok(ChatRequest {
            messages,
            tools,
            ..Default::default()
        })
    }

    /// Execute a list of tool calls and track them in the conversation manager
    async fn execute_tool_calls_with_conversation(
        &self,
        tool_calls: &[ProviderToolCall],
        conversation: &ConversationManager,
    ) -> Result<()> {
        for tool_call in tool_calls {
            let tool_name = &tool_call.function.name;
            let args: serde_json::Value =
                serde_json::from_str(&tool_call.function.arguments).unwrap_or_default();

            tracing::info!("Executing tool: {} with args: {:?}", tool_name, args);

            // Execute the tool first
            let result = self.tool_manager.execute_tool(tool_name, args).await?;

            // Record tool call in conversation manager (for per-request tracking)
            conversation
                .record_tool_call(
                    &tool_call.id,
                    tool_name,
                    &tool_call.function.arguments,
                    result.clone(),
                )
                .await;

            // Check for loop (but don't abort, just log warning)
            match conversation
                .check_loop(tool_name, &tool_call.function.arguments)
                .await
            {
                crate::memory::conversation::LoopDetectionResult::ApproachingLoop {
                    tool_name: tn,
                    count,
                    ..
                } => {
                    tracing::warn!(
                        "Loop warning: Tool '{}' has been called {} times consecutively. LLM may be approaching a loop.",
                        tn,
                        count
                    );
                }
                crate::memory::conversation::LoopDetectionResult::LoopDetected {
                    cycle_length,
                    ..
                } => {
                    tracing::warn!(
                        "Loop detected: Tool call pattern repeating with cycle length {}. Continuing with caution.",
                        cycle_length
                    );
                }
                crate::memory::conversation::LoopDetectionResult::NoLoop => {}
            }

            // Format result for logging
            let result_content = match &result {
                ExecuteResult::Success(content) => match content {
                    MessageContent::Text(text) => format!("Success: {}", text),
                    MessageContent::Parts(_) => "Success: [Multimodal result]".to_string(),
                },
                ExecuteResult::Failure(error) => format!("Error: {}", error),
            };

            tracing::info!("Tool {} executed, result: {}", tool_name, result_content);
        }

        Ok(())
    }
}
