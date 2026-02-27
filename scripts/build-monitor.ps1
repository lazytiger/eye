#!/usr/bin/env pwsh
<#
.SYNOPSIS
监控GitHub构建结果并采取相应措施

.DESCRIPTION
此脚本查询最近24小时的GitHub Actions工作流运行状态，并根据结果采取相应措施：
1. 检测失败的构建并创建issue
2. 更新README中的构建状态
3. 发送通知（如果配置了webhook）

.PARAMETER Repository
GitHub仓库，格式为"owner/repo"，默认为当前仓库

.PARAMETER GitHubToken
GitHub个人访问令牌，需要有repo权限

.PARAMETER DryRun
干运行模式，只查询不执行任何操作

.EXAMPLE
.\build-monitor.ps1 -Repository "lazytiger/eye" -GitHubToken "ghp_xxx"

.EXAMPLE
.\build-monitor.ps1 -DryRun
#>

param(
    [string]$Repository,
    [string]$GitHubToken,
    [switch]$DryRun
)

# 设置错误处理
$ErrorActionPreference = "Stop"

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

# 如果没有提供token，尝试从环境变量中获取
if (-not $GitHubToken) {
    $GitHubToken = $env:GITHUB_TOKEN
    if (-not $GitHubToken) {
        Write-Host "警告: 未提供GitHub Token，某些操作可能受限" -ForegroundColor Yellow
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

Write-Host "开始监控构建结果..." -ForegroundColor Cyan
Write-Host "仓库: $Repository" -ForegroundColor Cyan
Write-Host "时间: $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss')" -ForegroundColor Cyan
Write-Host ""

# 定义要监控的工作流
$workflows = @{
    'ci.yml' = 'CI'
    'release.yml' = 'Release'
    'dependabot.yml' = 'Dependabot'
}

# 查询工作流运行
function Get-WorkflowRuns {
    param(
        [string]$Owner,
        [string]$Repo,
        [string]$Token,
        [int]$Hours = 24
    )
    
    $since = (Get-Date).AddHours(-$Hours).ToString("yyyy-MM-ddTHH:mm:ssZ")
    $headers = @{
        "Accept" = "application/vnd.github.v3+json"
    }
    
    if ($Token) {
        $headers["Authorization"] = "Bearer $Token"
    }
    
    $url = "https://api.github.com/repos/$Owner/$Repo/actions/runs?per_page=50&created=>=${since}"
    
    try {
        Write-Host "查询工作流运行..." -ForegroundColor Gray
        $response = Invoke-RestMethod -Uri $url -Headers $headers -Method Get
        return $response.workflow_runs
    } catch {
        Write-Host "查询工作流运行失败: $_" -ForegroundColor Red
        return @()
    }
}

# 分析工作流运行结果
function Analyze-WorkflowRuns {
    param(
        [array]$Runs,
        [hashtable]$WorkflowMap
    )
    
    $results = @{}
    $hasFailures = $false
    $hasInProgress = $false
    
    # 按工作流分组
    $groupedRuns = @{}
    foreach ($run in $Runs) {
        $workflowFile = $run.path -split '/' | Select-Object -Last 1
        if ($WorkflowMap.ContainsKey($workflowFile)) {
            if (-not $groupedRuns.ContainsKey($workflowFile)) {
                $groupedRuns[$workflowFile] = @()
            }
            $groupedRuns[$workflowFile] += $run
        }
    }
    
    # 分析每个工作流
    foreach ($workflowFile in $groupedRuns.Keys) {
        $runs = $groupedRuns[$workflowFile] | Sort-Object -Property created_at -Descending
        $latestRun = $runs[0]
        
        $failedRuns = $runs | Where-Object { $_.conclusion -eq 'failure' }
        $inProgressRuns = $runs | Where-Object { $_.status -ne 'completed' }
        
        $results[$workflowFile] = @{
            Name = $WorkflowMap[$workflowFile]
            LatestStatus = $latestRun.status
            LatestConclusion = $latestRun.conclusion
            LatestUrl = $latestRun.html_url
            TotalRuns = $runs.Count
            FailedRuns = $failedRuns.Count
            InProgressRuns = $inProgressRuns.Count
            LatestRunId = $latestRun.id
            CreatedAt = $latestRun.created_at
        }
        
        if ($latestRun.conclusion -eq 'failure') {
            $hasFailures = $true
        }
        if ($latestRun.status -ne 'completed') {
            $hasInProgress = $true
        }
    }
    
    return @{
        Results = $results
        HasFailures = $hasFailures
        HasInProgress = $hasInProgress
    }
}

# 创建issue
function Create-IssueForFailures {
    param(
        [string]$Owner,
        [string]$Repo,
        [string]$Token,
        [hashtable]$Results
    )
    
    if (-not $Token) {
        Write-Host "未提供GitHub Token，跳过创建issue" -ForegroundColor Yellow
        return
    }
    
    $headers = @{
        "Accept" = "application/vnd.github.v3+json"
        "Authorization" = "Bearer $Token"
    }
    
    # 构建issue内容
    $issueBody = "# 🚨 Build Failures Detected`n`n"
    $issueBody += "The following workflows have failed in the last 24 hours:`n`n"
    
    foreach ($workflowFile in $Results.Keys) {
        $data = $Results[$workflowFile]
        if ($data.LatestConclusion -eq 'failure') {
            $issueBody += "## $($data.Name)`n"
            $issueBody += "- **Status**: $($data.LatestConclusion)`n"
            $issueBody += "- **Failed Runs**: $($data.FailedRuns)`n"
            $issueBody += "- **Latest Run**: [View Details]($($data.LatestUrl))`n`n"
        }
    }
    
    $issueBody += "---`n"
    $issueBody += "*This issue was automatically generated by the Build Monitor script.*`n"
    
    # 检查是否已存在相同issue
    $issuesUrl = "https://api.github.com/repos/$Owner/$Repo/issues?state=open&labels=build-failure,automated"
    
    try {
        $existingIssues = Invoke-RestMethod -Uri $issuesUrl -Headers $headers -Method Get
        
        $recentIssue = $existingIssues | Where-Object {
            $_.title -like "*Build Failures Detected*" -and
            ([datetime]$_.created_at) -gt (Get-Date).AddHours(-24)
        }
        
        if ($recentIssue) {
            Write-Host "已存在相同issue: $($recentIssue.html_url)" -ForegroundColor Yellow
            return
        }
        
        # 创建新issue
        $createIssueUrl = "https://api.github.com/repos/$Owner/$Repo/issues"
        $issueData = @{
            title = "🚨 Build Failures Detected"
            body = $issueBody
            labels = @("build-failure", "automated", "ci")
        } | ConvertTo-Json
        
        $response = Invoke-RestMethod -Uri $createIssueUrl -Headers $headers -Method Post -Body $issueData
        Write-Host "已创建issue: $($response.html_url)" -ForegroundColor Green
        
    } catch {
        Write-Host "创建issue失败: $_" -ForegroundColor Red
    }
}

# 更新README
function Update-ReadmeWithStatus {
    param(
        [string]$RepoPath,
        [hashtable]$Results
    )
    
    $readmePath = Join-Path $RepoPath "README.md"
    
    if (-not (Test-Path $readmePath)) {
        Write-Host "README.md不存在，跳过更新" -ForegroundColor Yellow
        return
    }
    
    try {
        $readmeContent = Get-Content $readmePath -Raw
        
        # 移除现有的构建状态部分
        $readmeContent = $readmeContent -replace '## 📊 Build Status[\s\S]*?(?=## |$)', ''
        
        # 构建新的状态表格
        $statusTable = "## 📊 Build Status`n`n"
        $statusTable += "Last updated: $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss')`n`n"
        $statusTable += "| Workflow | Status | Latest Run |`n"
        $statusTable += "|----------|--------|------------|`n"
        
        foreach ($workflowFile in $Results.Keys) {
            $data = $Results[$workflowFile]
            
            $statusEmoji = if ($data.LatestConclusion -eq 'success') { '✅' }
                          elseif ($data.LatestConclusion -eq 'failure') { '❌' }
                          elseif ($data.LatestStatus -eq 'in_progress') { '⏳' }
                          else { '❓' }
            
            $statusText = if ($data.LatestConclusion) { $data.LatestConclusion } else { $data.LatestStatus }
            $statusTable += "| $($data.Name) | $statusEmoji $statusText | [View]($($data.LatestUrl)) |`n"
        }
        
        $statusTable += "`n> *This section is automatically updated by the Build Monitor script.*`n"
        
        # 插入到README中
        $firstHeaderIndex = $readmeContent.IndexOf('##')
        if ($firstHeaderIndex -ne -1) {
            $before = $readmeContent.Substring(0, $firstHeaderIndex)
            $after = $readmeContent.Substring($firstHeaderIndex)
            $readmeContent = $before + $statusTable + "`n`n" + $after
        } else {
            $readmeContent = $statusTable + "`n`n" + $readmeContent
        }
        
        Set-Content -Path $readmePath -Value $readmeContent -NoNewline
        Write-Host "已更新README.md" -ForegroundColor Green
        
    } catch {
        Write-Host "更新README失败: $_" -ForegroundColor Red
    }
}

# 主执行逻辑
try {
    # 1. 查询工作流运行
    $runs = Get-WorkflowRuns -Owner $owner -Repo $repo -Token $GitHubToken -Hours 24
    
    if ($runs.Count -eq 0) {
        Write-Host "最近24小时内没有工作流运行" -ForegroundColor Yellow
        exit 0
    }
    
    # 2. 分析结果
    $analysis = Analyze-WorkflowRuns -Runs $runs -WorkflowMap $workflows
    $results = $analysis.Results
    
    # 3. 显示摘要
    Write-Host "📊 构建监控摘要" -ForegroundColor Cyan
    Write-Host "=======================" -ForegroundColor Cyan
    
    $totalRuns = 0
    $failedRuns = 0
    $successRuns = 0
    $inProgressRuns = 0
    
    foreach ($workflowFile in $results.Keys) {
        $data = $results[$workflowFile]
        
        Write-Host "`n$($data.Name):" -ForegroundColor White
        Write-Host "  状态: $($data.LatestConclusion ?? $data.LatestStatus)" -ForegroundColor Gray
        Write-Host "  总运行次数: $($data.TotalRuns)" -ForegroundColor Gray
        Write-Host "  失败: $($data.FailedRuns)" -ForegroundColor Gray
        Write-Host "  进行中: $($data.InProgressRuns)" -ForegroundColor Gray
        Write-Host "  最新运行: $($data.LatestUrl)" -ForegroundColor Gray
        
        $totalRuns += $data.TotalRuns
        $failedRuns += $data.FailedRuns
        $inProgressRuns += $data.InProgressRuns
        $successRuns += $data.TotalRuns - $data.FailedRuns - $data.InProgressRuns
    }
    
    Write-Host "`n📈 总体统计:" -ForegroundColor Cyan
    Write-Host "  总运行次数: $totalRuns" -ForegroundColor White
    Write-Host "  成功: $successRuns" -ForegroundColor Green
    Write-Host "  失败: $failedRuns" -ForegroundColor $(if ($failedRuns -gt 0) { 'Red' } else { 'White' })
    Write-Host "  进行中: $inProgressRuns" -ForegroundColor $(if ($inProgressRuns -gt 0) { 'Yellow' } else { 'White' })
    
    # 4. 根据结果采取行动
    if (-not $DryRun) {
        if ($analysis.HasFailures) {
            Write-Host "`n❌ 检测到构建失败，正在创建issue..." -ForegroundColor Red
            Create-IssueForFailures -Owner $owner -Repo $repo -Token $GitHubToken -Results $results
        } elseif ($analysis.HasInProgress) {
            Write-Host "`n⏳ 有构建正在进行中" -ForegroundColor Yellow
        } else {
            Write-Host "`n✅ 所有构建都已完成且成功！" -ForegroundColor Green
        }
        
        # 更新README
        Write-Host "`n📝 更新README..." -ForegroundColor Cyan
        Update-ReadmeWithStatus -RepoPath (Get-Location) -Results $results
        
    } else {
        Write-Host "`n🏃 干运行模式，未执行任何操作" -ForegroundColor Yellow
    }
    
    # 根据是否有失败设置退出码
    if ($analysis.HasFailures) {
        exit 1
    } else {
        exit 0
    }
    
} catch {
    Write-Host "`n❌ 脚本执行失败: $_" -ForegroundColor Red
    exit 1
}