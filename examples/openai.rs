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
use openai_api_rs::v1::api::OpenAIClient;
use openai_api_rs::v1::chat_completion::chat_completion::ChatCompletionRequest;
use openai_api_rs::v1::chat_completion::chat_completion_stream::{
    ChatCompletionStreamRequest, ChatCompletionStreamResponse,
};
use openai_api_rs::v1::chat_completion::{
    ChatCompletionMessage, Content, MessageRole, Tool, ToolType,
};
use openai_api_rs::v1::common::GPT4_O;
use openai_api_rs::v1::types::{Function, FunctionParameters, JSONSchemaDefine, JSONSchemaType};
use serde_json::Value;
use std::collections::HashMap;
use std::env;
use tokio_stream::StreamExt;

const MODEL: &str = "deepseek/deepseek-v3.2";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("OpenAI Chat Completion Examples");
    println!("===============================\n");

    // Check if API key is available
    let has_api_key = env::var("OPENAI_API_KEY").is_ok() || env::var("OPENROUTER_API_KEY").is_ok();

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
        //basic_conversation().await?;
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
        //multi_turn_conversation().await?;
    } else {
        println!("[Code structure shown - would make API call with valid key]");
        println!("Multi-turn conversation with travel assistant about Japan");
        println!("Expected response: Recommendations for Tokyo attractions");
    }
    println!();

    // Example 3: Streaming response
    println!("3. Streaming Response Example");
    println!("-----------------------------");
    if has_api_key {
        //streaming_response().await?;
    } else {
        println!("[Code structure shown - would make API call with valid key]");
        println!("Streaming response for poem about Rust programming");
        println!("Expected response: Poem displayed word by word");
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

    // Example 5: JSON schema output
    println!("5. JSON Schema Output Example");
    println!("-----------------------------");
    if has_api_key {
        //json_schema_output().await?;
    } else {
        println!("[Code structure shown - would make API call with valid key]");
        println!("JSON schema output for book information");
        println!("Expected response: Structured JSON about book '1984'");
    }

    Ok(())
}

/// Example 1: Basic conversation
async fn basic_conversation() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = create_client()?;

    let messages = vec![ChatCompletionMessage {
        role: openai_api_rs::v1::chat_completion::MessageRole::user,
        content: openai_api_rs::v1::chat_completion::Content::Text(
            "What is the capital of France?".to_string(),
        ),
        name: None,
        tool_calls: None,
        tool_call_id: None,
    }];

    let request = ChatCompletionRequest::new(MODEL.to_string(), messages);

    let result = client.chat_completion(request).await?;

    if let Some(choice) = result.choices.first() {
        if let Some(content) = &choice.message.content {
            println!("Assistant: {}", content);
        }
    }

    Ok(())
}

/// Example 2: Multi-turn conversation
async fn multi_turn_conversation() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = create_client()?;

    // Simulate a conversation history
    let messages = vec![
        ChatCompletionMessage {
            role: openai_api_rs::v1::chat_completion::MessageRole::system,
            content: openai_api_rs::v1::chat_completion::Content::Text(
                "You are a travel assistant.".to_string(),
            ),
            name: None,
            tool_calls: None,
            tool_call_id: None,
        },
        ChatCompletionMessage {
            role: openai_api_rs::v1::chat_completion::MessageRole::user,
            content: openai_api_rs::v1::chat_completion::Content::Text(
                "I'm planning a trip to Japan.".to_string(),
            ),
            name: None,
            tool_calls: None,
            tool_call_id: None,
        },
        ChatCompletionMessage {
            role: openai_api_rs::v1::chat_completion::MessageRole::assistant,
            content: openai_api_rs::v1::chat_completion::Content::Text(
                "That's great! Japan is an amazing destination. What would you like to know about?"
                    .to_string(),
            ),
            name: None,
            tool_calls: None,
            tool_call_id: None,
        },
        ChatCompletionMessage {
            role: openai_api_rs::v1::chat_completion::MessageRole::user,
            content: openai_api_rs::v1::chat_completion::Content::Text(
                "What are the must-visit places in Tokyo?".to_string(),
            ),
            name: None,
            tool_calls: None,
            tool_call_id: None,
        },
    ];

    let request = ChatCompletionRequest::new(MODEL.to_string(), messages);

    let result = client.chat_completion(request).await?;

    if let Some(choice) = result.choices.first() {
        if let Some(content) = &choice.message.content {
            println!("Assistant: {}", content);
        }
    }

    Ok(())
}

