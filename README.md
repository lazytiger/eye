# rust-template
template for creating a rust programs, including cli options, loggers, errors

## Development

### Code Formatting
This project uses `imports_granularity = "Crate"` in `rustfmt.toml`, which requires nightly Rust for formatting.

**Format code:**
```bash
cargo +nightly fmt --all
```

**Check formatting:**
```bash
cargo +nightly fmt --all -- --check
```

### Git Hooks
Pre-commit hooks automatically check formatting, run clippy, and tests. Install hooks:
```bash
# Windows PowerShell
.\\.githook\\install.ps1

# Unix/Linux/macOS or Git Bash
./.githook/install.sh
```

### CI/CD
GitHub Actions automatically runs:
- Formatting checks (nightly only)
- Clippy checks
- Tests on stable and nightly Rust
- Cross-platform builds
- Security audits
