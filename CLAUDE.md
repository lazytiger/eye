# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build and Test Commands

```bash
# Build
cargo build
cargo build --release

# Run
cargo run -- chat              # Interactive chat mode
cargo run -- query "text"      # Single query mode

# Test
cargo test                     # Run all tests
cargo test -- --ignored        # Run integration tests (requires API keys)
cargo test <test_name>         # Run specific test

# Format and lint
cargo fmt
cargo clippy
```

## Architecture Overview

Eye is a personal intelligent assistant that enables LLMs to interact with the real world through tool calling. The
architecture follows a clean modular design:

```
src/
├── main.rs              # CLI entry point (tokio runtime)
├── lib.rs               # Library exports and init_tracing
├── agent/               # Agent abstraction (LLM conversation loop)
├── config/
│   ├── cli.rs           # Command-line parsing (clap)
│   └── settings.rs      # TOML configuration (eye.toml)
├── provider/            # LLM provider abstraction
│   ├── types.rs         # Unified request/response types
│   ├── openai.rs        # OpenAI provider
│   ├── openrouter.rs    # OpenRouter provider
│   ├── deepseek.rs      # DeepSeek provider
│   └── compatible.rs    # OpenAI-compatible custom providers
├── tool/                # Tool system
│   ├── shell.rs         # Shell command execution
│   ├── time.rs          # Current time utility
│   ├── search.rs        # Web search
│   └── web_fetch.rs     # Webpage content retrieval
├── interface/           # User interface abstraction
│   └── cli.rs           # CLI interface implementation
├── skill/               # Skill system (extensible capabilities)
└── memory/              # Conversation history and memory
```

## Core Abstractions

### Provider Trait (`src/provider/mod.rs`)

All LLM providers implement this trait:

- `chat()` - Send chat completion requests
- `embedding()` - Generate embeddings
- `capabilities()` - Check model features (vision, function calling)
- `max_context_length()` - Get token limit

Factory function: `create_provider(name, model, api_key)` supports:

- Built-in: `"openai"`, `"openrouter"`, `"deepseek"`
- Custom: `"name:https://endpoint.com/v1"` format

### Tool Trait (`src/tool/mod.rs`)

All tools implement:

- `name()`, `description()`, `parameters()` - Tool metadata
- `execute(&self, args: Value) -> Result<ExecuteResult>` - Execution logic

Tools are registered and passed to providers for function calling.

### Configuration

Uses `eye.toml` with clap overrides. Key sections:

- `[openrouter]` - API key, endpoint, default model
- `[model]` - temperature, max_tokens, stream
- `[tools]` - enabled tools list
- `[tools.shell]` - allowed commands whitelist

## Development Guidelines

- **Platform**: Windows development (PowerShell preferred over cmd)
- **Async**: tokio runtime, use `async_trait` for trait methods
- **Error handling**: Use `anyhow::Result`, return `ExecuteResult::Success/Failure` for tool outcomes
- **Testing**: Unit tests in source files, integration tests in `tests/`
- **API keys**: Use environment variables (`OPENROUTER_API_KEY`, `DEEPSEEK_API_KEY`, etc.)
- **Modules**: <500 lines
- **Dependencies**: Search in `crates.io` for libraries that fit the project needs most and use the latest version.
- **API**: Search in `docs.rs` for Rust library documentation when method signatures don't match.

## Running Integration Tests

Integration tests require real API keys:

```bash
# Set API key
$env:OPENROUTER_API_KEY="your-key"

# Run ignored tests
cargo test -- --ignored
```
