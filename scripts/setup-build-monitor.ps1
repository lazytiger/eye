#!/usr/bin/env pwsh
<#
.SYNOPSIS
设置构建监控定时任务

.DESCRIPTION
此脚本帮助设置构建监控的定时任务，包括：
1. 创建GitHub Token（如果需要）
2. 设置环境变量
3. 配置Windows定时任务或cron job
4. 测试监控脚本

.PARAMETER InstallType
安装类型：'windows'（Windows定时任务）或 'cron'（Linux/macOS cron job）

.PARAMETER GitHubToken
GitHub个人访问令牌，如果未提供则会提示输入

.PARAMETER Repository
GitHub仓库，格式为"owner/repo"，默认为当前仓库

.PARAMETER Schedule
定时任务计划，默认为"每天午夜12点"
对于Windows：默认为"Daily at 12:00 AM"
对于cron：默认为"0 0 * * *"

.EXAMPLE
.\setup-build-monitor.ps1 -InstallType windows

.EXAMPLE
.\setup-build-monitor.ps1 -InstallType cron -GitHubToken "ghp_xxx" -Repository "lazytiger/eye"
#>

param(
    [ValidateSet('windows', 'cron')]
    [string]$InstallType = 'windows',
    
    [string]$GitHubToken,
    [string]$Repository,
    [string]$Schedule
)

# 设置错误处理
$ErrorActionPreference = "Stop"

Write-Host "🚀 开始设置构建监控定时任务" -ForegroundColor Cyan
Write-Host "======================================" -ForegroundColor Cyan

# 如果没有提供仓库，尝试从git配置中获取
if (-not $Repository) {
    try {
        $remoteUrl = git config --get remote.origin.url
        if ($remoteUrl -match 'github\.com[:/]([^/]+)/([^/.]+)') {
            $Repository = "$($matches[1])/$($matches[2])"
            Write-Host "检测到仓库: $Repository" -ForegroundColor Green
        } else {
            throw "无法从git配置中检测到GitHub仓库"
        }
    } catch {
        Write-Host "错误: $_" -ForegroundColor Red
        Write-Host "请使用 -Repository 参数指定仓库，格式为 'owner/repo'" -ForegroundColor Yellow
        exit 1
    }
}

# 如果没有提供token，提示用户输入
if (-not $GitHubToken) {
    $GitHubToken = $env:GITHUB_TOKEN
    if (-not $GitHubToken) {
        Write-Host "`n🔑 GitHub Token 配置" -ForegroundColor Yellow
        Write-Host "构建监控需要GitHub Token来访问API和创建issue。" -ForegroundColor Gray
        Write-Host "请创建具有以下权限的Token：" -ForegroundColor Gray
        Write-Host "  - repo (完全控制私有仓库)" -ForegroundColor Gray
        Write-Host "  - workflow (读取工作流)" -ForegroundColor Gray
        Write-Host "创建地址: https://github.com/settings/tokens/new" -ForegroundColor Gray
        
        $GitHubToken = Read-Host -Prompt "请输入GitHub Token" -AsSecureString
        $GitHubToken = [System.Runtime.InteropServices.Marshal]::PtrToStringAuto(
            [System.Runtime.InteropServices.Marshal]::SecureStringToBSTR($GitHubToken)
        )
        
        if (-not $GitHubToken) {
            Write-Host "错误: 必须提供GitHub Token" -ForegroundColor Red
            exit 1
        }
    }
}

# 设置默认计划
if (-not $Schedule) {
    switch ($InstallType) {
        'windows' { $Schedule = 'Daily at 12:00 AM' }
        'cron' { $Schedule = '0 0 * * *' }
    }
}

# 解析仓库信息
$repoParts = $Repository -split '/'
if ($repoParts.Count -ne 2) {
    Write-Host "错误: 仓库格式不正确，应为 'owner/repo'" -ForegroundColor Red
    exit 1
}

$owner = $repoParts[0]
$repo = $repoParts[1]

Write-Host "`n📋 配置信息:" -ForegroundColor Cyan
Write-Host "  仓库: $Repository" -ForegroundColor White
Write-Host "  安装类型: $InstallType" -ForegroundColor White
Write-Host "  计划: $Schedule" -ForegroundColor White
Write-Host ""

