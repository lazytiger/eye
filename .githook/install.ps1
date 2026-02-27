# PowerShell script for installing git hooks on Windows

Write-Host "Installing git hooks..." -ForegroundColor Green

# 检查是否在git仓库中
if (-not (Test-Path ".git")) {
    Write-Host "Error: This is not a git repository." -ForegroundColor Red
    exit 1
}

# 设置执行权限
Write-Host "Setting execute permissions..." -ForegroundColor Yellow
Get-ChildItem .githook\* | ForEach-Object {
    icacls $_.FullName /grant:r "$($env:USERNAME):(RX)"
}

# 配置git使用.githook目录作为hook路径
git config core.hooksPath .githook

# 验证配置
$hooksPath = git config --get core.hooksPath
if ($hooksPath -eq ".githook") {
    Write-Host "Successfully configured git hooks path to .githook" -ForegroundColor Green
    Write-Host "Available hooks:" -ForegroundColor Yellow
    Get-ChildItem .githook\
} else {
    Write-Host "Error: Failed to configure git hooks path." -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "Git hooks installation completed!" -ForegroundColor Green
Write-Host "The following hooks are now active:" -ForegroundColor Yellow
Write-Host "  - pre-commit: Runs formatting, clippy, and tests before commit"
Write-Host "  - commit-msg: Validates commit message format (Conventional Commits)"
Write-Host "  - pre-push: Additional checks when pushing to main/master branch"
Write-Host "  - post-checkout: Updates dependencies after branch checkout"
Write-Host ""
Write-Host "To run the bash install script instead, use: bash .githook/install.sh" -ForegroundColor Cyan