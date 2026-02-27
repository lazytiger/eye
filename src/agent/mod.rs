//! Agent module
//!
//! Manages the main agent logic, including:
//! - Agent struct definition
//! - Agent execution methods
//! - Tool call handling

use crate::{
    config::settings,
    interface::{self, Interface, MessageRole as InterfaceMessageRole},
    model::{self, ChatCompletionRequest, ChatMessage, MessageRole},
    skill::SkillManager,
    tool::ToolManager,
};
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Agent state
pub struct Agent {
    /// Model provider
    model_provider: Box<dyn model::ModelProvider>,
    /// Tool manager
    tool_manager: Arc<ToolManager>,
    /// Skill manager
    skill_manager: Arc<Mutex<SkillManager>>,
    /// Interface
    interface: Box<dyn Interface>,
    /// Configuration
    config: settings::Settings,
    /// Conversation history
    conversation_history: Vec<ChatMessage>,
}

impl Agent {
    /// Create new agent
    pub fn new(
        model_provider: Box<dyn model::ModelProvider>,
        tool_manager: Arc<ToolManager>,
        skill_manager: Arc<Mutex<SkillManager>>,
        interface: Box<dyn Interface>,
        config: settings::Settings,
    ) -> Self {
        Self {
            model_provider,
            tool_manager,
            skill_manager,
            interface,
            config,
            conversation_history: Vec::new(),
        }
    }

    /// Add message to conversation history
    fn add_message(&mut self, message: ChatMessage) {
        self.conversation_history.push(message);
    }

    /// Get conversation history
    fn get_conversation_history(&self) -> &[ChatMessage] {
        &self.conversation_history
    }

    /// Handle tool calls
    async fn handle_tool_calls(&self, tool_calls: &[model::ToolCall]) -> Result<Vec<ChatMessage>> {
        let mut tool_messages = Vec::new();

        for tool_call in tool_calls {
            // Display tool call
            self.interface
                .display_tool_call(&tool_call.name, &tool_call.arguments)
                .await?;

            // Execute tool
            let result = self
                .tool_manager
                .execute_tool(&tool_call.name, &tool_call.id, tool_call.arguments.clone())
                .await?;

            // Display tool result
            self.interface
                .display_tool_result(&result.tool_name, &result.result, result.success)
                .await?;

            // Create tool message
            let tool_message = ChatMessage {
                role: MessageRole::Tool,
                content: serde_json::to_string(&result.result)?,
                tool_calls: None,
            };

            tool_messages.push(tool_message);
        }

        Ok(tool_messages)
    }

    /// Interactive chat mode
    pub async fn chat_mode(&mut self, system_prompt: Option<String>) -> Result<()> {
        // Display welcome message
        self.interface
            .display_info("Welcome to Eye Personal Intelligent Assistant!")
            .await?;
        self.interface.display_info("Type 'exit' or 'quit' to exit, 'clear' to clear screen, 'history' to view conversation history").await?;

        // Add system prompt (if any)
        if let Some(prompt) = system_prompt {
            let system_message = ChatMessage {
                role: MessageRole::System,
                content: prompt,
                tool_calls: None,
            };
            self.add_message(system_message);
        }

        loop {
            // Get user input
            let user_input = self.interface.get_user_input().await?;

            // Handle special commands
            match user_input.to_lowercase().as_str() {
                "exit" | "quit" => {
                    self.interface.display_info("Goodbye!").await?;
                    break;
                }
                "clear" => {
                    self.interface.clear_screen().await?;
                    continue;
                }
                "history" => {
                    self.interface.display_info("Conversation history:").await?;
                    for (i, msg) in self.conversation_history.iter().enumerate() {
                        let role_str = match msg.role {
                            MessageRole::System => "System",
                            MessageRole::User => "User",
                            MessageRole::Assistant => "Assistant",
                            MessageRole::Tool => "Tool",
                        };
                        self.interface
                            .display_info(&format!("{}: {} - {}", i + 1, role_str, msg.content))
                            .await?;
                    }
                    continue;
                }
                _ => {}
            }

            // Create user message
            let user_message = ChatMessage {
                role: MessageRole::User,
                content: user_input.clone(),
                tool_calls: None,
            };

            // Display user message
            self.interface
                .display_message(&user_input, InterfaceMessageRole::User)
                .await?;

            // Add to conversation history
            self.add_message(user_message);

            // Get tool definitions
            let tool_definitions = self.tool_manager.get_tool_definitions();

            // Convert tool definitions to model format
            let model_tool_definitions: Vec<model::ToolDefinition> = tool_definitions
                .into_iter()
                .map(|td| model::ToolDefinition {
                    name: td.name,
                    description: td.description,
                    parameters: td.parameters,
                })
                .collect();

            // Build chat request
            let request = ChatCompletionRequest {
                messages: self.get_conversation_history().to_vec(),
                tools: Some(model_tool_definitions),
                temperature: Some(self.config.model.temperature),
                max_tokens: Some(self.config.model.max_tokens),
                stream: self.config.model.stream,
            };

            // Send request
            let response = self.model_provider.chat_completion(request).await?;

            // Display assistant message
            self.interface
                .display_message(&response.message.content, InterfaceMessageRole::Assistant)
                .await?;

            // Add to conversation history
            self.add_message(response.message.clone());

            // Handle tool calls
            if let Some(tool_calls) = &response.message.tool_calls {
                let tool_messages = self.handle_tool_calls(tool_calls).await?;

                // Add tool messages to conversation history
                for tool_message in tool_messages {
                    self.add_message(tool_message);
                }

                // If there are tool calls, need to continue conversation
                if !tool_calls.is_empty() {
                    // Resend request (including tool results)
                    let tool_definitions = self.tool_manager.get_tool_definitions();
                    let model_tool_definitions: Vec<model::ToolDefinition> = tool_definitions
                        .into_iter()
                        .map(|td| model::ToolDefinition {
                            name: td.name,
                            description: td.description,
                            parameters: td.parameters,
                        })
                        .collect();

                    let request = ChatCompletionRequest {
                        messages: self.get_conversation_history().to_vec(),
                        tools: Some(model_tool_definitions),
                        temperature: Some(self.config.model.temperature),
                        max_tokens: Some(self.config.model.max_tokens),
                        stream: self.config.model.stream,
                    };

                    let response = self.model_provider.chat_completion(request).await?;

                    // Display assistant message
                    self.interface
                        .display_message(&response.message.content, InterfaceMessageRole::Assistant)
                        .await?;

                    // Add to conversation history
                    self.add_message(response.message);
                }
            }

            // Display token usage
            if let Some(usage) = response.usage {
                self.interface
                    .display_usage(&interface::Usage {
                        prompt_tokens: usage.prompt_tokens,
                        completion_tokens: usage.completion_tokens,
                        total_tokens: usage.total_tokens,
                    })
                    .await?;
            }
        }

        Ok(())
    }

