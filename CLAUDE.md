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
cargo run -- list-routes       # List model routes
cargo run -- list-tools        # List available tools

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

Configuration file location: `~/.eye/config.toml`

**Required Fields:**
- `active_route` - Name of the active model route (must match a route name)
- `[[model_routes]]` - Array of model route configurations (at least one required)

**Model Route Configuration:**

Each `[[model_routes]]` entry supports:
- `name` - Unique identifier for the route (e.g., "fast", "smart")
- `provider` - Provider name: "openai", "openrouter", "deepseek", or custom "name:endpoint"
- `model` - Model identifier (e.g., "gpt-4o", "claude-3-opus")
- `api_key` - API key (optional, can use env var `PROVIDER_API_KEY`)
- `endpoint` - Optional custom endpoint for compatible providers
- `temperature` - Optional temperature override (0-2)
- `max_tokens` - Optional max tokens override
- `stream` - Optional streaming toggle

**Optional Sections:**
- `[model]` - Default model parameters (temperature, max_tokens, stream)
- `[tools]` - Tool configuration
- `[tools.shell]` - Shell tool settings
- `[interface]` - UI configuration
- `[agent]` - Agent settings (system prompt)

**Example Configuration:**
```toml
active_route = "fast"

[model]
temperature = 0.7
stream = true

[[model_routes]]
name = "fast"
provider = "openrouter"
model = "openai/gpt-4o-mini"
api_key = ""  # Uses OPENROUTER_API_KEY env var

[[model_routes]]
name = "smart"
provider = "openrouter"
model = "anthropic/claude-3-opus"

[[model_routes]]
name = "deepseek"
provider = "deepseek"
model = "deepseek-chat"
# Uses DEEPSEEK_API_KEY env var

[[model_routes]]
name = "bytedance"
provider = "bytedance:https://ark.cn-beijing.volces.com/api/coding/v3"
model = "ark-code-latest"
# Uses BYTEDANCE_API_KEY env var
```

**Backwards Compatibility:**

Legacy configs with `[openrouter]` section are automatically migrated:
```toml
# Legacy format (still supported)
[openrouter]
api_key = "sk-..."
default_model = "openai/gpt-4o-mini"
```

This is automatically converted to a single `model_routes` entry with name "default".

**API Key Priority:**
1. Environment variable: `PROVIDER_API_KEY` (uppercase provider name, e.g., `OPENROUTER_API_KEY`)
2. Config file `api_key` field in route

**CLI Commands:**
- `cargo run -- list-routes` - List all configured model routes

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
