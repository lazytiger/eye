// Integration tests for eye crate providers
// These tests require real API keys and will make actual API calls

use eye::provider::create_provider;
use eye::provider::{ChatRequest, ChatMessage, Role};
use std::env;

/// Integration test for DeepSeek chat API
/// Requires DEEPSEEK_API_KEY environment variable
#[tokio::test]
#[ignore = "Requires real DeepSeek API key"]
async fn test_deepseek_chat() {
    // Get API key from environment variable
    let api_key = env::var("DEEPSEEK_API_KEY")
        .expect("DEEPSEEK_API_KEY environment variable must be set for this test");
    
    // Create provider
    let provider = create_provider("deepseek", "deepseek-chat", &api_key)
        .expect("Failed to create DeepSeek provider");
    
    // Create test request
    let request = ChatRequest {
        messages: vec![
            ChatMessage::new_text(Role::User, "Hello, can you help me with a simple question?")
        ],
        model: "deepseek-chat".to_string(),
        temperature: Some(0.7),
        top_p: Some(0.9),
        stream: Some(false),
        tools: None,
        tool_choice: None,
        max_tokens: Some(200),
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
    
    // Make API call
    let response = provider.chat(request).await
        .expect("DeepSeek chat API call failed");
    
    // Verify response
    assert!(!response.choices.is_empty(), "Response should contain at least one choice");
    println!("DeepSeek API call succeeded! Response: {:?}", response);
}

/// Integration test for OpenRouter chat API
/// Requires OPENROUTER_API_KEY environment variable
#[tokio::test]
#[ignore = "Requires real OpenRouter API key"]
async fn test_openrouter_chat() {
    // Get API key from environment variable
    let api_key = env::var("OPENROUTER_API_KEY")
        .expect("OPENROUTER_API_KEY environment variable must be set for this test");
    
    // Create provider
    let provider = create_provider("openrouter", "deepseek/deepseek-v3.2", &api_key)
        .expect("Failed to create OpenRouter provider");
    
    // Create test request
    let request = ChatRequest {
        messages: vec![
            ChatMessage::new_text(Role::User, "Hello, can you help me with a simple question?")
        ],
        model: "deepseek/deepseek-v3.2".to_string(),
        temperature: Some(0.7),
        top_p: Some(0.9),
        stream: Some(false),
        tools: None,
        tool_choice: None,
        max_tokens: Some(200),
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
    
    // Make API call
    let response = provider.chat(request).await
        .expect("OpenRouter chat API call failed");
    
    // Verify response
    assert!(!response.choices.is_empty(), "Response should contain at least one choice");
    println!("OpenRouter chat API call succeeded! Response: {:?}", response);
}

/// Integration test for OpenRouter embedding API
/// Requires OPENROUTER_API_KEY environment variable
#[tokio::test]
#[ignore = "Requires real OpenRouter API key"]
async fn test_openrouter_embedding() {
    // Get API key from environment variable
    let api_key = env::var("OPENROUTER_API_KEY")
        .expect("OPENROUTER_API_KEY environment variable must be set for this test");
    
    // Create provider
    let provider = create_provider("openrouter", "openai/text-embedding-3-large", &api_key)
        .expect("Failed to create OpenRouter provider");
    
    // Create embedding request
    let request = eye::provider::EmbeddingRequest {
        input: vec![
            "The quick brown fox jumps over the lazy dog".to_string(),
            "I love programming in Rust".to_string()
        ],
        model: "openai/text-embedding-3-large".to_string(),
        encoding_format: None,
        dimensions: None,
        user: None,
    };
    
    // Make API call
    let response = provider.embedding(request).await
        .expect("OpenRouter embedding API call failed");
    
    // Verify response
    assert!(!response.data.is_empty(), "Response should contain embedding data");
    assert!(response.data.len() == 2, "Should get 2 embeddings for 2 inputs");
    println!("OpenRouter embedding API call succeeded! Got {} embeddings", response.data.len());
}