/// Example 3: Streaming response
async fn streaming_response() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = create_client()?;

    let messages = vec![
        ChatCompletionMessage {
            role: openai_api_rs::v1::chat_completion::MessageRole::system,
            content: openai_api_rs::v1::chat_completion::Content::Text(
                "You are a poet.".to_string(),
            ),
            name: None,
            tool_calls: None,
            tool_call_id: None,
        },
        ChatCompletionMessage {
            role: openai_api_rs::v1::chat_completion::MessageRole::user,
            content: openai_api_rs::v1::chat_completion::Content::Text(
                "Write a short poem about Rust programming.".to_string(),
            ),
            name: None,
            tool_calls: None,
            tool_call_id: None,
        },
    ];

    let request = ChatCompletionStreamRequest::new(MODEL.to_string(), messages);

    println!("Streaming response (word by word):");
    print!("Assistant: ");

    let mut stream = client.chat_completion_stream(request).await?;

    while let Some(response) = stream.next().await {
        // The streaming response has a different structure
        // Print the entire response for demonstration
        match response {
            ChatCompletionStreamResponse::Content(c) => {
                print!("{}", c);
            }
            ChatCompletionStreamResponse::ToolCall(tc) => {
                println!("{:?}", tc);
            }
            ChatCompletionStreamResponse::Done => {}
        }
    }

    println!();

    Ok(())
}

