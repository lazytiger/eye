# Eye - Personal Intelligent Assistant

A personal intelligent assistant that enables large language models to interact with the real world through tool calling.

## Features

- **Model Provider Abstraction**: Support for OpenRouter (with OpenAI API compatibility)
- **Tool System**: Extensible tool calling system with Shell command execution
- **Interface Abstraction**: CLI interface with support for future GUI/Web interfaces
- **Configuration Management**: TOML-based configuration with CLI overrides
- **Skill System**: Extensible skill framework for specialized capabilities
- **Cross-platform**: Windows compatibility with PowerShell support

## Architecture

The project is organized into modular components:

```
src/
├── main.rs                # CLI entry point
├── lib.rs                 # Library exports
├── agent/                # Agent abstraction
│   └── mod.rs           # Agent struct and execution logic
├── config/               # Configuration management
│   ├── cli.rs           # Command-line argument parsing (clap)
│   └── settings.rs      # TOML configuration structure
├── model/               # Model provider abstraction
│   ├── trait.rs         # ModelProvider trait definition
│   └── openrouter.rs    # OpenRouter implementation
├── tool/                # Tool system
│   ├── trait.rs         # Tool trait definition
│   ├── shell.rs         # Shell command execution tool
│   └── mod.rs           # Tool manager
├── interface/           # User interface abstraction
│   ├── trait.rs         # Interface trait definition
│   └── cli.rs           # CLI interface implementation
└── skill/              # Skill system
    ├── trait.rs         # Skill trait definition
    └── mod.rs           # Skill manager
```

## Usage

### Installation

```bash
cargo build --release
```

### Configuration

Create a configuration file `eye.toml`:

```toml
[openrouter]
api_key = "your-openrouter-api-key"
endpoint = "https://openrouter.ai/api/v1"
default_model = "openai/gpt-4o-mini"

[model]
temperature = 0.7
max_tokens = 2048
stream = true

[tools]
enabled = ["shell"]

[tools.shell]
allowed_commands = ["ls", "pwd", "echo", "dir", "cd"]
allow_any_command = false
timeout_seconds = 30

[interface]
prompt = "eye> "
show_timestamp = false
enable_colors = true
```

### Command Line Usage

```bash
# Interactive chat mode
eye chat

# Single query mode
eye query "What's the weather like?"

# List available tools
eye list-tools

# List available skills
eye list-skills

# With custom configuration
eye --config custom.toml chat

# With API key from environment variable
export OPENROUTER_API_KEY=your-key
eye chat
```

## Development Status

✅ **Completed**:
- Project structure and module organization
- Configuration system (TOML + clap)
- Model provider abstraction (trait)
- Tool system abstraction (trait + Shell implementation)
- Interface abstraction (trait + CLI implementation)
- Skill system abstraction (trait)
- Agent abstraction layer
- Main application integration
- Comprehensive test coverage
- Windows compatibility

🔄 **In Progress**:
- OpenRouter API integration (mock implementation)
- Additional tool implementations
- Skill implementations

📋 **Planned**:
- Web interface
- GUI interface
- Additional model providers
- Plugin system

## Testing

Run the test suite:

```bash
cargo test
```

## Dependencies

- **clap**: Command-line argument parsing
- **tokio**: Async runtime
- **serde/serde_json**: JSON serialization
- **toml**: Configuration file format
- **async-trait**: Async trait support
- **openai-api-rs**: OpenRouter/OpenAI API client

## License

[Add your license here]