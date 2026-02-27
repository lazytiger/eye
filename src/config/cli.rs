//! 命令行参数定义
//!
//! 使用 clap 定义所有命令行参数

use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// Eye - 个人智能助理
#[derive(Parser, Debug)]
#[command(name = "eye")]
#[command(version = "0.1.0")]
#[command(about = "个人智能助理 - 通过工具调用与现实世界交互", long_about = None)]
pub struct Cli {
    /// 配置文件路径
    #[arg(short, long, value_name = "FILE")]
    pub config_path: Option<PathBuf>,

    /// OpenRouter API Key
    #[arg(short, long, env = "OPENROUTER_API_KEY")]
    pub api_key: Option<String>,

    /// 模型名称
    #[arg(short, long, default_value = "openai/gpt-4o-mini")]
    pub model: String,

    /// 交互模式
    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// 子命令
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// 启动交互式会话
    Chat {
        /// 系统提示词
        #[arg(short, long)]
        system_prompt: Option<String>,
    },
    /// 执行单次查询
    Query {
        /// 查询内容
        query: String,
    },
    /// 列出可用工具
    ListTools,
    /// 列出可用技能
    ListSkills,
}

/// 解析命令行参数
pub fn parse_args() -> Cli {
    Cli::parse()
}
