//! Tests for the Provider trait implementation

use eye::provider::{ChatRequest, ChatMessage, Role, EmbeddingRequest, create_openai_compatible, Provider};
use anyhow::Result;

#[tokio::test]
async fn test_openai_provider() -> Result<()> {
    // Create an OpenAI provider (mock, no actual API key needed for this test)
    let provider = create_openai_compatible("openai", "test-key", "gpt-3.5-turbo")?;
    
    // Test name
    assert_eq!(provider.name(), "openai");
    
    // Test capabilities
    let capabilities = provider.capabilities();
    assert!(capabilities.contains(eye::provider::ModelCapabilities::TEXT_GENERATION));
    assert!(capabilities.contains(eye::provider::ModelCapabilities::FUNCTION_CALLING));
    
    // Test max context length
    let context_length = provider.max_context_length();
    assert!(context_length > 0);
    
    // Test chat (mock implementation)
    let request = ChatRequest {
        messages: vec![
            ChatMessage {
                role: Role::User,
                content: Some("Hello, world!".to_string()),
                name: None,
                tool_calls: None,
                tool_call_id: None,
            }
        ],
        model: "gpt-3.5-turbo".to_string(),
        temperature: Some(0.7),
        top_p: Some(1.0),
        stream: Some(false),
        tools: None,
        tool_choice: None,
        max_tokens: Some(100),
        n: Some(1),
        stop: None,
        frequency_penalty: Some(0.0),
        presence_penalty: Some(0.0),
        logit_bias: None,
        logprobs: None,
        top_logprobs: None,
        seed: None,
        user: None,
        response_format: None,
        parallel_tool_calls: None,
        stream_options: None,
    };
    
    let response = provider.chat(request).await?;
    assert_eq!(response.model, "gpt-3.5-turbo");
    assert!(!response.choices.is_empty());
    
    // Test embedding (mock implementation)
    let embedding_request = EmbeddingRequest {
        input: vec!["test text".to_string()],
        model: "text-embedding-ada-002".to_string(),
        encoding_format: None,
        dimensions: None,
        user: None,
    };
    
    let embedding_response = provider.embedding(embedding_request).await?;
    assert_eq!(embedding_response.model, "gpt-3.5-turbo");
    assert!(!embedding_response.data.is_empty());
    
    Ok(())
}

#[tokio::test]
async fn test_openrouter_provider() -> Result<()> {
    // Create an OpenRouter provider
    let provider = create_openai_compatible("openrouter", "test-key", "openai/gpt-3.5-turbo")?;
    
    assert_eq!(provider.name(), "openrouter");
    
    // Test capabilities
    let capabilities = provider.capabilities();
    assert!(capabilities.contains(eye::provider::ModelCapabilities::TEXT_GENERATION));
    
    Ok(())
}

#[tokio::test]
async fn test_deepseek_provider() -> Result<()> {
    // Create a DeepSeek provider
    let provider = create_openai_compatible("deepseek", "test-key", "deepseek-chat")?;
    
    assert_eq!(provider.name(), "deepseek");
    
    // Test capabilities
    let capabilities = provider.capabilities();
    assert!(capabilities.contains(eye::provider::ModelCapabilities::TEXT_GENERATION));
    
    Ok(())
}