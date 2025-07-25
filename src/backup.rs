use crate::cli::Cli;
use crate::utils;
use chrono::{Datelike, Local, Months, NaiveDate};
use log::{debug, info, warn};
use std::fs::File;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use zip::write::{FileOptions, ZipWriter};

/// 备份流程的主执行函数
pub fn run(args: &Cli) -> Result<(), Box<dyn std::error::Error>> {
    let now = Local::now();
    let mut months_to_backup = vec![];

    // 1. 根据规则，判断需要备份的月份
    let current_month_str = now.format("%Y-%m").to_string();
    months_to_backup.push(current_month_str);

    if now.day() <= 7 {
        // 使用 NaiveDate 进行月份计算，避免时区问题
        let first_day_of_current_month =
            NaiveDate::from_ymd_opt(now.year(), now.month(), 1).unwrap();
        let last_month_date = first_day_of_current_month - Months::new(1);
        let last_month_str = last_month_date.format("%Y-%m").to_string();

        info!("检测到当前为月初，将同时备份上个月: {}", last_month_str);
        months_to_backup.push(last_month_str);
    }

    // 2. 为每个需要备份的月份执行具体流程
    for month_str in months_to_backup {
        info!("开始处理 {} 月的备份...", month_str);
        if let Err(e) = process_backup_for_month(&args.from, &args.to, &month_str) {
            // 在迭代中处理错误，而不是让整个程序失败
            warn!(
                "处理 {} 月份时发生错误: {}。将继续处理下一个月份。",
                month_str, e
            );
        }
    }

    Ok(())
}

/// 处理单个指定月份的备份流程
fn process_backup_for_month(
    from_dir: &Path,
    to_dir: &Path,
    month: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // 3. 查找源文件目录
    let img_sources = find_img_sources(from_dir, month);
    let vid_source = from_dir.join("msg").join("video").join(month);

    // 如果两个来源都不存在，则跳过此月份
    if img_sources.is_empty() && !vid_source.exists() {
        info!("在 {} 月未找到图片或视频文件，跳过。", month);
        return Ok(());
    }

    // 4. 创建 ZIP 文件并准备写入
    let zip_path = to_dir.join(format!("{}_backup.zip", month));
    let zip_file = File::create(&zip_path)?;
    let mut zip = ZipWriter::new(zip_file);
    let options = FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    // 5. 将找到的文件添加到 ZIP 包
    // 添加图片文件
    for img_dir in &img_sources {
        utils::add_files_to_zip(&mut zip, img_dir, "Img", options)?;
    }
    // 添加视频文件
    if vid_source.exists() {
        debug!("找到 Vid 源: {:?}", vid_source);
        utils::add_files_to_zip(&mut zip, &vid_source, "Vid", options)?;
    }

    zip.finish()?;
    info!("成功创建备份文件: {:?}", zip_path);
    Ok(())
}

/// 在 `.../msg/attach/` 目录下查找所有符合月份条件的 Img 源目录
fn find_img_sources(from_dir: &Path, month: &str) -> Vec<PathBuf> {
    let mut sources = Vec::new();
    let attach_dir = from_dir.join("msg").join("attach");
    if !attach_dir.is_dir() {
        return sources;
    }

    // 遍历 attach 目录的第一层子目录 (即 32 位哈希值的目录)
    for entry in WalkDir::new(attach_dir).min_depth(1).max_depth(1) {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };
        let path = entry.path();
        if path.is_dir() {
            // 拼接并检查目标 Img 目录是否存在
            let img_path = path.join(month).join("Img");
            if img_path.is_dir() {
                debug!("找到 Img 源: {:?}", img_path);
                sources.push(img_path);
            }
        }
    }
    sources
}
