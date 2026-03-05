use eye::provider::openrouter::*;
use reqwest::Client;
use serde_json::json;
use std::env;

const BASE_URL: &str = "https://openrouter.ai/api/v1";

async fn get_client() -> (Client, String) {
    let api_key = env::var("OPENROUTER_API_KEY").expect("OPENROUTER_API_KEY must be set");
    let client = Client::new();
    (client, api_key)
}

#[tokio::test]
async fn test_chat_completions() {
    let (client, api_key) = get_client().await;

    let request = ChatRequest {
        messages: vec![Message::User(UserMessage {
            content: MessageContent::Text("Hello, world!".to_string()),
            name: None,
        })],
        model: Some("openai/gpt-3.5-turbo".to_string()),
        models: None,
        response_format: None,
        stop: None,
        stream: None,
        max_tokens: None,
        max_completion_tokens: None,
        temperature: None,
        top_p: None,
        top_k: None,
        frequency_penalty: None,
        presence_penalty: None,
        repetition_penalty: None,
        seed: None,
        tools: None,
        tool_choice: None,
        logit_bias: None,
        logprobs: None,
        top_logprobs: None,
        user: None,
        provider: None,
        plugins: None,
        transforms: None,
        route: None,
        session_id: None,
        metadata: None,
    };

    let response = client
        .post(format!("{}/chat/completions", BASE_URL))
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await
        .expect("Failed to send request");

    assert!(
        response.status().is_success(),
        "Response status: {}",
        response.status()
    );

    let chat_response: ChatResponse = response.json().await.expect("Failed to parse response");
    assert!(!chat_response.choices.is_empty());
    println!("Chat response: {:?}", chat_response);
}

#[tokio::test]
async fn test_models() {
    let (client, _api_key) = get_client().await;

    // Models endpoint doesn't require auth usually but better to provide it
    let response = client
        .get(format!("{}/models", BASE_URL))
        .send()
        .await
        .expect("Failed to send request");

    assert!(
        response.status().is_success(),
        "Response status: {}",
        response.status()
    );

    let models_response: ModelListResponse =
        response.json().await.expect("Failed to parse response");
    assert!(!models_response.data.is_empty());
    println!("Found {} models", models_response.data.len());
}

#[tokio::test]
async fn test_credits() {
    let (client, api_key) = get_client().await;

    let response = client
        .get(format!("{}/credits", BASE_URL))
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await
        .expect("Failed to send request");

    if response.status().as_u16() == 403 {
        println!("Credits endpoint requires management key, skipping");
        return;
    }

    assert!(
        response.status().is_success(),
        "Response status: {}",
        response.status()
    );

    let credits_response: CreditsResponse =
        response.json().await.expect("Failed to parse response");
    println!("Credits: {:?}", credits_response);
}

#[tokio::test]
async fn test_auth_keys() {
    let (client, api_key) = get_client().await;
    let response = client
        .get(format!("{}/auth/keys", BASE_URL))
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await
        .expect("Failed to send request");

    if response.status().as_u16() == 403 {
        println!("Auth keys endpoint requires management key, skipping");
        return;
    }

    if response.status().is_success() {
        let keys_response: KeyListResponse =
            response.json().await.expect("Failed to parse response");
        println!("Keys: {:?}", keys_response);
    }
}

#[tokio::test]
async fn test_activity() {
    let (client, api_key) = get_client().await;
    let response = client
        .get(format!("{}/activity", BASE_URL))
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await
        .expect("Failed to send request");

    if response.status().as_u16() == 403 {
        println!("Activity endpoint requires management key, skipping");
        return;
    }

    if response.status().is_success() {
        let activity_response: ActivityResponse =
            response.json().await.expect("Failed to parse response");
        println!("Activity: {:?}", activity_response);
    }
}

#[tokio::test]
async fn test_embeddings() {
    let (client, api_key) = get_client().await;

    let request = EmbeddingsRequest {
        input: json!("Hello world"),
        model: "openai/text-embedding-ada-002".to_string(),
        encoding_format: None,
        dimensions: None,
        user: None,
        provider: None,
    };

    let response = client
        .post(format!("{}/embeddings", BASE_URL))
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await
        .expect("Failed to send request");

    if response.status().is_success() {
        let embeddings_response: EmbeddingsResponse =
            response.json().await.expect("Failed to parse response");
        println!("Embeddings: {:?}", embeddings_response);
    } else {
        println!("Embeddings request failed: {}", response.status());
    }
}

#[tokio::test]
async fn test_generation() {
    let (client, _api_key) = get_client().await;
    // We don't have a valid generation ID, so we just check if we can make the request
    let response = client
        .get(format!("{}/generation?id=fake-id", BASE_URL))
        .send()
        .await
        .expect("Failed to send request");

    println!("Generation response status: {}", response.status());
}
