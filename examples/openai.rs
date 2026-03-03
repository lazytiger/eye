//! Comprehensive OpenAI chat_completion example
//!
//! This example demonstrates various OpenAI chat completion features:
//! 1. Basic conversation
//! 2. Multi-turn conversation
//! 3. Streaming responses
//! 4. Tool/function calling
//! 5. JSON schema output
//!
//! Requirements:
//! - Set OPENAI_API_KEY environment variable
//! - For OpenRouter, set OPENROUTER_API_KEY instead

use chrono::Local;
use eye::provider::openai::OpenaiCompatibleProvider;
use eye::provider::{Content, Provider, Request, Tool};
use eye::OptionToResult;
use serde::Deserialize;
use serde_json::json;
use std::env;
use std::sync::Arc;
use tokio_stream::StreamExt;

const MODEL: &str = "deepseek-chat";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("OpenAI Chat Completion Examples");
    println!("===============================\n");

    // Check if API key is available
    let has_api_key =
        env::var("DEEPSEEK_API_KEY").is_ok() || env::var("OPENROUTER_API_KEY").is_ok();

    if !has_api_key {
        println!("⚠️  No API key found. Running in demonstration mode.");
        println!("To run actual API calls, set one of these environment variables:");
        println!("  - OPENAI_API_KEY (for OpenAI API)");
        println!("  - OPENROUTER_API_KEY (for OpenRouter API)");
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

    // Example 4: Tool/function calling
    println!("4. Tool/Function Calling Example");
    println!("--------------------------------");
    if has_api_key {
        tool_calling().await?;
    } else {
        println!("[Code structure shown - would make API call with valid key]");
        println!("Tool calling for weather information");
        println!("Expected response: Model requests to call get_weather tool");
    }
    println!();

    Ok(())
}

/// Example 1: Basic conversation
async fn basic_conversation() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = create_client()?;

    let mut request = Request::new();
    request.add_user_message("What is the capital of France?".to_string());

    let result = client.chat(request).await?;

    if let Some(choice) = result.choices.first() {
        let Content(content) = &choice.message.content;
        println!("Assistant: {:?}", content);
    }

    Ok(())
}

/// Example 2: Multi-turn conversation
async fn multi_turn_conversation() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = create_client()?;

    // Simulate a conversation history
    let mut request = Request::new();
    request
        .add_system_message("You are a travel assistant")
        .add_user_message("I'm planning a trip to Japan.")
        .add_assistant_message(
            "That's great! Japan is an amazing destination. What would you like to know about?",
        )
        .add_user_message("What are the must-visit temples in Tokyo?");

    let result = client.chat(request).await?;

    if let Some(choice) = result.choices.first() {
        let Content(content) = &choice.message.content;
        println!("Assistant: {:?}", content);
    }

    Ok(())
}

#[derive(Debug, Deserialize)]
pub struct MarkCacheableParam {
    pub cacheable: bool,
}

/// Example 4: Tool/function calling
async fn tool_calling() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = create_client()?;

    let tool1 = json!({
        "type": "function",
        "function":{
            "name": "datetime",
            "description": "get current datetime",
            "parameters": {
                "type": "object",
                "properties": {},
            }
        }
    });
    let tool1: Tool = serde_json::from_value(tool1)?;

    let tool2 = json!({
        "type": "function",
        "function":{
            "name": "mark_cacheable",
            "description": "Indicate whether the current request/response is eligible for response caching. This tool is called for every request. If the request can be safely cached (no context dependence, no realtime data, deterministic answer), cacheable is true; otherwise false.",
            "parameters": {
                "type": "object",
                "properties": {
                    "cacheable": {
                        "type": "boolean",
                        "description": "Whether the current request is eligible for response caching."
                    }
                },
                "required": ["cacheable"]
            }
        }
    });
    let tool2: Tool = serde_json::from_value(tool2)?;

    let mut request = Request::new();
    request
        .add_system_message("You are a helpful assistant with access to tools.")
        .add_user_message("What time is it now? What is the capital of France?");
    for i in 0..10 {
        // Note: Tool calling requires specific provider support (GPT-4 with function calling)
        // For this example, we make a regular request and show how tool calls would be handled
        if i == 0 {
            request.tools = Some(vec![tool1.clone(), tool2.clone()]);
            request.tool_choice_function("mark_cacheable");
        } else {
            request.tools = Some(vec![tool1.clone()]);
            request.tool_choice = None;
        }
        request.max_completion_tokens = Some(4096);

        let result = client.chat(request.clone()).await?;

        let choice = &result.choices[0];
        println!("Finish reason: {:?}", choice.finish_reason);

        request.messages.push(choice.message.clone().into());

        // Check if the provider wants to call a tool
        if let Some(tool_calls) = &choice.message.tool_calls {
            println!("\n=== Tool Calls ===");
            println!("Model wants to call {} tool(s):", tool_calls.len());

            for (i, tool_call) in tool_calls.iter().enumerate() {
                println!("\nTool call {}:", i + 1);
                println!("  Raw tool call: {:?}", tool_call);

                // In a real application, you would:
                // 1. Parse the tool_call to get function name and arguments
                // 2. Execute the actual function
                // 3. Send the result back to the provider for a final response
                match tool_call.function.name.as_str() {
                    "mark_cacheable" => {
                        let args = tool_call.function.arguments.clone().to_ok()?;
                        let args: MarkCacheableParam = serde_json::from_str(&args)?;
                        println!("  Mark cacheable:{}", args.cacheable);
                        request.add_tool_message(&tool_call.id, "");
                    }
                    "datetime" => {
                        request.add_tool_message(&tool_call.id, Local::now().to_string());
                    }
                    _ => {}
                }
            }

            println!("\nNote: In a full implementation, you would send the tool results");
            println!("back to the provider to generate a natural language response.");
        }
        {
            let Content(content) = &choice.message.content;
            println!("Assistant: {:?}", content);
            println!("\nNote: The provider did not request a tool call.");
            println!("This may happen if the provider doesn't support function calling");
            println!("or if it's not configured to use tools.");
        }

        if choice
            .message
            .tool_calls
            .as_ref()
            .map(|tc| tc.len())
            .unwrap_or(0)
            == 0
        {
            break;
        }
    }

    Ok(())
}

/// Create OpenAI client based on available API keys
fn create_client() -> anyhow::Result<OpenaiCompatibleProvider> {
    println!("DEEPSEEK_API_KEY: {:?}", env::var("DEEPSEEK_API_KEY"));
    // Try OpenRouter first, then OpenAI
    let client = Arc::new(reqwest::Client::new());
    if let Ok(api_key) = env::var("OPENROUTER_API_KEY") {
        println!("Using OpenRouter API");
        let client = OpenaiCompatibleProvider::new(
            "OpenRouter".to_string(),
            MODEL.to_string(),
            "https://openrouter.ai/api/v1".to_string(),
            api_key,
            client,
        );
        Ok(client)
    } else if let Ok(api_key) = env::var("DEEPSEEK_API_KEY") {
        let client = OpenaiCompatibleProvider::new(
            "OpenRouter".to_string(),
            MODEL.to_string(),
            "https://api.deepseek.com".to_string(),
            api_key,
            client,
        );
        Ok(client)
    } else {
        anyhow::bail!(
            "API key not found. Please set OPENAI_API_KEY or OPENROUTER_API_KEY environment variable."
        )
    }
}
