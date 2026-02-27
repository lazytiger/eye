# Git Hooks

This directory contains git hooks for the project. These hooks help maintain code quality and enforce project standards.

## Available Hooks

### pre-commit
Runs before a commit is created. Performs the following checks:
- Code formatting with `cargo fmt`
- Linting with `cargo clippy`
- Runs tests with `cargo test`

### commit-msg
Validates commit message format. Requires messages to follow [Conventional Commits](https://www.conventionalcommits.org/) format:
```
<type>(<scope>): <description>
```

Allowed types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `perf`: Performance improvements
- `test`: Adding or updating tests
- `build`: Build system changes
- `ci`: CI configuration changes
- `chore`: Maintenance tasks
- `revert`: Revert a previous commit

### pre-push
Runs before pushing to remote. Additional checks when pushing to `main` or `master` branch:
- Checks for TODO/FIXME comments
- Runs full test suite

### post-checkout
Runs after checking out a branch. Automatically updates dependencies if `Cargo.lock` has changed.

## Installation

Run the installation script:

```bash
./.githook/install.sh
```

This will:
1. Set execution permissions for all hooks
2. Configure git to use `.githook` as the hooks directory
3. Verify the configuration

## Manual Installation

If the installation script doesn't work, you can manually configure:

```bash
# Set execution permissions
chmod +x .githook/*

# Configure git to use .githook directory
git config core.hooksPath .githook

# Verify configuration
git config --get core.hooksPath
```

## Disabling Hooks Temporarily

To bypass hooks for a single command, use the `--no-verify` flag:

```bash
git commit -m "message" --no-verify
git push --no-verify
```

## Removing Hooks

To remove the hooks configuration:

```bash
git config --unset core.hooksPath
```

## Customizing Hooks

You can edit the hook scripts in this directory to customize their behavior. Each hook is a bash script that can be modified as needed.

## Requirements

- Git 2.9 or later (for `core.hooksPath` support)
- Rust toolchain (for pre-commit checks)
- Bash shell