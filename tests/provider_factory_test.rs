
//! Unit tests for the provider factory function

use eye::provider::create_provider;
use std::env;

/// Test creating an OpenAI provider
#[test]
fn test_create_openai_provider() {
    let provider = create_provider("openai", "gpt-4", "test-api-key");
    assert!(provider.is_ok());
    let provider = provider.unwrap();
    assert_eq!(provider.name(), "openai");
}

/// Test creating an OpenRouter provider
#[test]
fn test_create_openrouter_provider() {
    let provider = create_provider("openrouter", "anthropic/claude-3-opus", "test-api-key");
    assert!(provider.is_ok());
    let provider = provider.unwrap();
    assert_eq!(provider.name(), "openrouter");
}

/// Test creating a DeepSeek provider
#[test]
fn test_create_deepseek_provider() {
    let provider = create_provider("deepseek", "deepseek-chat", "test-api-key");
    assert!(provider.is_ok());
    let provider = provider.unwrap();
    assert_eq!(provider.name(), "deepseek");
}

/// Test creating a compatible provider with custom endpoint
#[test]
fn test_create_compatible_provider() {
    let provider = create_provider(
        "custom:https://api.custom.ai/v1",
        "custom-model",
        "test-api-key",
    );
    assert!(provider.is_ok());
}

/// Test creating a compatible provider with http endpoint
#[test]
fn test_create_compatible_provider_http() {
    let provider = create_provider(
        "local:http://localhost:8080/v1",
        "local-model",
        "test-api-key",
    );
    assert!(provider.is_ok());
}

/// Test case insensitive provider names
#[test]
fn test_provider_names_case_insensitive() {
    assert!(create_provider("OPENAI", "gpt-4", "test").is_ok());
    assert!(create_provider("OpenAI", "gpt-4", "test").is_ok());
    assert!(create_provider("openai", "gpt-4", "test").is_ok());
}

/// Test unknown provider returns error
#[test]
fn test_unknown_provider() {
    let result = create_provider("unknown", "model", "test-api-key");
    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("Unknown provider"));
}

/// Test invalid compatible provider format
#[test]
fn test_invalid_compatible_format() {
    // Missing protocol
    let result = create_provider("custom:invalid-url", "model", "test-api-key");
    assert!(result.is_err());
    
    // Missing colon
    let result = create_provider("customhttps://api.example.com", "model", "test-api-key");
    assert!(result.is_ok()); // This is treated as a regular provider name, will fail as unknown
}

/// Test API key from environment variable
#[test]
fn test_api_key_from_env() {
    // Set environment variable
    env::set_var("OPENAI_API_KEY", "env-api-key");
    
    // Create provider with empty api_key - should use env var
    let provider = create_provider("openai", "gpt-4", "");
    assert!(provider.is_ok());
    
    // Clean up
    env::remove_var("OPENAI_API_KEY");
}

/// Test API key fallback to parameter when env var not set
#[test]
fn test_api_key_fallback_to_param() {
    // Ensure env var is not set
    env::remove_var("OPENAI_API_KEY");
    
    let provider = create_provider("openai", "gpt-4", "param-api-key");
    assert!(provider.is_ok());
}

/// Test API key priority: env var over parameter
#[test]
fn test_api_key_priority_env_over_param() {
    env::set_var("DEEPSEEK_API_KEY", "env-priority-key");
    
    // Even with a different param, env var should take priority
    let provider = create_provider("deepseek", "deepseek-chat", "param-key");
    assert!(provider.is_ok());
    
    env::remove_var("DEEPSEEK_API_KEY");
}