//! Helper functions for API calls
//!
//! This module provides utility functions for making API calls to LLM providers.

use super::chat::{ChatRequest, ChatResponse};
use super::embedding::{EmbeddingRequest, EmbeddingResponse};
use crate::utils::reqwest_client;
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;

pub async fn call_chat_completions<
    RQ: TryFrom<ChatRequest, Error = anyhow::Error> + Serialize + Debug,
    RS: Into<ChatResponse> + DeserializeOwned,
>(
    url: &str,
    api_key: &str,
    request: ChatRequest,
) -> anyhow::Result<ChatResponse> {
    let req: RQ = request.try_into()?;
    tracing::debug!("Calling chat completions: {}", url);
    tracing::debug!("Request: {:?}", serde_json::to_string(&req)?);
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
        anyhow::bail!("Failed to call chat completions: {:?}", resp.text().await?);
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
