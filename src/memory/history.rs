use crate::provider::{ChatMessage, ChatRequest, ChatResponse, MessageContent, Provider};
use crate::OptionToResult;
use anyhow::Context;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct HistoryManagerInner {
    messages: Vec<ChatMessage>,
    summary_provider: Arc<dyn Provider>,
    system: ChatMessage,
    user: ChatMessage,
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
        })))
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
