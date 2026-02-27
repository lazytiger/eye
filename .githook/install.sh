#!/bin/bash

echo "Installing git hooks..."

# 检查是否在git仓库中
if [ ! -d ".git" ]; then
    echo "Error: This is not a git repository."
    exit 1
fi

# 设置执行权限（兼容Windows和Unix系统）
if [[ "$OSTYPE" == "msys" || "$OSTYPE" == "cygwin" || "$OSTYPE" == "win32" ]]; then
    # Windows系统
    echo "Windows system detected, setting execute permissions..."
    Get-ChildItem .githook\* | ForEach-Object { icacls $_.FullName /grant:r "$($env:USERNAME):(RX)" }
else
    # Unix系统
    echo "Unix system detected, setting execute permissions..."
    chmod +x .githook/*
fi

# 配置git使用.githook目录作为hook路径
git config core.hooksPath .githook

# 验证配置
HOOKS_PATH=$(git config --get core.hooksPath)
if [ "$HOOKS_PATH" = ".githook" ]; then
    echo "Successfully configured git hooks path to .githook"
    echo "Available hooks:"
    if [[ "$OSTYPE" == "msys" || "$OSTYPE" == "cygwin" || "$OSTYPE" == "win32" ]]; then
        dir .githook/
    else
        ls -la .githook/
    fi
else
    echo "Error: Failed to configure git hooks path."
    exit 1
fi

echo ""
echo "Git hooks installation completed!"
echo "The following hooks are now active:"
echo "  - pre-commit: Runs formatting, clippy, and tests before commit"
echo "  - commit-msg: Validates commit message format (Conventional Commits)"
echo "  - pre-push: Additional checks when pushing to main/master branch"
echo "  - post-checkout: Updates dependencies after branch checkout"