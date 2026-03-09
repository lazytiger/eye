// Integration tests for provider implementations
// These tests require real API keys and will make actual API calls

use eye::provider::{create_provider, ChatRequest, ChatMessage, MessageContent, ContentPart, EmbeddingRequest, EmbeddingInput};
use eye::provider::types::chat::{ImageUrl, ImageDetail};
use std::env;

// ============================================================================
// OpenRouter Tests
// ============================================================================

/// Integration test for OpenRouter chat API
#[tokio::test]
#[ignore = "Requires real OpenRouter API key"]
async fn test_openrouter_chat() {
    let api_key = env::var("OPENROUTER_API_KEY")
        .expect("OPENROUTER_API_KEY environment variable must be set for this test");

    let provider = create_provider("openrouter", "deepseek/deepseek-chat", &api_key)
        .expect("Failed to create OpenRouter provider");

    let request = ChatRequest {
        messages: vec![
            ChatMessage::User(eye::provider::UserMessage {
                content: MessageContent::Text("Hello, can you help me with a simple question?".to_string()),
                name: None,
            })
        ],
        model: Some("deepseek/deepseek-chat".to_string()),
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

/// Integration test for OpenRouter with multimodal content (image)
#[tokio::test]
#[ignore = "Requires real OpenRouter API key"]
async fn test_openrouter_multimodal_image() {
    let api_key = env::var("OPENROUTER_API_KEY")
        .expect("OPENROUTER_API_KEY environment variable must be set for this test");

    let provider = create_provider("openrouter", "openai/gpt-4o-mini", &api_key)
        .expect("Failed to create OpenRouter provider");

    let request = ChatRequest {
        messages: vec![
            ChatMessage::User(eye::provider::UserMessage {
                content: MessageContent::Parts(vec![
                    ContentPart::Text { text: "What's in this image?".to_string() },
                    ContentPart::ImageUrl {
                        image_url: ImageUrl {
                            url: "https://upload.wikimedia.org/wikipedia/commons/thumb/d/dd/Gfp-wisconsin-madison-the-nature-boardwalk.jpg/2560px-Gfp-wisconsin-madison-the-nature-boardwalk.jpg".to_string(),
                            detail: Some(ImageDetail::Auto),
                        }
                    },
                ]),
                name: None,
            })
        ],
        model: Some("openai/gpt-4o-mini".to_string()),
        temperature: Some(0.7),
        stream: Some(false),
        max_tokens: Some(500),
        ..Default::default()
    };

    let response = provider.chat(request).await
        .expect("OpenRouter multimodal API call failed");

    assert!(!response.choices.is_empty(), "Response should contain at least one choice");
    println!("OpenRouter multimodal API call succeeded!");
}

// ============================================================================
// DeepSeek Tests
// ============================================================================

/// Integration test for DeepSeek chat API
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
    println!("DeepSeek chat API call succeeded! Response: {:?}", response);
}

/// Integration test for DeepSeek with reasoning model
#[tokio::test]
#[ignore = "Requires real DeepSeek API key"]
async fn test_deepseek_reasoner() {
    let api_key = env::var("DEEPSEEK_API_KEY")
        .expect("DEEPSEEK_API_KEY environment variable must be set for this test");

    let provider = create_provider("deepseek", "deepseek-reasoner", &api_key)
        .expect("Failed to create DeepSeek provider");

    let request = ChatRequest {
        messages: vec![
            ChatMessage::User(eye::provider::UserMessage {
                content: MessageContent::Text("If a train travels 120 km in 2 hours, what is its average speed?".to_string()),
                name: None,
            })
        ],
        model: Some("deepseek-reasoner".to_string()),
        temperature: Some(0.6),
        stream: Some(false),
        max_tokens: Some(1000),
        ..Default::default()
    };

    let response = provider.chat(request).await
        .expect("DeepSeek reasoner API call failed");

    assert!(!response.choices.is_empty(), "Response should contain at least one choice");
    println!("DeepSeek reasoner API call succeeded!");
}

// ============================================================================
// Compatible Provider Tests
// ============================================================================

/// Integration test for Compatible provider with OpenRouter endpoint
#[tokio::test]
#[ignore = "Requires real OpenRouter API key"]
async fn test_compatible_with_openrouter() {
    let api_key = env::var("OPENROUTER_API_KEY")
        .expect("OPENROUTER_API_KEY environment variable must be set for this test");

    let provider = create_provider(
        "custom:https://openrouter.ai/api/v1",
        "deepseek/deepseek-chat",
        &api_key,
    ).expect("Failed to create compatible provider");

    let request = ChatRequest {
        messages: vec![
            ChatMessage::User(eye::provider::UserMessage {
                content: MessageContent::Text("Hello from compatible provider!".to_string()),
                name: None,
            })
        ],
        model: Some("deepseek/deepseek-chat".to_string()),
        temperature: Some(0.7),
        stream: Some(false),
        max_tokens: Some(200),
        ..Default::default()
    };

    let response = provider.chat(request).await
        .expect("Compatible provider chat API call failed");

    assert!(!response.choices.is_empty(), "Response should contain at least one choice");
    println!("Compatible provider (OpenRouter) API call succeeded!");
}

/// Integration test for Compatible provider with DeepSeek endpoint
#[tokio::test]
#[ignore = "Requires real DeepSeek API key"]
async fn test_compatible_with_deepseek() {
    let api_key = env::var("DEEPSEEK_API_KEY")
        .expect("DEEPSEEK_API_KEY environment variable must be set for this test");

    let provider = create_provider(
        "custom:https://api.deepseek.com/v1",
        "deepseek-chat",
        &api_key,
    ).expect("Failed to create compatible provider");

    let request = ChatRequest {
        messages: vec![
            ChatMessage::User(eye::provider::UserMessage {
                content: MessageContent::Text("Hello from compatible provider!".to_string()),
                name: None,
            })
        ],
        model: Some("deepseek-chat".to_string()),
        temperature: Some(0.7),
        stream: Some(false),
        max_tokens: Some(200),
        ..Default::default()
    };

    let response = provider.chat(request).await
        .expect("Compatible provider chat API call failed");

    assert!(!response.choices.is_empty(), "Response should contain at least one choice");
    println!("Compatible provider (DeepSeek) API call succeeded!");
}

/// Integration test for Compatible provider embedding with OpenRouter endpoint
#[tokio::test]
#[ignore = "Requires real OpenRouter API key"]
async fn test_compatible_embedding_openrouter() {
    let api_key = env::var("OPENROUTER_API_KEY")
        .expect("OPENROUTER_API_KEY environment variable must be set for this test");

    let provider = create_provider(
        "custom:https://openrouter.ai/api/v1",
        "openai/text-embedding-3-large",
        &api_key,
    ).expect("Failed to create compatible provider");

    let request = EmbeddingRequest {
        input: EmbeddingInput::StringArray(vec![
            "Testing compatible provider embedding".to_string(),
        ]),
        model: "openai/text-embedding-3-large".to_string(),
        encoding_format: None,
        dimensions: None,
        user: None,
    };

    let response = provider.embedding(request).await
        .expect("Compatible provider embedding API call failed");

    assert!(!response.data.is_empty(), "Response should contain embedding data");
    println!("Compatible provider (OpenRouter) embedding API call succeeded!");
}

// ============================================================================
// Error Handling Tests (no API key required)
// ============================================================================

/// Test that unsupported content types return proper errors for DeepSeek
#[test]
fn test_deepseek_unsupported_content_error() {
    use eye::provider::types::chat::{ChatRequest as TypesChatRequest, ChatMessage as TypesChatMessage, MessageContent as TypesMessageContent, UserMessage};
    use eye::provider::deepseek::types::ChatRequest;
    use std::convert::TryFrom;

    // Create a request with image content that DeepSeek doesn't support
    let types_request = TypesChatRequest {
        messages: vec![
            TypesChatMessage::User(UserMessage {
                content: TypesMessageContent::Parts(vec![
                    ContentPart::Text { text: "What's in this image?".to_string() },
                    ContentPart::ImageUrl {
                        image_url: ImageUrl {
                            url: "https://example.com/image.jpg".to_string(),
                            detail: None,
                        }
                    },
                ]),
                name: None,
            })
        ],
        model: Some("deepseek-chat".to_string()),
        ..Default::default()
    };

    // Conversion should fail with an error
    let result = ChatRequest::try_from(types_request);
    assert!(result.is_err(), "DeepSeek should reject image input");
    let err_msg = format!("{:?}", result.err().unwrap());
    assert!(err_msg.contains("image"), "Error should mention image");
}

/// Test that unsupported content types return proper errors for compatible provider
#[test]
fn test_compatible_unsupported_video_error() {
    use eye::provider::types::chat::{ChatRequest as TypesChatRequest, ChatMessage as TypesChatMessage, MessageContent as TypesMessageContent, UserMessage, InputVideo, VideoFormat};
    use eye::provider::compatible::types::ChatCompletionRequest;
    use std::convert::TryFrom;

    // Create a request with video content that compatible provider doesn't support
    let types_request = TypesChatRequest {
        messages: vec![
            TypesChatMessage::User(UserMessage {
                content: TypesMessageContent::Parts(vec![
                    ContentPart::Text { text: "Analyze this video".to_string() },
                    ContentPart::InputVideo {
                        input_video: InputVideo {
                            data: "https://example.com/video.mp4".to_string(),
                            format: VideoFormat::Mp4,
                        }
                    },
                ]),
                name: None,
            })
        ],
        model: Some("test-model".to_string()),
        ..Default::default()
    };

    // Conversion should fail with an error
    let result = ChatCompletionRequest::try_from(types_request);
    assert!(result.is_err(), "Compatible provider should reject video input");
    let err_msg = format!("{:?}", result.err().unwrap());
    assert!(err_msg.contains("video"), "Error should mention video");
}

/// Test that unsupported content types return proper errors for OpenAI provider
#[test]
fn test_openai_unsupported_content_error() {
    use eye::provider::types::chat::{ChatRequest as TypesChatRequest, ChatMessage as TypesChatMessage, MessageContent as TypesMessageContent, UserMessage, InputVideo, VideoFormat};
    use eye::provider::openai::types::CreateChatCompletionRequest;
    use std::convert::TryFrom;

    // Create a request with video content that OpenAI doesn't support
    let types_request = TypesChatRequest {
        messages: vec![
            TypesChatMessage::User(UserMessage {
                content: TypesMessageContent::Parts(vec![
                    ContentPart::Text { text: "Analyze this".to_string() },
                    ContentPart::InputVideo {
                        input_video: InputVideo {
                            data: "https://example.com/video.mp4".to_string(),
                            format: VideoFormat::Mp4,
                        }
                    },
                ]),
                name: None,
            })
        ],
        model: Some("gpt-4".to_string()),
        ..Default::default()
    };

    // Conversion should fail with an error
    let result = CreateChatCompletionRequest::try_from(types_request);
    assert!(result.is_err(), "OpenAI provider should reject video input");
    let err_msg = format!("{:?}", result.err().unwrap());
    assert!(err_msg.contains("video"), "Error should mention video");
}
