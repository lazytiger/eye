//! OpenAI API integration tests
//!
//! These tests verify the OpenAI API request and response structs
//! by testing serialization and deserialization with example data.

use eye::provider::openai::*;
use serde_json::json;

#[test]
fn test_chat_completion_request_serialization() {
    let request = CreateChatCompletionRequest {
        messages: vec![
            ChatCompletionRequestMessage::System {
                content: "You are a helpful assistant.".to_string(),
                name: None,
            },
            ChatCompletionRequestMessage::User {
                content: ChatCompletionRequestMessageContent::Text(
                    "Hello, how are you?".to_string(),
                ),
                name: None,
            },
        ],
        model: "gpt-4o".to_string(),
        modalities: Some(vec![ResponseModality::Text]),
        reasoning_effort: Some(ReasoningEffort::Medium),
        max_completion_tokens: Some(100),
        frequency_penalty: Some(0.0),
        presence_penalty: Some(0.0),
        web_search_options: None,
        top_logprobs: None,
        response_format: Some(ResponseFormat::Text),
        audio: None,
        store: Some(false),
        stream: Some(false),
        stop: Some(StopConfiguration::Single("\n".to_string())),
        logit_bias: None,
        logprobs: Some(false),
        max_tokens: None,
        n: Some(1),
        prediction: None,
        seed: Some(42),
        stream_options: Some(ChatCompletionStreamOptions {
            include_usage: Some(true),
        }),
        tools: None,
        tool_choice: None,
        parallel_tool_calls: Some(true),
        function_call: None,
        functions: None,
        temperature: Some(0.7),
        top_p: Some(0.9),
        user: Some("test-user".to_string()),
        session_id: Some("test-session".to_string()),
    };

    let serialized = serde_json::to_string(&request).unwrap();
    let deserialized: CreateChatCompletionRequest = serde_json::from_str(&serialized).unwrap();

    assert_eq!(deserialized.model, "gpt-4o");
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
        "model": "gpt-4o",
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
        }
    });

    let response: CreateChatCompletionResponse =
        serde_json::from_value(response_json).unwrap();

    assert_eq!(response.id, "chatcmpl-123");
    assert_eq!(response.model, "gpt-4o");
    assert_eq!(response.choices.len(), 1);
    assert_eq!(
        response.choices[0].message.content,
        Some("Hello! I'm doing well, thank you for asking. How can I assist you today?".to_string())
    );
    assert_eq!(response.usage.unwrap().total_tokens, 25);
}

