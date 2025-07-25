mod backup;
mod cli;
mod utils;

use clap::Parser;
use cli::Cli;

fn main() {
    // 1. 解析命令行参数
    let args = Cli::parse();

    // 2. 验证输入和输出路径
    if !args.from.is_dir() {
        eprintln!("错误: 源路径 {:?} 不是一个有效的目录。", args.from);
        std::process::exit(1);
    }
    if !args.to.is_dir() {
        eprintln!("错误: 目标路径 {:?} 不是一个有效的目录。", args.to);
        std::process::exit(1);
    }

    // 3. 运行主备份逻辑
    if !args.silent {
        println!("微信备份程序启动...");
        println!("源目录: {:?}", args.from);
        println!("目标目录: {:?}", args.to);
    }

    if let Err(e) = backup::run(&args) {
        // 如果是静默模式，则不输出错误信息，仅以错误码退出
        if !args.silent {
            eprintln!("\n备份过程中发生错误: {}", e);
        }
        std::process::exit(1);
    }

    if !args.silent {
        println!("\n所有备份任务已成功完成！");
    }
}
