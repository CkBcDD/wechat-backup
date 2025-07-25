use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum LogLevel {
    /// (W)arn: 只显示警告和错误
    W,
    /// (I)nfo: 显示信息、警告和错误
    I,
    /// (D)ebug: 显示调试信息及以上
    D,
    /// (T)race: 显示所有跟踪信息
    T,
}

/// 一个简单的微信文件备份工具
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// 需要备份的微信文件根目录 (例如: D:\\xwechat_files\\xxxxx)
    #[arg(long)]
    pub from: PathBuf,

    /// 备份文件存放的目标目录 (例如: D:\\backup)
    #[arg(long)]
    pub to: PathBuf,

    /// 静默模式，程序将不会在控制台输出任何信息
    #[arg(short, long, default_value_t = false, conflicts_with = "verbose")]
    pub silent: bool,

    /// 设置日志输出的详细级别 [可选值: W, I, D, T]
    #[arg(long, value_name = "LEVEL")]
    pub verbose: Option<LogLevel>,
}