# 测试GitHub Token
function Test-GitHubToken {
    param(
        [string]$Token,
        [string]$Owner,
        [string]$Repo
    )
    
    Write-Host "测试GitHub Token..." -ForegroundColor Gray
    
    $headers = @{
        "Accept" = "application/vnd.github.v3+json"
        "Authorization" = "Bearer $Token"
    }
    
    $url = "https://api.github.com/repos/$Owner/$Repo"
    
    try {
        $response = Invoke-RestMethod -Uri $url -Headers $headers -Method Get
        Write-Host "✅ Token测试成功" -ForegroundColor Green
        Write-Host "   仓库: $($response.full_name)" -ForegroundColor Gray
        Write-Host "   权限: $($response.permissions.admin ? '管理员' : $response.permissions.push ? '写入' : '读取')" -ForegroundColor Gray
        return $true
    } catch {
        Write-Host "❌ Token测试失败: $_" -ForegroundColor Red
        return $false
    }
}

# 测试监控脚本
function Test-MonitorScript {
    param(
        [string]$Token,
        [string]$Repository
    )
    
    Write-Host "`n测试监控脚本..." -ForegroundColor Gray
    
    $scriptPath = Join-Path $PSScriptRoot "build-monitor.ps1"
    if (-not (Test-Path $scriptPath)) {
        Write-Host "❌ 监控脚本不存在: $scriptPath" -ForegroundColor Red
        return $false
    }
    
    try {
        Write-Host "运行测试（干运行模式）..." -ForegroundColor Gray
        & $scriptPath -Repository $Repository -GitHubToken $Token -DryRun
        $exitCode = $LASTEXITCODE
        
        if ($exitCode -eq 0) {
            Write-Host "✅ 监控脚本测试成功" -ForegroundColor Green
            return $true
        } else {
            Write-Host "❌ 监控脚本测试失败，退出码: $exitCode" -ForegroundColor Red
            return $false
        }
    } catch {
        Write-Host "❌ 监控脚本测试失败: $_" -ForegroundColor Red
        return $false
    }
}

# 创建环境变量配置文件
function Create-EnvFile {
    param(
        [string]$Token,
        [string]$Repository
    )
    
    $envFile = Join-Path $PSScriptRoot ".env"
    
    $envContent = @"
# GitHub构建监控配置
GITHUB_TOKEN=$Token
GITHUB_REPOSITORY=$Repository
"@
    
    try {
        Set-Content -Path $envFile -Value $envContent
        Write-Host "✅ 创建环境变量文件: $envFile" -ForegroundColor Green
        
        # 设置文件权限（仅限所有者可读）
        if (Test-Path $envFile) {
            icacls $envFile /inheritance:r /grant:r "$env:USERNAME:(R)"
        }
        
        return $true
    } catch {
        Write-Host "❌ 创建环境变量文件失败: $_" -ForegroundColor Red
        return $false
    }
}

# 设置Windows定时任务
function Setup-WindowsTask {
    param(
        [string]$ScriptPath,
        [string]$Schedule,
        [string]$Repository
    )
    
    Write-Host "`n设置Windows定时任务..." -ForegroundColor Cyan
    
    $taskName = "GitHubBuildMonitor_$($Repository.Replace('/', '_'))"
    $taskDescription = "每天监控GitHub构建结果 - $Repository"
    
    # 创建任务触发器
    $trigger = New-ScheduledTaskTrigger -Daily -At "12:00AM"
    
    # 创建任务操作
    $action = New-ScheduledTaskAction `
        -Execute "PowerShell.exe" `
        -Argument "-NoProfile -ExecutionPolicy Bypass -File `"$ScriptPath`""
    
    # 创建任务主体
    $principal = New-ScheduledTaskPrincipal `
        -UserId "$env:USERDOMAIN\$env:USERNAME" `
        -LogonType S4U `
        -RunLevel Highest
    
    # 创建任务设置
    $settings = New-ScheduledTaskSettingsSet `
        -AllowStartIfOnBatteries `
        -DontStopIfGoingOnBatteries `
        -StartWhenAvailable `
        -WakeToRun
    
    # 注册任务
    try {
        # 检查任务是否已存在
        $existingTask = Get-ScheduledTask -TaskName $taskName -ErrorAction SilentlyContinue
        if ($existingTask) {
            Write-Host "任务已存在，更新中..." -ForegroundColor Yellow
            Unregister-ScheduledTask -TaskName $taskName -Confirm:$false
        }
        
        Register-ScheduledTask `
            -TaskName $taskName `
            -Trigger $trigger `
            -Action $action `
            -Principal $principal `
            -Settings $settings `
            -Description $taskDescription
        
        Write-Host "✅ Windows定时任务设置成功" -ForegroundColor Green
        Write-Host "   任务名称: $taskName" -ForegroundColor Gray
        Write-Host "   计划: $Schedule" -ForegroundColor Gray
        Write-Host "   下次运行: $(Get-ScheduledTask -TaskName $taskName).NextRunTime" -ForegroundColor Gray
        
        return $true
    } catch {
        Write-Host "❌ Windows定时任务设置失败: $_" -ForegroundColor Red
        return $false
    }
}

