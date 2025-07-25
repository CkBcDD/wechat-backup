mod backup;
mod cli;

use clap::Parser;
use cli::{Cli, LogLevel};
use log::{error, info};

#[tokio::main]
async fn main() {
    // 1. 解析命令行参数
    let args = Cli::parse();

    // 2. 初始化日志系统
    let log_level = if args.silent {
        log::LevelFilter::Off
    } else {
        match args.verbose {
            Some(LogLevel::W) => log::LevelFilter::Warn,
            Some(LogLevel::I) => log::LevelFilter::Info,
            Some(LogLevel::D) => log::LevelFilter::Debug,
            Some(LogLevel::T) => log::LevelFilter::Trace,
            None => log::LevelFilter::Error, // 默认级别
        }
    };

    env_logger::Builder::new()
        .filter_level(log_level)
        .format_timestamp(None) // 不显示时间戳
        .init();

    // 3. 验证输入和输出路径
    if !args.from.is_dir() {
        error!("源路径 {:?} 不是一个有效的目录。", args.from);
        std::process::exit(1);
    }
    if !args.to.is_dir() {
        error!("目标路径 {:?} 不是一个有效的目录。", args.to);
        std::process::exit(1);
    }

    // 4. 运行主备份逻辑
    info!("微信备份程序启动...");
    info!("源目录: {:?}", args.from);
    info!("目标目录: {:?}", args.to);

    if let Err(e) = backup::run(&args).await {
        error!("\n备份过程中发生错误: {}", e);
        std::process::exit(1);
    }

    info!("\n所有备份任务已成功完成！");
}
