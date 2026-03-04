use eye::provider::deepseek::*;
use serde_json::json;
use std::env;

#[tokio::test]
async fn test_deepseek_api_integration() {
    // Only run this test if DEEPSEEK_API_KEY is set
    let api_key = match env::var("DEEPSEEK_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            println!("Skipping integration test: DEEPSEEK_API_KEY not set");
            return;
        }
    };

    let client = reqwest::Client::new();
    let request = DeepSeekRequest {
        messages: vec![
            Message::System {
                content: "You are a helpful assistant".to_string(),
                name: None,
            },
            Message::User {
                content: "Hello, who are you?".to_string(),
                name: None,
            },
        ],
        model: DeepSeekModel::DeepSeekChat,
        frequency_penalty: None,
        max_tokens: Some(100),
        presence_penalty: None,
        response_format: None,
        stop: None,
        stream: Some(false),
        stream_options: None,
        temperature: Some(0.7),
        top_p: None,
        tools: None,
        tool_choice: None,
        logprobs: None,
        top_logprobs: None,
        thinking: None,
    };

    let response = client
        .post("https://api.deepseek.com/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await
        .expect("Failed to send request");

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap();
        panic!("API request failed: {}", error_text);
    }

    let response_body: DeepSeekResponse = response
        .json()
        .await
        .expect("Failed to deserialize response");

    assert!(!response_body.choices.is_empty());
    match &response_body.choices[0].message {
        Message::Assistant { content, .. } => {
            assert!(content.is_some());
            println!("Response: {}", content.as_ref().unwrap());
        }
        _ => panic!("Expected assistant message"),
    }
}

#[test]
fn test_deepseek_request_serialization() {
    let request = DeepSeekRequest {
        messages: vec![
            Message::System {
                content: "You are a helpful assistant".to_string(),
                name: None,
            },
            Message::User {
                content: "Hello".to_string(),
                name: None,
            },
        ],
        model: DeepSeekModel::DeepSeekChat,
        frequency_penalty: Some(0.5),
        max_tokens: Some(1024),
        presence_penalty: Some(0.5),
        response_format: Some(ResponseFormat {
            type_: ResponseFormatType::JsonObject,
        }),
        stop: Some(Stop::Array(vec!["stop".to_string()])),
        stream: Some(true),
        stream_options: Some(StreamOptions {
            include_usage: Some(true),
        }),
        temperature: Some(0.8),
        top_p: Some(0.9),
        tools: None,
        tool_choice: None,
        logprobs: Some(true),
        top_logprobs: Some(5),
        thinking: Some(Thinking {
            type_: ThinkingType::Enabled,
        }),
    };

    let json = serde_json::to_value(&request).unwrap();

    assert_eq!(json["model"], "deepseek-chat");
    assert_eq!(json["frequency_penalty"], 0.5);
    assert_eq!(json["thinking"]["type"], "enabled");
    assert_eq!(json["response_format"]["type"], "json_object");
}

#[test]
fn test_deepseek_response_deserialization() {
    let json_data = json!({
        "id": "chatcmpl-123",
        "object": "chat.completion",
        "created": 1677652288,
        "model": "deepseek-chat",
        "system_fingerprint": "fp_44709d6fcb",
        "choices": [{
            "index": 0,
            "message": {
                "role": "assistant",
                "content": "Hello there!",
            },
            "finish_reason": "stop"
        }],
        "usage": {
            "prompt_tokens": 9,
            "completion_tokens": 12,
            "total_tokens": 21
        }
    });

    let response: DeepSeekResponse = serde_json::from_value(json_data).unwrap();

    assert_eq!(response.id, "chatcmpl-123");
    assert_eq!(response.model, "deepseek-chat");
    assert_eq!(response.choices.len(), 1);
    assert_eq!(response.choices[0].index, 0);

    match &response.choices[0].message {
        Message::Assistant { content, .. } => {
            assert_eq!(content.as_ref().unwrap(), "Hello there!");
        }
        _ => panic!("Expected assistant message"),
    }
}

#[test]
fn test_deepseek_model_enum() {
    let model = DeepSeekModel::DeepSeekReasoner;
    let json = serde_json::to_value(&model).unwrap();
    assert_eq!(json, "deepseek-reasoner");
}
