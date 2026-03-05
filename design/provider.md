# Model Provider Abstract Design

## Overview
The `Provider` trait serves as a unified abstraction layer for interacting with various Large Language Model (LLM) providers (e.g., OpenAI, Anthropic, OpenRouter, DeepSeek). This design ensures that the application can switch between different providers seamlessly without changing the core business logic.

## Core Trait Definition

The `Provider` trait is designed to be asynchronous and thread-safe.

```rust
use async_trait::async_trait;
use crate::provider::types::{ChatRequest, ChatResponse, EmbeddingRequest, EmbeddingResponse, ModelCapabilities};
use anyhow::Result;
use futures::Stream;
use std::pin::Pin;

pub type BoxStream<T> = Pin<Box<dyn Stream<Item = Result<T>> + Send>>;

#[async_trait]
pub trait Provider: Send + Sync {
    /// Returns the unique identifier/name of the provider (e.g., "openai", "openrouter").
    fn name(&self) -> &str;

    /// Sends a chat completion request and returns the full response.
    /// Used for non-streaming interactions.
    async fn chat(&self, request: ChatRequest) -> Result<ChatResponse>;

    /// Generates embeddings for the given input text.
    /// Essential for RAG (Retrieval-Augmented Generation) features.
    async fn embedding(&self, request: EmbeddingRequest) -> Result<EmbeddingResponse>;

    /// Returns the capabilities of the currently configured model (e.g., vision, function calling).
    /// This helps the client know what features are available without try-and-error.
    fn capabilities(&self) -> ModelCapabilities;

    /// Returns the maximum context length (in tokens) for the currently configured model.
    /// Used for context window management to avoid overflow errors.
    fn max_context_length(&self) -> usize;
}
```

## Function Details

### 1. `name`
- **Purpose**: Identify the provider instance.
- **Usage**: Logging, configuration debugging, and conditional logic if strictly necessary.

### 2. `chat`
- **Purpose**: The primary interaction method for getting a complete response from the model.
- **Input**: `ChatRequest` struct containing messages, model parameters (temperature, top_p), tools, etc.
- **Output**: `ChatResponse` struct containing the generated message, usage stats, and finish reason.

### 3. `embedding`
- **Purpose**: Converts text into vector representations.
- **Input**: `EmbeddingRequest` (list of strings).
- **Output**: `EmbeddingResponse` (list of vectors).

### 4. `capabilities`
- **Purpose**: Returns the supported features of the configured model.
- **Output**: `ModelCapabilities` struct/bitflags.
- **Usage**:
  - Disable "Upload Image" button if vision is not supported.
  - Hide "Audio" input if audio is not supported.
  - Determine if function calling can be used.

### 5. `max_context_length`
- **Purpose**: Returns the hard limit of tokens the model can handle (input + output).
- **Usage**: The conversation history manager uses this to truncate or summarize old messages.

## Data Types

The design relies on standardizing the input/output structs in `crate::provider::types`.

- **ChatRequest**: Unified chat request body (OpenAI-compatible).
- **ChatResponse**: Unified chat response body.
- **EmbeddingRequest**: `{ input: Vec<String>, model: String }`
- **EmbeddingResponse**: `{ data: Vec<EmbeddingObject>, usage: Usage }`
- **ModelCapabilities**: A bitflags or struct representing supported features.

```rust
bitflags::bitflags! {
    pub struct ModelCapabilities: u32 {
        const TEXT_GENERATION = 1 << 0;
        const FUNCTION_CALLING = 1 << 1;
        const VISION = 1 << 2;
        const AUDIO_INPUT = 1 << 3;
        const OBJECT_GENERATION = 1 << 4; // JSON mode
    }
}
```

## Extension Points

To add a new provider (e.g., `Anthropic`):
1. Create a struct `AnthropicProvider`.
2. Implement the `Provider` trait.
3. Map internal types to the generic `ChatRequest`/`ChatResponse` types.
4. Register the provider in the factory method (e.g., `create_provider`).
