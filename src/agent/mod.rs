//! Agent module
//!
//! The core agent that orchestrates interactions between:
//! - Provider (LLM API)
//! - HistoryManager (conversation history)
//! - ToolManager (tool execution)
//! - SkillManager (skill execution)
//! - Interface (user I/O)

use crate::interface::Interface;
use crate::memory::history::HistoryManager;
use crate::provider::{ChatRequest, MessageContent, Provider, Tool, ToolCall as ProviderToolCall};
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
    ) -> Self {
        Self {
            provider,
            history,
            tool_manager,
            skill_manager,
            interface,
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
        // 1. Add user message to history
        self.history.add_user_message(user_input).await;

        // 2. Build request with current history
        let mut request = self.build_request().await?;

        // 3. Main conversation loop (handles tool calls)
        loop {
            // 4. Send request to LLM
            let response = self.provider.chat(request).await?;

            // 5. Get assistant message
            let assistant_message = &response.choices.first().unwrap().message;

            // 6. Record response to history
            self.history.on_response(&response).await;

            match &assistant_message.tool_calls {
                Some(tool_calls) if !tool_calls.is_empty() => {
                    // Execute all tool calls
                    self.execute_tool_calls(tool_calls).await?;

                    // Build new request with updated history (including tool results)
                    request = self.build_request().await?;
                    // Continue loop to send back to LLM
                }
                _ => {
                    // No tool calls, send final response to user

                    if let Some(content) = &assistant_message.content {
                        // Convert MessageContent to string for display
                        let content_str = match content {
                            MessageContent::Text(text) => text.clone(),
                            MessageContent::Parts(_) => "[Multimodal content]".to_string(),
                        };
                        self.interface.send(content_str).await?;
                    }
                    break; // Exit the loop
                }
            }
        }

        Ok(())
    }

    /// Build a chat request from current history
    async fn build_request(&self) -> Result<ChatRequest> {
        let messages = self.history.messages().await;

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

    /// Execute a list of tool calls from the LLM response
    async fn execute_tool_calls(&self, tool_calls: &[ProviderToolCall]) -> Result<()> {
        for tool_call in tool_calls {
            let tool_name = &tool_call.function.name;
            let args: serde_json::Value =
                serde_json::from_str(&tool_call.function.arguments).unwrap_or_default();

            tracing::info!("Executing tool: {} with args: {:?}", tool_name, args);

            // Execute the tool
            let result = self.tool_manager.execute_tool(tool_name, args).await?;

            // Format result for history
            let result_content = match result {
                ExecuteResult::Success(content) => match content {
                    MessageContent::Text(text) => format!("Success: {}", text),
                    MessageContent::Parts(_) => "Success: [Multimodal result]".to_string(),
                },
                ExecuteResult::Failure(error) => format!("Error: {}", error),
            };

            // Add tool result to history
            self.history
                .add_tool_result(&tool_call.id, MessageContent::Text(result_content.clone()))
                .await;

            tracing::info!("Tool {} executed, result: {}", tool_name, result_content);
        }

        Ok(())
    }
}
