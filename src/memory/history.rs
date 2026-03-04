use crate::provider::{ChatMessageContent, Content, Message, Provider, Request, Response};
use crate::OptionToResult;
use anyhow::Context;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct HistoryManagerInner {
    messages: Vec<Message>,
    summary_provider: Box<dyn Provider>,
    system: Message,
    user: Message,
}

#[derive(Clone)]
pub struct HistoryManager(Arc<RwLock<HistoryManagerInner>>);

impl HistoryManager {
    const MIN_HISTORY_SIZE: usize = 20;
    pub fn new<T: Provider + 'static>(provider: T) -> Self {
        Self(Arc::new(RwLock::new(HistoryManagerInner {
            messages: Vec::new(),
            summary_provider: Box::new(provider),
            system: Message::System {
                name: None,
                content: r#"You are a conversation compression engine.

Task:
Summarize the provided conversation history into a concise summary not exceeding 600 characters.

Language requirement:
The summary must be written in the same language as the original conversation. Do not translate.

Rules:
1. Output only the summary content.
2. Do not add any explanations, introductions, or closing remarks.
3. Do not use phrases like “Here is the summary” or similar.
4. Do not use bullet points or numbered lists.
5. Do not add information that is not present in the original conversation.
6. Remove greetings, repetition, and irrelevant details.
7. The output must be a single continuous paragraph.

Return only the final summary."#.to_string().into(),
            },
            user: Message::User {
                name: None,
                content: "Compress the full conversation history in this context according to the system instructions.".to_string().into(),
            },
        })))
    }

    pub async fn on_response(&self, response: &Response) {
        let mut inner = self.0.write().await;
        if let Some(choice) = response.choices.first() {
            inner.messages.push(choice.message.clone().into());
        } else {
            tracing::error!("No choices found in response");
        }
    }

    pub async fn add_user_message(&self, message: impl Into<String>) {
        let mut inner = self.0.write().await;
        inner.messages.push(Message::User {
            name: None,
            content: message.into().into(),
        });
    }

    pub async fn add_tool_result(
        &self,
        tool_call_id: impl Into<String>,
        content: impl Into<Content<ChatMessageContent>>,
    ) {
        let mut inner = self.0.write().await;
        inner.messages.push(Message::Tool {
            tool_call_id: tool_call_id.into(),
            content: content.into(),
        });
    }

    pub async fn messages(&self) -> Vec<Message> {
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
            .find(|(i, m)| *i > len / 2 && matches!(m, Message::User { .. }))
            .to_ok()
            .context("Failed to find a suitable position to split history")?;
        let to_be_summary = inner.messages.split_off(index);
        let mut request = Request::new();
        request.messages.push(inner.system.clone());
        request.messages.extend(to_be_summary);
        request.messages.push(inner.user.clone());
        let response = inner.summary_provider.chat(request).await?;
        if let Some(choice) = response.choices.first() {
            inner.messages.insert(
                0,
                Message::User {
                    name: None,
                    content: choice.message.content.clone(),
                },
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
    use crate::provider::create_openai_compatible;

    #[tokio::test]
    async fn test_history() -> anyhow::Result<()> {
        let history =
            HistoryManager::new(create_openai_compatible("deepseek", "", "deepseek-chat")?);
        let provider = create_openai_compatible("deepseek", "", "deepseek-chat")?;
        println!("test begin");
        let mut request = Request::new();
        // 添加一段较长的用户-助手对话历史，用于测试历史压缩功能
        request.add_system_message("您是一个专业的rust语言专家， 您的任务是对用户的问题进行回答。");
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
                .push(response.choices[0].message.clone().into());
            history.add_user_message(question).await;
            history.on_response(&response).await;
            println!("Answer: {:?}", response);
        }
        history.compact(true).await?;
        println!("{:?}", history.messages().await);

        Ok(())
    }
}
