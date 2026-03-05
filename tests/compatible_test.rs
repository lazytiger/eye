//! OpenAI-compatible API integration tests
//!
//! These tests verify the OpenAI-compatible API request and response structs
//! by testing serialization and deserialization with example data.

use eye::provider::compatible::*;
use serde_json::json;

#[test]
fn test_chat_completion_request_serialization() {
    let request = ChatCompletionRequest {
        messages: vec![
            ChatMessage {
                role: Role::System,
                content: Some("You are a helpful assistant.".to_string()),
                name: None,
                tool_calls: None,
                tool_call_id: None,
            },
            ChatMessage {
                role: Role::User,
                content: Some("Hello, how are you?".to_string()),
                name: None,
                tool_calls: None,
                tool_call_id: None,
            },
        ],
        model: "gpt-4o-mini".to_string(),
        temperature: Some(0.7),
        top_p: Some(0.9),
        stream: Some(false),
        tools: None,
        tool_choice: None,
        max_tokens: Some(100),
        n: Some(1),
        stop: Some(Stop::Single("\n".to_string())),
        frequency_penalty: Some(0.0),
        presence_penalty: Some(0.0),
        logit_bias: None,
        logprobs: Some(false),
        top_logprobs: None,
        seed: Some(42),
        user: Some("test-user".to_string()),
        response_format: Some(ResponseFormat::Text),
        parallel_tool_calls: Some(true),
        stream_options: Some(StreamOptions {
            include_usage: Some(true),
        }),
    };

    let serialized = serde_json::to_string(&request).unwrap();
    let deserialized: ChatCompletionRequest = serde_json::from_str(&serialized).unwrap();

    assert_eq!(deserialized.model, "gpt-4o-mini");
    assert_eq!(deserialized.messages.len(), 2);
    assert_eq!(deserialized.temperature, Some(0.7));
    assert_eq!(deserialized.seed, Some(42));
}

#[test]
fn test_chat_completion_response_deserialization() {
    let response_json = json!({
        "id": "chatcmpl-123",
        "object": "chat.completion",
        "created": 1677652288,
        "model": "gpt-4o-mini",
        "choices": [{
            "index": 0,
            "message": {
                "role": "assistant",
                "content": "Hello! I'm doing well, thank you for asking. How can I assist you today?"
            },
            "finish_reason": "stop",
            "logprobs": null
        }],
        "usage": {
            "prompt_tokens": 10,
            "completion_tokens": 15,
            "total_tokens": 25
        },
        "system_fingerprint": "fp_123456"
    });

    let response: ChatCompletionResponse = serde_json::from_value(response_json).unwrap();

    assert_eq!(response.id, "chatcmpl-123");
    assert_eq!(response.model, "gpt-4o-mini");
    assert_eq!(response.choices.len(), 1);
    assert_eq!(
        response.choices[0].message.content,
        Some("Hello! I'm doing well, thank you for asking. How can I assist you today?".to_string())
    );
    assert_eq!(response.usage.unwrap().total_tokens, 25);
    assert_eq!(response.system_fingerprint, Some("fp_123456".to_string()));
}

#[test]
fn test_chat_completion_with_tools() {
    let request = ChatCompletionRequest {
        messages: vec![ChatMessage {
            role: Role::User,
            content: Some("What's the weather in San Francisco?".to_string()),
            name: None,
            tool_calls: None,
            tool_call_id: None,
        }],
        model: "gpt-4o-mini".to_string(),
        tools: Some(vec![Tool {
            tool_type: "function".to_string(),
            function: FunctionDefinition {
                name: "get_weather".to_string(),
                description: Some("Get the current weather in a location".to_string()),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "location": {
                            "type": "string",
                            "description": "The city and state, e.g. San Francisco, CA"
                        }
                    },
                    "required": ["location"]
                }),
                strict: Some(true),
            },
        }]),
        tool_choice: Some(ToolChoice::String("auto".to_string())),
        parallel_tool_calls: Some(true),
        temperature: None,
        top_p: None,
        stream: None,
        max_tokens: None,
        n: None,
        stop: None,
        frequency_penalty: None,
        presence_penalty: None,
        logit_bias: None,
        logprobs: None,
        top_logprobs: None,
        seed: None,
        user: None,
        response_format: None,
        stream_options: None,
    };

    let serialized = serde_json::to_string(&request).unwrap();
    assert!(serialized.contains("get_weather"));
    assert!(serialized.contains("auto"));
    assert!(serialized.contains("parallel_tool_calls"));
}