/// Example 4: Tool/function calling
async fn tool_calling() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = create_client()?;

    let tool1 = Tool {
        r#type: ToolType::Function,
        function: Function {
            name: "datetime".into(),
            description: Some("get current datetime".into()),
            parameters: FunctionParameters {
                schema_type: JSONSchemaType::Object,
                properties: Some(HashMap::new()),
                required: None,
            },
        },
    };
    let tool2 = Tool {
            r#type: ToolType::Function,
            function: Function {
                name: "mark_cacheable".into(),
                description: Some("Indicate whether the current request/response is eligible for response caching. This tool is called for every request. If the request can be safely cached (no context dependence, no realtime data, deterministic answer), cacheable is true; otherwise false.".into()),
                parameters: FunctionParameters {
                    schema_type: JSONSchemaType::Object,
                    properties: Some(HashMap::from([("cacheable".to_string(), Box::new(JSONSchemaDefine{
                        schema_type: Some(JSONSchemaType::Boolean),
                        description: Some("Whether the current request is eligible for response caching.".to_string()),
                        enum_values: None,
                        properties: None,
                        required: None,
                        items: None,
                    }))])),
                    required: Some(vec!["cacheable".to_string()]),
                },
            },
        };

    let mut messages = vec![
        ChatCompletionMessage {
            role: openai_api_rs::v1::chat_completion::MessageRole::system,
            content: openai_api_rs::v1::chat_completion::Content::Text(
                "You are a helpful assistant with access to tools. ".to_string(),
            ),
            name: None,
            tool_calls: None,
            tool_call_id: None,
        },
        ChatCompletionMessage {
            role: openai_api_rs::v1::chat_completion::MessageRole::user,
            content: openai_api_rs::v1::chat_completion::Content::Text(
                "What time is it now? What is the capital of France?".to_string(),
            ),
            name: None,
            tool_calls: None,
            tool_call_id: None,
        },
    ];

    for i in 0..10 {
        // Note: Tool calling requires specific provider support (GPT-4 with function calling)
        // For this example, we make a regular request and show how tool calls would be handled
        let mut request = ChatCompletionRequest::new(GPT4_O.to_string(), messages.clone());
        if i == 0 {
            request.tools = Some(vec![tool1.clone(), tool2.clone()]);
        } else {
            request.tools = Some(vec![tool1.clone()]);
        }

        let result = client.chat_completion(request).await?;

        let choice = &result.choices[0];
        println!("Finish reason: {:?}", choice.finish_reason);

        messages.push(ChatCompletionMessage {
            role: choice.message.role.clone(),
            content: Content::Text(choice.message.content.clone().unwrap_or_default()),
            name: choice.message.name.clone(),
            tool_calls: choice.message.tool_calls.clone().map(|mut s| {
                s.retain(|t| t.function.name != Some("mark_cacheable".into()));
                s
            }),
            tool_call_id: None,
        });

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
                match tool_call.function.name.as_ref().map(|s| s.as_str()) {
                    Some("mark_cacheable") => {
                        println!("  Mark cacheable");
                    }
                    Some("datetime") => {
                        messages.push(ChatCompletionMessage {
                            role: MessageRole::tool,
                            content: Content::Text(Local::now().to_string()),
                            name: None,
                            tool_calls: None,
                            tool_call_id: Some(tool_call.id.clone()),
                        });
                    }
                    _ => {}
                }
            }

            println!("\nNote: In a full implementation, you would send the tool results");
            println!("back to the provider to generate a natural language response.");
        } else if let Some(content) = &choice.message.content {
            println!("Assistant: {}", content);
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

/// Example 5: JSON schema output
async fn json_schema_output() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = create_client()?;

    let messages = vec![
        ChatCompletionMessage {
            role: openai_api_rs::v1::chat_completion::MessageRole::system,
            content: openai_api_rs::v1::chat_completion::Content::Text("You are a book database assistant. Always respond with valid JSON.".to_string()),
            name: None,
            tool_calls: None,
            tool_call_id: None,
        },
        ChatCompletionMessage {
            role: openai_api_rs::v1::chat_completion::MessageRole::user,
            content: openai_api_rs::v1::chat_completion::Content::Text("Tell me about the book '1984' by George Orwell. Return the response as JSON with fields: title, author, year, genres (array), summary.".to_string()),
            name: None,
            tool_calls: None,
            tool_call_id: None,
        },
    ];

    let request = ChatCompletionRequest::new(MODEL.to_string(), messages);

    let result = client.chat_completion(request).await?;

    if let Some(choice) = result.choices.first() {
        if let Some(content) = &choice.message.content {
            println!("JSON Response:");

            // Try to parse and pretty-print the JSON
            match serde_json::from_str::<Value>(content) {
                Ok(parsed_json) => {
                    println!("{}", serde_json::to_string_pretty(&parsed_json)?);
                }
                Err(e) => {
                    println!("Raw response (not valid JSON): {}", content);
                    println!("JSON parse error: {}", e);
                }
            }
        }
    }

    Ok(())
}

/// Create OpenAI client based on available API keys
fn create_client() -> Result<OpenAIClient, Box<dyn std::error::Error>> {
    // Try OpenRouter first, then OpenAI
    if let Ok(api_key) = env::var("OPENROUTER_API_KEY") {
        println!("Using OpenRouter API");
        let client = OpenAIClient::builder()
            .with_endpoint("https://openrouter.ai/api/v1")
            .with_api_key(api_key)
            .build()?;
        Ok(client)
    } else if let Ok(api_key) = env::var("OPENAI_API_KEY") {
        println!("Using OpenAI API");
        let client = OpenAIClient::builder().with_api_key(api_key).build()?;
        Ok(client)
    } else {
        Err("API key not found. Please set OPENAI_API_KEY or OPENROUTER_API_KEY environment variable.".into())
    }
}
