//! Comprehensive OpenAI chat_completion example
//!
//! This example demonstrates various OpenAI chat completion features:
//! 1. Basic conversation
//! 2. Multi-turn conversation
//! 3. Tool/function calling
//!
//! Requirements:
//! - Set OPENAI_API_KEY environment variable
//! - For OpenRouter, set OPENROUTER_API_KEY instead
//! - For DeepSeek, set DEEPSEEK_API_KEY instead

use chrono::Local;
use eye::provider::compatible::OpenaiCompatibleProvider;
use eye::provider::{Content, Provider, ChatRequest, ChatMessage, Role, Tool};
use eye::OptionToResult;
use serde::Deserialize;
use serde_json::json;
use std::env;

const MODEL: &str = "deepseek-chat";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("OpenAI Chat Completion Examples");
    println!("===============================\n");

    // Check if API key is available
    let has_api_key =
        env::var("DEEPSEEK_API_KEY").is_ok() || env::var("OPENROUTER_API_KEY").is_ok() || env::var("OPENAI_API_KEY").is_ok();

    if !has_api_key {
        println!("⚠️  No API key found. Running in demonstration mode.");
        println!("To run actual API calls, set one of these environment variables:");
        println!("  - OPENAI_API_KEY (for OpenAI API)");
        println!("  - OPENROUTER_API_KEY (for OpenRouter API)");
        println!("  - DEEPSEEK_API_KEY (for DeepSeek API)");
        println!();
    }

    // Example 1: Basic conversation
    println!("1. Basic Conversation Example");
    println!("-----------------------------");
    if has_api_key {
        basic_conversation().await?;
    } else {
        println!("[Code structure shown - would make API call with valid key]");
        println!("Message: \"What is the capital of France?\"");
        println!("Expected response: Information about Paris");
    }
    println!();

    // Example 2: Multi-turn conversation
    println!("2. Multi-turn Conversation Example");
    println!("----------------------------------");
    if has_api_key {
        multi_turn_conversation().await?;
    } else {
        println!("[Code structure shown - would make API call with valid key]");
        println!("Multi-turn conversation with travel assistant about Japan");
        println!("Expected response: Recommendations for Tokyo attractions");
    }
    println!();

    Ok(())
}

/// Example 1: Basic conversation
async fn basic_conversation() -> Result<(), Box<dyn std::error::Error>> {
    let client = create_client()?;

    let request = ChatRequest {
        messages: vec![
            ChatMessage {
                role: Role::User,
                content: Some(Content::Text("What is the capital of France?".to_string())),
                name: None,
                tool_calls: None,
                tool_call_id: None,
            }
        ],
        model: client.name().to_string(),
        temperature: Some(0.7),
        top_p: Some(1.0),
        stream: Some(false),
        tools: None,
        tool_choice: None,
        max_tokens: Some(4096),
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

    let result = client.chat(request).await?;

    if let Some(choice) = result.choices.first() {
        if let Some(content) = &choice.message.content {
            match content {
                Content::Text(text) => println!("Assistant: {}", text),
                Content::Parts(parts) => {
                    for part in parts {
                        println!("Assistant: {:?}", part);
                    }
                }
            }
        }
    }

    Ok(())
}

/// Example 2: Multi-turn conversation
async fn multi_turn_conversation() -> Result<(), Box<dyn std::error::Error>> {
    let client = create_client()?;

    // Simulate a conversation history
    let request = ChatRequest {
        messages: vec![
            ChatMessage {
                role: Role::System,
                content: Some(Content::Text("You are a travel assistant".to_string())),
                name: None,
                tool_calls: None,
                tool_call_id: None,
            },
            ChatMessage {
                role: Role::User,
                content: Some(Content::Text("I'm planning a trip to Japan.".to_string())),
                name: None,
                tool_calls: None,
                tool_call_id: None,
            },
            ChatMessage {
                role: Role::Assistant,
                content: Some(Content::Text("That's great! Japan is an amazing destination. What would you like to know about?".to_string())),
                name: None,
                tool_calls: None,
                tool_call_id: None,
            },
            ChatMessage {
                role: Role::User,
                content: Some(Content::Text("What are the must-visit temples in Tokyo?".to_string())),
                name: None,
                tool_calls: None,
                tool_call_id: None,
            },
        ],
        model: client.name().to_string(),
        temperature: Some(0.7),
        top_p: Some(1.0),
        stream: Some(false),
        tools: None,
        tool_choice: None,
        max_tokens: Some(4096),
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

    let result = client.chat(request).await?;

    if let Some(choice) = result.choices.first() {
        if let Some(content) = &choice.message.content {
            match content {
                Content::Text(text) => println!("Assistant: {}", text),
                Content::Parts(parts) => {
                    for part in parts {
                        println!("Assistant: {:?}", part);
                    }
                }
            }
        }
    }

    Ok(())
}

#[derive(Debug, Deserialize)]
pub struct MarkCacheableParam {
    pub cacheable: bool,
}

/// Create OpenAI client based on available API keys
fn create_client() -> anyhow::Result<OpenaiCompatibleProvider> {
    println!("DEEPSEEK_API_KEY: {:?}", env::var("DEEPSEEK_API_KEY"));
    
    // Try DeepSeek first
    if let Ok(api_key) = env::var("DEEPSEEK_API_KEY") {
        println!("Using DeepSeek API");
        let client = OpenaiCompatibleProvider::new(
            "DeepSeek".to_string(),
            MODEL.to_string(),
            "https://api.deepseek.com".to_string(),
            api_key,
            4096,
        );
        Ok(client)
    // Then try OpenRouter
    } else if let Ok(api_key) = env::var("OPENROUTER_API_KEY") {
        println!("Using OpenRouter API");
        let client = OpenaiCompatibleProvider::new(
            "OpenRouter".to_string(),
            MODEL.to_string(),
            "https://openrouter.ai/api/v1".to_string(),
            api_key,
            4096,
        );
        Ok(client)
    // Then try OpenAI
    } else if let Ok(api_key) = env::var("OPENAI_API_KEY") {
        println!("Using OpenAI API");
        let client = OpenaiCompatibleProvider::new(
            "OpenAI".to_string(),
            MODEL.to_string(),
            "https://api.openai.com/v1".to_string(),
            api_key,
            4096,
        );
        Ok(client)
    } else {
        anyhow::bail!(
            "API key not found. Please set one of these environment variables:\n\
             - OPENAI_API_KEY (for OpenAI API)\n\
             - OPENROUTER_API_KEY (for OpenRouter API)\n\
             - DEEPSEEK_API_KEY (for DeepSeek API)"
        )
    }
}