# 设置cron job
function Setup-CronJob {
    param(
        [string]$ScriptPath,
        [string]$Schedule,
        [string]$Repository
    )
    
    Write-Host "`n设置cron job..." -ForegroundColor Cyan
    
    $cronLine = "$Schedule cd `"$PSScriptRoot`" && pwsh -File `"$ScriptPath`" >> `"$PSScriptRoot/build-monitor.log`" 2>&1"
    $cronComment = "# GitHub构建监控 - $Repository"
    
    try {
        # 获取当前crontab
        $currentCrontab = crontab -l 2>$null
        if ($LASTEXITCODE -ne 0) {
            $currentCrontab = ""
        }
        
        # 移除现有的相同任务
        $lines = $currentCrontab -split "`n" | Where-Object {
            $_ -notlike "*GitHub构建监控 - $Repository*"
        }
        
        # 添加新任务
        $newCrontab = ($lines + $cronComment + $cronLine) -join "`n"
        
        # 写入新crontab
        $newCrontab | crontab -
        
        if ($LASTEXITCODE -eq 0) {
            Write-Host "✅ cron job设置成功" -ForegroundColor Green
            Write-Host "   计划: $Schedule" -ForegroundColor Gray
            Write-Host "   日志文件: $PSScriptRoot/build-monitor.log" -ForegroundColor Gray
            
            # 显示当前crontab
            Write-Host "`n当前crontab:" -ForegroundColor Gray
            crontab -l | Select-String -Pattern "GitHub构建监控"
            
            return $true
        } else {
            Write-Host "❌ cron job设置失败" -ForegroundColor Red
            return $false
        }
    } catch {
        Write-Host "❌ cron job设置失败: $_" -ForegroundColor Red
        return $false
    }
}