#[test]
fn test_response_format_json_schema() {
    let request = ChatCompletionRequest {
        messages: vec![ChatMessage {
            role: Role::User,
            content: Some("Extract person information from the text".to_string()),
            name: None,
            tool_calls: None,
            tool_call_id: None,
        }],
        model: "gpt-4o-mini".to_string(),
        response_format: Some(ResponseFormat::JsonSchema {
            json_schema: JsonSchemaFormat {
                name: "person_schema".to_string(),
                description: Some("Schema for person information".to_string()),
                schema: json!({
                    "type": "object",
                    "properties": {
                        "name": {"type": "string"},
                        "age": {"type": "integer"},
                        "email": {"type": "string"}
                    },
                    "required": ["name", "age"]
                }),
                strict: Some(true),
            },
        }),
        temperature: None,
        top_p: None,
        stream: None,
        tools: None,
        tool_choice: None,
        max_tokens: None,
        n: None,
        stop: None,
        frequency_penalty: None,
        presence_penalty: None,
        logit_bias: None,
        logprobs: None,
        top_logprobs: None,
        seed: None,
        user: None,
        parallel_tool_calls: None,
        stream_options: None,
    };

    let serialized = serde_json::to_string(&request).unwrap();
    assert!(serialized.contains("json_schema"));
    assert!(serialized.contains("person_schema"));
    assert!(serialized.contains("strict"));
}

#[test]
fn test_stop_configuration() {
    // Test single stop sequence
    let stop_single = Stop::Single("\n".to_string());
    let serialized_single = serde_json::to_string(&stop_single).unwrap();
    assert_eq!(serialized_single, "\"\\n\"");

    // Test multiple stop sequences
    let stop_multiple = Stop::Multiple(vec!["\n".to_string(), ".".to_string()]);
    let serialized_multiple = serde_json::to_string(&stop_multiple).unwrap();
    assert_eq!(serialized_multiple, "[\"\\n\",\".\"]");
}

#[test]
fn test_tool_choice_enum() {
    // Test string tool choice
    let tool_choice_string = ToolChoice::String("auto".to_string());
    let serialized_string = serde_json::to_string(&tool_choice_string).unwrap();
    assert_eq!(serialized_string, "\"auto\"");

    // Test object tool choice
    let tool_choice_object = ToolChoice::Object(NamedToolChoice {
        tool_type: "function".to_string(),
        function: NamedToolChoiceFunction {
            name: "get_weather".to_string(),
        },
    });
    let serialized_object = serde_json::to_string(&tool_choice_object).unwrap();
    assert!(serialized_object.contains("get_weather"));
    assert!(serialized_object.contains("function"));
}

#[test]
fn test_role_enum() {
    let system_role = Role::System;
    let user_role = Role::User;
    let assistant_role = Role::Assistant;
    let tool_role = Role::Tool;

    assert_eq!(
        serde_json::to_string(&system_role).unwrap(),
        "\"system\""
    );
    assert_eq!(serde_json::to_string(&user_role).unwrap(), "\"user\"");
    assert_eq!(
        serde_json::to_string(&assistant_role).unwrap(),
        "\"assistant\""
    );
    assert_eq!(serde_json::to_string(&tool_role).unwrap(), "\"tool\"");
}

#[test]
fn test_finish_reason_enum() {
    let stop_reason = FinishReason::Stop;
    let length_reason = FinishReason::Length;
    let tool_calls_reason = FinishReason::ToolCalls;

    assert_eq!(serde_json::to_string(&stop_reason).unwrap(), "\"stop\"");
    assert_eq!(
        serde_json::to_string(&length_reason).unwrap(),
        "\"length\""
    );
    assert_eq!(
        serde_json::to_string(&tool_calls_reason).unwrap(),
        "\"tool_calls\""
    );
}