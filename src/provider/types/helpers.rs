//! Helper functions for API calls
//!
//! This module provides utility functions for making API calls to LLM providers.

use super::chat::{ChatRequest, ChatResponse};
use super::embedding::{EmbeddingRequest, EmbeddingResponse};
use crate::utils::reqwest_client;
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;
use std::time::Instant;

pub async fn call_chat_completions<
    RQ: TryFrom<ChatRequest, Error = anyhow::Error> + Serialize + Debug,
    RS: Into<ChatResponse> + DeserializeOwned,
>(
    url: &str,
    api_key: &str,
    request: ChatRequest,
) -> anyhow::Result<ChatResponse> {
    let req: RQ = request.try_into()?;
    let req_str = serde_json::to_string(&req).unwrap_or_else(|_| "<serialization failed>".to_string());

    tracing::info!("Calling chat completions: {}", url);
    tracing::debug!("Request body: {}", req_str);

    let start = Instant::now();

    let resp = reqwest_client()
        .post(url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&req)
        .send()
        .await;

    let send_duration = start.elapsed();
    tracing::debug!("Request sent in {:?}", send_duration);

    let resp = match resp {
        Ok(r) => r,
        Err(e) => {
            tracing::error!("Request failed after {:?}: {}", send_duration, e);
            if e.is_timeout() {
                anyhow::bail!("Request timeout after {:?}. The API took too long to respond.", send_duration);
            }
            if e.is_connect() {
                anyhow::bail!("Connection failed: {}. Check your network connection and API endpoint.", e);
            }
            anyhow::bail!("Request failed: {}", e);
        }
    };

    let status = resp.status();
    tracing::debug!("Response status: {}", status);

    if status.is_success() {
        let response = resp.json::<RS>().await?;
        let total_duration = start.elapsed();
        tracing::info!("Chat completions succeeded in {:?}", total_duration);
        Ok(response.into())
    } else {
        let error_text = resp.text().await?;
        tracing::error!("Chat completions failed with status {}: {}", status, error_text);
        anyhow::bail!("Failed to call chat completions: {}", error_text);
    }
}

pub async fn call_embedding<
    RQ: TryFrom<EmbeddingRequest, Error = anyhow::Error> + Serialize + Debug,
    RS: Into<EmbeddingResponse> + DeserializeOwned,
>(
    url: &str,
    api_key: &str,
    request: EmbeddingRequest,
) -> anyhow::Result<EmbeddingResponse> {
    let req: RQ = request.try_into()?;
    tracing::debug!("Calling embedding: {}", url);
    let resp = reqwest_client()
        .post(url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&req)
        .send()
        .await?;
    if resp.status().is_success() {
        let response = resp.json::<RS>().await?;
        Ok(response.into())
    } else {
        anyhow::bail!("Failed to call embedding: {:?}", resp.text().await?);
    }
}
