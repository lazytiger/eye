# Windows兼容性测试脚本

Write-Host "Testing git hooks compatibility on Windows..." -ForegroundColor Green
Write-Host "=============================================" -ForegroundColor Yellow

# 测试1: 检查git配置
Write-Host "`nTest 1: Checking git configuration..." -ForegroundColor Cyan
$hooksPath = git config --get core.hooksPath
if ($hooksPath -eq ".githook") {
    Write-Host "✓ Git hooks path correctly configured: $hooksPath" -ForegroundColor Green
} else {
    Write-Host "✗ Git hooks path not configured correctly: $hooksPath" -ForegroundColor Red
}

# 测试2: 检查文件权限
Write-Host "`nTest 2: Checking file permissions..." -ForegroundColor Cyan
$hookFiles = Get-ChildItem .githook\* -Exclude "*.md", "*.ps1", "*.sh"
$allExecutable = $true
foreach ($file in $hookFiles) {
    $acl = Get-Acl $file.FullName
    $access = $acl.Access | Where-Object { $_.IdentityReference -like "*$($env:USERNAME)*" }
    if ($access.FileSystemRights -match "ExecuteFile|ReadAndExecute") {
        Write-Host "✓ $($file.Name) has execute permission" -ForegroundColor Green
    } else {
        Write-Host "✗ $($file.Name) missing execute permission" -ForegroundColor Red
        $allExecutable = $false
    }
}

# 测试3: 检查脚本语法
Write-Host "`nTest 3: Checking script syntax..." -ForegroundColor Cyan

# 测试bash脚本语法
if (Get-Command bash -ErrorAction SilentlyContinue) {
    Write-Host "Testing bash scripts with Git Bash..." -ForegroundColor Yellow
    
    $bashScripts = @("pre-commit", "commit-msg", "pre-push", "post-checkout")
    foreach ($script in $bashScripts) {
        $result = bash -n ".githook/$script" 2>&1
        if ($LASTEXITCODE -eq 0) {
            Write-Host "✓ $script has valid bash syntax" -ForegroundColor Green
        } else {
            Write-Host "✗ $script has bash syntax errors: $result" -ForegroundColor Red
        }
    }
} else {
    Write-Host "Warning: bash not found. Skipping bash syntax check." -ForegroundColor Yellow
}

# 测试4: 检查shebang兼容性
Write-Host "`nTest 4: Checking shebang compatibility..." -ForegroundColor Cyan
$hookFiles = Get-ChildItem .githook\* -Exclude "*.md", "*.ps1", "*.sh"
foreach ($file in $hookFiles) {
    $firstLine = Get-Content $file.FullName -First 1
    if ($firstLine -eq "#!/bin/bash") {
        Write-Host "✓ $($file.Name) has compatible shebang (#!/bin/bash)" -ForegroundColor Green
    } else {
        Write-Host "✗ $($file.Name) has unexpected shebang: $firstLine" -ForegroundColor Red
    }
}

# 测试5: 检查Windows Git版本
Write-Host "`nTest 5: Checking Windows Git version..." -ForegroundColor Cyan
$gitVersion = git --version
Write-Host "Git version: $gitVersion" -ForegroundColor Yellow
if ($gitVersion -match "windows") {
    Write-Host "✓ Running Windows Git" -ForegroundColor Green
} else {
    Write-Host "⚠ Not running Windows Git" -ForegroundColor Yellow
}

# 测试6: 检查关键命令可用性
Write-Host "`nTest 6: Checking command availability..." -ForegroundColor Cyan
$requiredCommands = @("git", "cargo")
foreach ($cmd in $requiredCommands) {
    if (Get-Command $cmd -ErrorAction SilentlyContinue) {
        Write-Host "✓ $cmd is available" -ForegroundColor Green
    } else {
        Write-Host "✗ $cmd is not available" -ForegroundColor Red
    }
}

# 总结
Write-Host "`n=============================================" -ForegroundColor Yellow
Write-Host "Windows Compatibility Test Summary" -ForegroundColor Green
Write-Host "=============================================" -ForegroundColor Yellow

if ($hooksPath -eq ".githook" -and $allExecutable) {
    Write-Host "✅ Git hooks are properly configured for Windows!" -ForegroundColor Green
    Write-Host "`nNext steps:" -ForegroundColor Cyan
    Write-Host "1. Make a test commit to verify pre-commit hook works" -ForegroundColor White
    Write-Host "2. Try pushing to test pre-push hook (use --no-verify to skip if needed)" -ForegroundColor White
    Write-Host "3. Switch branches to test post-checkout hook" -ForegroundColor White
} else {
    Write-Host "⚠ Some issues detected. Please run the install script:" -ForegroundColor Yellow
    Write-Host "   .\\.githook\\install.ps1" -ForegroundColor White
}

Write-Host "`nNote: Git hooks run in Git Bash environment on Windows." -ForegroundColor Cyan
Write-Host "      Make sure Git Bash is installed and in your PATH." -ForegroundColor Cyan