    /// Single query mode
    pub async fn query_mode(&mut self, query: &str) -> Result<()> {
        // Display query
        self.interface
            .display_message(query, InterfaceMessageRole::User)
            .await?;

        // Create user message
        let user_message = ChatMessage {
            role: MessageRole::User,
            content: query.to_string(),
            tool_calls: None,
        };

        // Add to conversation history
        self.add_message(user_message);

        // Get tool definitions
        let tool_definitions = self.tool_manager.get_tool_definitions();

        // Convert tool definitions to model format
        let model_tool_definitions: Vec<model::ToolDefinition> = tool_definitions
            .into_iter()
            .map(|td| model::ToolDefinition {
                name: td.name,
                description: td.description,
                parameters: td.parameters,
            })
            .collect();

        // Build chat request
        let request = ChatCompletionRequest {
            messages: self.get_conversation_history().to_vec(),
            tools: Some(model_tool_definitions),
            temperature: Some(self.config.model.temperature),
            max_tokens: Some(self.config.model.max_tokens),
            stream: self.config.model.stream,
        };

        // Send request
        let response = self.model_provider.chat_completion(request).await?;

        // Display assistant message
        self.interface
            .display_message(&response.message.content, InterfaceMessageRole::Assistant)
            .await?;

        // Handle tool calls
        if let Some(tool_calls) = &response.message.tool_calls {
            let tool_messages = self.handle_tool_calls(tool_calls).await?;

            // Add tool messages to conversation history
            for tool_message in tool_messages {
                self.add_message(tool_message);
            }

            // If there are tool calls, need to continue conversation
            if !tool_calls.is_empty() {
                // Resend request (including tool results)
                let tool_definitions = self.tool_manager.get_tool_definitions();
                let model_tool_definitions: Vec<model::ToolDefinition> = tool_definitions
                    .into_iter()
                    .map(|td| model::ToolDefinition {
                        name: td.name,
                        description: td.description,
                        parameters: td.parameters,
                    })
                    .collect();

                let request = ChatCompletionRequest {
                    messages: self.get_conversation_history().to_vec(),
                    tools: Some(model_tool_definitions),
                    temperature: Some(self.config.model.temperature),
                    max_tokens: Some(self.config.model.max_tokens),
                    stream: self.config.model.stream,
                };

                let response = self.model_provider.chat_completion(request).await?;

                // Display assistant message
                self.interface
                    .display_message(&response.message.content, InterfaceMessageRole::Assistant)
                    .await?;
            }
        }

        // Display token usage
        if let Some(usage) = response.usage {
            self.interface
                .display_usage(&interface::Usage {
                    prompt_tokens: usage.prompt_tokens,
                    completion_tokens: usage.completion_tokens,
                    total_tokens: usage.total_tokens,
                })
                .await?;
        }

        Ok(())
    }

    /// List tools
    pub async fn list_tools(&self) -> Result<()> {
        let tools = self.tool_manager.list_tools();

        self.interface.display_info("Available tools:").await?;
        for tool in tools {
            self.interface
                .display_info(&format!("  - {}", tool))
                .await?;
        }

        Ok(())
    }

    /// List skills
    pub async fn list_skills(&self) -> Result<()> {
        let skill_manager = self.skill_manager.lock().await;

        let skills = skill_manager.list_skills();

        self.interface.display_info("Available skills:").await?;
        for skill in skills {
            self.interface
                .display_info(&format!("  - {}", skill))
                .await?;
        }

        Ok(())
    }
}