#[test]
fn test_chat_completion_with_tools() {
    let request = CreateChatCompletionRequest {
        messages: vec![ChatCompletionRequestMessage::User {
            content: ChatCompletionRequestMessageContent::Text(
                "What's the weather in San Francisco?".to_string(),
            ),
            name: None,
        }],
        model: "gpt-4o".to_string(),
        tools: Some(vec![ChatCompletionTool {
            tool_type: "function".to_string(),
            function: ChatCompletionToolFunction {
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
        tool_choice: Some(ChatCompletionToolChoiceOption::String("auto".to_string())),
        parallel_tool_calls: Some(true),
        modalities: None,
        reasoning_effort: None,
        max_completion_tokens: None,
        frequency_penalty: None,
        presence_penalty: None,
        web_search_options: None,
        top_logprobs: None,
        response_format: None,
        audio: None,
        store: None,
        stream: None,
        stop: None,
        logit_bias: None,
        logprobs: None,
        max_tokens: None,
        n: None,
        prediction: None,
        seed: None,
        stream_options: None,
        function_call: None,
        functions: None,
        temperature: None,
        top_p: None,
        user: None,
        session_id: None,
    };

    let serialized = serde_json::to_string(&request).unwrap();
    assert!(serialized.contains("get_weather"));
    assert!(serialized.contains("function"));
}

#[test]
fn test_completion_request_serialization() {
    let request = CreateCompletionRequest {
        model: "text-davinci-003".to_string(),
        prompt: CompletionPrompt::Text("Once upon a time".to_string()),
        suffix: Some("and they lived happily ever after.".to_string()),
        max_tokens: Some(50),
        temperature: Some(0.7),
        top_p: Some(0.9),
        n: Some(1),
        stream: Some(false),
        logprobs: Some(5),
        echo: Some(false),
        stop: Some(StopConfiguration::Multiple(vec![
            "\n".to_string(),
            ".".to_string(),
        ])),
        presence_penalty: Some(0.0),
        frequency_penalty: Some(0.0),
        best_of: Some(1),
        logit_bias: None,
        user: Some("test-user".to_string()),
        seed: Some(123),
    };

    let serialized = serde_json::to_string(&request).unwrap();
    let deserialized: CreateCompletionRequest = serde_json::from_str(&serialized).unwrap();

    assert_eq!(deserialized.model, "text-davinci-003");
    assert_eq!(deserialized.max_tokens, Some(50));
    assert_eq!(deserialized.temperature, Some(0.7));
}

#[test]
fn test_embedding_request_serialization() {
    let request = CreateEmbeddingRequest {
        input: EmbeddingInput::Text("The food was delicious and the waiter...".to_string()),
        model: "text-embedding-3-small".to_string(),
        encoding_format: Some(EmbeddingEncodingFormat::Float),
        dimensions: Some(256),
        user: Some("test-user".to_string()),
    };

    let serialized = serde_json::to_string(&request).unwrap();
    let deserialized: CreateEmbeddingRequest = serde_json::from_str(&serialized).unwrap();

    assert_eq!(deserialized.model, "text-embedding-3-small");
    assert_eq!(deserialized.dimensions, Some(256));
}

#[test]
fn test_embedding_response_deserialization() {
    let response_json = json!({
        "object": "list",
        "data": [{
            "object": "embedding",
            "index": 0,
            "embedding": [0.1, 0.2, 0.3, 0.4, 0.5]
        }],
        "model": "text-embedding-3-small",
        "usage": {
            "prompt_tokens": 8,
            "total_tokens": 8
        }
    });

    let response: CreateEmbeddingResponse = serde_json::from_value(response_json).unwrap();

    assert_eq!(response.model, "text-embedding-3-small");
    assert_eq!(response.data.len(), 1);
    assert_eq!(response.data[0].embedding.len(), 5);
    assert_eq!(response.usage.total_tokens, 8);
}

#[test]
fn test_image_generation_request_serialization() {
    let request = CreateImageRequest {
        prompt: "A cute baby sea otter".to_string(),
        model: Some("dall-e-3".to_string()),
        n: Some(1),
        size: Some(ImageSize::Size1024x1024),
        response_format: Some(ImageResponseFormat::Url),
        quality: Some(ImageQuality::Standard),
        style: Some(ImageStyle::Vivid),
        user: Some("test-user".to_string()),
    };

    let serialized = serde_json::to_string(&request).unwrap();
    let deserialized: CreateImageRequest = serde_json::from_str(&serialized).unwrap();

    assert_eq!(deserialized.prompt, "A cute baby sea otter");
    assert_eq!(deserialized.model, Some("dall-e-3".to_string()));
    assert_eq!(deserialized.size, Some(ImageSize::Size1024x1024));
}

#[test]
fn test_image_generation_response_deserialization() {
    let response_json = json!({
        "created": 1589478378,
        "data": [{
            "url": "https://example.com/image.png",
            "revised_prompt": "A cute baby sea otter wearing a bowtie"
        }]
    });

    let response: CreateImageResponse = serde_json::from_value(response_json).unwrap();

    assert_eq!(response.created, 1589478378);
    assert_eq!(response.data.len(), 1);
    assert_eq!(
        response.data[0].url,
        Some("https://example.com/image.png".to_string())
    );
    assert_eq!(
        response.data[0].revised_prompt,
        Some("A cute baby sea otter wearing a bowtie".to_string())
    );
}

#[test]
fn test_transcription_request_serialization() {
    let request = CreateTranscriptionRequest {
        file: "audio.mp3".to_string(),
        model: "whisper-1".to_string(),
        language: Some("en".to_string()),
        prompt: Some("Transcribe this audio file".to_string()),
        response_format: Some(TranscriptionResponseFormat::Json),
        temperature: Some(0.0),
        timestamp_granularities: Some(vec![
            TranscriptionTimestampGranularity::Word,
            TranscriptionTimestampGranularity::Segment,
        ]),
    };

    let serialized = serde_json::to_string(&request).unwrap();
    let deserialized: CreateTranscriptionRequest = serde_json::from_str(&serialized).unwrap();

    assert_eq!(deserialized.model, "whisper-1");
    assert_eq!(deserialized.language, Some("en".to_string()));
    assert_eq!(deserialized.temperature, Some(0.0));
}

#[test]
fn test_models_list_deserialization() {
    let response_json = json!({
        "object": "list",
        "data": [{
            "id": "gpt-4o",
            "object": "model",
            "created": 1686935000,
            "owned_by": "openai"
        }, {
            "id": "gpt-3.5-turbo",
            "object": "model",
            "created": 1677610600,
            "owned_by": "openai"
        }]
    });

    let response: ListModelsResponse = serde_json::from_value(response_json).unwrap();

    assert_eq!(response.object, "list");
    assert_eq!(response.data.len(), 2);
    assert_eq!(response.data[0].id, "gpt-4o");
    assert_eq!(response.data[1].id, "gpt-3.5-turbo");
}

#[test]
fn test_moderation_request_serialization() {
    let request = CreateModerationRequest {
        input: ModerationInput::Text("I want to hurt someone.".to_string()),
        model: Some("text-moderation-latest".to_string()),
    };

    let serialized = serde_json::to_string(&request).unwrap();
    let deserialized: CreateModerationRequest = serde_json::from_str(&serialized).unwrap();

    assert_eq!(
        deserialized.model,
        Some("text-moderation-latest".to_string())
    );
}

#[test]
fn test_moderation_response_deserialization() {
    let response_json = json!({
        "id": "modr-123",
        "model": "text-moderation-007",
        "results": [{
            "flagged": true,
            "categories": {
                "hate": true,
                "hate/threatening": false,
                "harassment": false,
                "harassment/threatening": false,
                "self-harm": false,
                "self-harm/intent": false,
                "self-harm/instructions": false,
                "sexual": false,
                "sexual/minors": false,
                "violence": true,
                "violence/graphic": false
            },
            "category_scores": {
                "hate": 0.8,
                "hate/threatening": 0.1,
                "harassment": 0.05,
                "harassment/threatening": 0.02,
                "self-harm": 0.01,
                "self-harm/intent": 0.01,
                "self-harm/instructions": 0.01,
                "sexual": 0.02,
                "sexual/minors": 0.01,
                "violence": 0.9,
                "violence/graphic": 0.1
            }
        }]
    });

    let response: CreateModerationResponse = serde_json::from_value(response_json).unwrap();

    assert_eq!(response.id, "modr-123");
    assert_eq!(response.model, "text-moderation-007");
    assert_eq!(response.results.len(), 1);
    assert!(response.results[0].flagged);
    assert!(response.results[0].categories.violence);
    assert!(!response.results[0].categories.sexual);
    assert_eq!(response.results[0].category_scores.violence, 0.9);
}

#[test]
fn test_chat_completion_with_image_content() {
    let request = CreateChatCompletionRequest {
        messages: vec![ChatCompletionRequestMessage::User {
            content: ChatCompletionRequestMessageContent::Array(vec![
                ChatCompletionRequestMessageContentPart::Text {
                    text: "What's in this image?".to_string(),
                },
                ChatCompletionRequestMessageContentPart::ImageUrl {
                    image_url: ChatCompletionRequestMessageContentPartImageUrl {
                        url: "https://example.com/image.jpg".to_string(),
                        detail: Some(ImageDetail::High),
                    },
                },
            ]),
            name: None,
        }],
        model: "gpt-4o".to_string(),
        modalities: None,
        reasoning_effort: None,
        max_completion_tokens: None,
        frequency_penalty: None,
        presence_penalty: None,
        web_search_options: None,
        top_logprobs: None,
        response_format: None,
        audio: None,
        store: None,
        stream: None,
        stop: None,
        logit_bias: None,
        logprobs: None,
        max_tokens: None,
        n: None,
        prediction: None,
        seed: None,
        stream_options: None,
        tools: None,
        tool_choice: None,
        parallel_tool_calls: None,
        function_call: None,
        functions: None,
        temperature: None,
        top_p: None,
        user: None,
        session_id: None,
    };

    let serialized = serde_json::to_string(&request).unwrap();
    assert!(serialized.contains("image_url"));
    assert!(serialized.contains("high"));
}

#[test]
fn test_chat_completion_with_audio_output() {
    let request = CreateChatCompletionRequest {
        messages: vec![ChatCompletionRequestMessage::User {
            content: ChatCompletionRequestMessageContent::Text(
                "Say hello in a friendly voice".to_string(),
            ),
            name: None,
        }],
        model: "gpt-4o".to_string(),
        modalities: Some(vec![ResponseModality::Audio]),
        reasoning_effort: None,
        max_completion_tokens: None,
        frequency_penalty: None,
        presence_penalty: None,
        web_search_options: None,
        top_logprobs: None,
        response_format: None,
        audio: Some(AudioOutputParameters {
            voice: VoiceId::Nova,
            format: AudioFormat::Mp3,
        }),
        store: None,
        stream: None,
        stop: None,
        logit_bias: None,
        logprobs: None,
        max_tokens: None,
        n: None,
        prediction: None,
        seed: None,
        stream_options: None,
        tools: None,
        tool_choice: None,
        parallel_tool_calls: None,
        function_call: None,
        functions: None,
        temperature: None,
        top_p: None,
        user: None,
        session_id: None,
    };

    let serialized = serde_json::to_string(&request).unwrap();
    assert!(serialized.contains("audio"));
    assert!(serialized.contains("nova"));
    assert!(serialized.contains("mp3"));
}

#[test]
fn test_chat_completion_with_web_search() {
    let request = CreateChatCompletionRequest {
        messages: vec![ChatCompletionRequestMessage::User {
            content: ChatCompletionRequestMessageContent::Text(
                "What's the latest news about AI?".to_string(),
            ),
            name: None,
        }],
        model: "gpt-4o".to_string(),
        modalities: None,
        reasoning_effort: None,
        max_completion_tokens: None,
        frequency_penalty: None,
        presence_penalty: None,
        web_search_options: Some(WebSearchOptions {
            user_location: Some(WebSearchUserLocation {
                location_type: "approximate".to_string(),
                approximate: WebSearchLocation {
                    country: "US".to_string(),
                    region: Some("CA".to_string()),
                    city: Some("San Francisco".to_string()),
                },
            }),
            search_context_size: Some(WebSearchContextSize::High),
        }),
        top_logprobs: None,
        response_format: None,
        audio: None,
        store: None,
        stream: None,
        stop: None,
        logit_bias: None,
        logprobs: None,
        max_tokens: None,
        n: None,
        prediction: None,
        seed: None,
        stream_options: None,
        tools: None,
        tool_choice: None,
        parallel_tool_calls: None,
        function_call: None,
        functions: None,
        temperature: None,
        top_p: None,
        user: None,
        session_id: None,
    };

    let serialized = serde_json::to_string(&request).unwrap();
    assert!(serialized.contains("web_search_options"));
    assert!(serialized.contains("US"));
    assert!(serialized.contains("San Francisco"));
}

#[test]
fn test_chat_completion_with_json_schema() {
    let request = CreateChatCompletionRequest {
        messages: vec![ChatCompletionRequestMessage::User {
            content: ChatCompletionRequestMessageContent::Text(
                "Extract person information from the text".to_string(),
            ),
            name: None,
        }],
        model: "gpt-4o".to_string(),
        modalities: None,
        reasoning_effort: None,
        max_completion_tokens: None,
        frequency_penalty: None,
        presence_penalty: None,
        web_search_options: None,
        top_logprobs: None,
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
        audio: None,
        store: None,
        stream: None,
        stop: None,
        logit_bias: None,
        logprobs: None,
        max_tokens: None,
        n: None,
        prediction: None,
        seed: None,
        stream_options: None,
        tools: None,
        tool_choice: None,
        parallel_tool_calls: None,
        function_call: None,
        functions: None,
        temperature: None,
        top_p: None,
        user: None,
        session_id: None,
    };

    let serialized = serde_json::to_string(&request).unwrap();
    assert!(serialized.contains("json_schema"));
    assert!(serialized.contains("person_schema"));
    assert!(serialized.contains("strict"));
}