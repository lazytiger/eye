// Integration tests for eye crate providers
// These tests require real API keys and will make actual API calls

use eye::provider::{create_provider, ChatRequest, ChatMessage, MessageContent, EmbeddingRequest, EmbeddingInput};
use std::env;

/// Integration test for DeepSeek chat API
/// Requires DEEPSEEK_API_KEY environment variable
#[tokio::test]
#[ignore = "Requires real DeepSeek API key"]
async fn test_deepseek_chat() {
    let api_key = env::var("DEEPSEEK_API_KEY")
        .expect("DEEPSEEK_API_KEY environment variable must be set for this test");

    let provider = create_provider("deepseek", "deepseek-chat", &api_key)
        .expect("Failed to create DeepSeek provider");

    let request = ChatRequest {
        messages: vec![
            ChatMessage::User(eye::provider::UserMessage {
                content: MessageContent::Text("Hello, can you help me with a simple question?".to_string()),
                name: None,
            })
        ],
        model: Some("deepseek-chat".to_string()),
        temperature: Some(0.7),
        top_p: Some(0.9),
        stream: Some(false),
        max_tokens: Some(200),
        ..Default::default()
    };

    let response = provider.chat(request).await
        .expect("DeepSeek chat API call failed");

    assert!(!response.choices.is_empty(), "Response should contain at least one choice");
    println!("DeepSeek API call succeeded! Response: {:?}", response);
}

/// Integration test for OpenRouter chat API
/// Requires OPENROUTER_API_KEY environment variable
#[tokio::test]
#[ignore = "Requires real OpenRouter API key"]
async fn test_openrouter_chat() {
    let api_key = env::var("OPENROUTER_API_KEY")
        .expect("OPENROUTER_API_KEY environment variable must be set for this test");

    let provider = create_provider("openrouter", "deepseek/deepseek-v3.2", &api_key)
        .expect("Failed to create OpenRouter provider");

    let request = ChatRequest {
        messages: vec![
            ChatMessage::User(eye::provider::UserMessage {
                content: MessageContent::Text("Hello, can you help me with a simple question?".to_string()),
                name: None,
            })
        ],
        model: Some("deepseek/deepseek-v3.2".to_string()),
        temperature: Some(0.7),
        top_p: Some(0.9),
        stream: Some(false),
        max_tokens: Some(200),
        ..Default::default()
    };

    let response = provider.chat(request).await
        .expect("OpenRouter chat API call failed");

    assert!(!response.choices.is_empty(), "Response should contain at least one choice");
    println!("OpenRouter chat API call succeeded! Response: {:?}", response);
}

/// Integration test for OpenRouter embedding API
/// Requires OPENROUTER_API_KEY environment variable
#[tokio::test]
#[ignore = "Requires real OpenRouter API key"]
async fn test_openrouter_embedding() {
    let api_key = env::var("OPENROUTER_API_KEY")
        .expect("OPENROUTER_API_KEY environment variable must be set for this test");

    let provider = create_provider("openrouter", "openai/text-embedding-3-large", &api_key)
        .expect("Failed to create OpenRouter provider");

    let request = EmbeddingRequest {
        input: EmbeddingInput::StringArray(vec![
            "The quick brown fox jumps over the lazy dog".to_string(),
            "I love programming in Rust".to_string()
        ]),
        model: "openai/text-embedding-3-large".to_string(),
        encoding_format: None,
        dimensions: None,
        user: None,
    };

    let response = provider.embedding(request).await
        .expect("OpenRouter embedding API call failed");

    assert!(!response.data.is_empty(), "Response should contain embedding data");
    assert_eq!(response.data.len(), 2, "Should get 2 embeddings for 2 inputs");
    println!("OpenRouter embedding API call succeeded! Got {} embeddings", response.data.len());
}
