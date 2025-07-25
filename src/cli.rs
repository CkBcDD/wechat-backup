use clap::Parser;
use std::path::PathBuf;

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
    #[arg(short, long, default_value_t = false)]
    pub silent: bool,

    /// 输出详细日志 (暂未实现)
    #[arg(long, value_name = "LEVEL")]
    pub verbose: Option<u8>,
}