# 创建使用说明
function Create-Readme {
    param(
        [string]$InstallType,
        [string]$Repository,
        [string]$ScriptPath
    )
    
    $readmePath = Join-Path $PSScriptRoot "README-monitor.md"
    
    $readmeContent = @"
# GitHub构建监控

此目录包含GitHub构建监控的脚本和配置。

## 配置信息
- **仓库**: $Repository
- **安装类型**: $InstallType
- **监控脚本**: $(Split-Path $ScriptPath -Leaf)
- **设置脚本**: $(Split-Path $PSCommandPath -Leaf)

## 使用方法

### 手动运行监控
```powershell
.\build-monitor.ps1 -Repository "$Repository"
```

### 带Token运行
```powershell
.\build-monitor.ps1 -Repository "$Repository" -GitHubToken "your_token_here"
```

### 干运行模式（只查询不执行操作）
```powershell
.\build-monitor.ps1 -Repository "$Repository" -DryRun
```

## 定时任务

### 查看任务状态
$(if ($InstallType -eq 'windows') {
"```powershell
# 查看任务状态
Get-ScheduledTask -TaskName "GitHubBuildMonitor_$($Repository.Replace('/', '_'))"

# 立即运行任务
Start-ScheduledTask -TaskName "GitHubBuildMonitor_$($Repository.Replace('/', '_'))"

# 禁用任务
Disable-ScheduledTask -TaskName "GitHubBuildMonitor_$($Repository.Replace('/', '_'))"

# 删除任务
Unregister-ScheduledTask -TaskName "GitHubBuildMonitor_$($Repository.Replace('/', '_'))" -Confirm:`$false
```"
} else {
"```bash
# 查看crontab
crontab -l

# 编辑crontab
crontab -e
```"
})

## 环境变量

环境变量存储在 \`.env\` 文件中：
- \`GITHUB_TOKEN\`: GitHub个人访问令牌
- \`GITHUB_REPOSITORY\`: GitHub仓库（格式：owner/repo）

## 日志

监控脚本的输出会记录到以下位置：
$(if ($InstallType -eq 'windows') {
"- Windows事件查看器：应用程序和服务日志 -> Microsoft -> Windows -> PowerShell -> Operational"
} else {
"- 日志文件：\`build-monitor.log\`"
})

## 故障排除

1. **Token权限不足**
   - 确保Token具有 \`repo\` 和 \`workflow\` 权限
   - 重新生成Token并更新 \`.env\` 文件

2. **脚本执行失败**
   - 检查PowerShell执行策略：\`Get-ExecutionPolicy\`
   - 如果需要，设置为RemoteSigned：\`Set-ExecutionPolicy RemoteSigned -Scope CurrentUser\`

3. **定时任务不运行**
   - 检查任务状态
   - 查看系统日志
   - 手动运行脚本测试

## 支持的响应

监控脚本检测到构建失败时会：
1. 在GitHub仓库中创建issue
2. 更新README.md中的构建状态
3. 输出详细的构建状态摘要

## 自定义

要修改监控的工作流，编辑 \`build-monitor.ps1\` 文件中的 \``\$workflows`` 哈希表。
"@
    
    try {
        Set-Content -Path $readmePath -Value $readmeContent
        Write-Host "✅ 创建使用说明: $readmePath" -ForegroundColor Green
        return $true
    } catch {
        Write-Host "❌ 创建使用说明失败: $_" -ForegroundColor Red
        return $false
    }
}

# 主执行逻辑
try {
    Write-Host "`n🔧 开始设置过程..." -ForegroundColor Cyan
    
    # 1. 测试GitHub Token
    if (-not (Test-GitHubToken -Token $GitHubToken -Owner $owner -Repo $repo)) {
        exit 1
    }
    
    # 2. 测试监控脚本
    $scriptPath = Join-Path $PSScriptRoot "build-monitor.ps1"
    if (-not (Test-MonitorScript -Token $GitHubToken -Repository $Repository)) {
        exit 1
    }
    
    # 3. 创建环境变量文件
    if (-not (Create-EnvFile -Token $GitHubToken -Repository $Repository)) {
        exit 1
    }
    
    # 4. 设置定时任务
    $setupSuccess = $false
    switch ($InstallType) {
        'windows' {
            $setupSuccess = Setup-WindowsTask -ScriptPath $scriptPath -Schedule $Schedule -Repository $Repository
        }
        'cron' {
            $setupSuccess = Setup-CronJob -ScriptPath $scriptPath -Schedule $Schedule -Repository $Repository
        }
    }
    
    if (-not $setupSuccess) {
        exit 1
    }
    
    # 5. 创建使用说明
    Create-Readme -InstallType $InstallType -Repository $Repository -ScriptPath $scriptPath
    
    Write-Host "`n🎉 设置完成！" -ForegroundColor Green
    Write-Host "======================================" -ForegroundColor Green
    
    Write-Host "`n📋 下一步：" -ForegroundColor Cyan
    Write-Host "1. 定时任务将在 $Schedule 自动运行" -ForegroundColor White
    Write-Host "2. 你可以手动运行测试: .\build-monitor.ps1 -DryRun" -ForegroundColor White
    Write-Host "3. 查看使用说明: scripts\README-monitor.md" -ForegroundColor White
    
    if ($InstallType -eq 'windows') {
        Write-Host "`n🔍 立即测试：" -ForegroundColor Yellow
        Write-Host "Start-ScheduledTask -TaskName `"GitHubBuildMonitor_$($Repository.Replace('/', '_'))`"" -ForegroundColor Gray
    }
    
} catch {
    Write-Host "`n❌ 设置过程失败: $_" -ForegroundColor Red
    exit 1
}