//! Example to test the Provider trait implementation

use anyhow::Result;
use eye::provider::{ChatMessage, ChatRequest, EmbeddingRequest, Provider, Role, create_provider};

#[tokio::main]
async fn main() -> Result<()> {
    println!("Testing Provider trait implementation...");

    // Test OpenAI provider
    println!("\n1. Testing OpenAI provider:");
    let openai_provider = create_provider("openai", "gpt-3.5-turbo", "test-key")?;
    test_provider(&openai_provider).await?;

    // Test OpenRouter provider
    println!("\n2. Testing OpenRouter provider:");
    let openrouter_provider = create_provider("openrouter", "openai/gpt-3.5-turbo", "test-key")?;
    test_provider(&openrouter_provider).await?;

    // Test DeepSeek provider
    println!("\n3. Testing DeepSeek provider:");
    let deepseek_provider = create_provider("deepseek", "deepseek-chat", "test-key")?;
    test_provider(&deepseek_provider).await?;

    println!("\nAll tests passed!");
    Ok(())
}

async fn test_provider(provider: &impl Provider) -> Result<()> {
    println!("  - Provider name: {}", provider.name());

    // Test capabilities
    let capabilities = provider.capabilities();
    println!("  - Capabilities: {:?}", capabilities);
    assert!(capabilities.contains(eye::provider::ModelCapabilities::TEXT_GENERATION));

    // Test max context length
    let context_length = provider.max_context_length();
    println!("  - Max context length: {}", context_length);
    assert!(context_length > 0);

    // Test chat (mock implementation)
    let request = ChatRequest {
        messages: vec![ChatMessage {
            role: Role::User,
            content: eye::provider::Content::Text("Hello, world!".to_string()),
            name: None,
            tool_calls: None,
            tool_call_id: None,
        }],
        model: provider.name().to_string(),
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
    println!("  - Chat response model: {}", response.model);
    println!("  - Number of choices: {}", response.choices.len());
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
    println!("  - Embedding response model: {}", embedding_response.model);
    println!(
        "  - Number of embeddings: {}",
        embedding_response.data.len()
    );
    assert!(!embedding_response.data.is_empty());

    Ok(())
}
