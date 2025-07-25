use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use zip::write::{FileOptions, ZipWriter};

/// 将指定目录下的所有文件添加到 ZIP 压缩包中
///
/// # 参数
/// * `zip` - 可变的 ZipWriter 实例
/// * `source_dir` - 要添加文件的源目录路径
/// * `zip_folder` - 在 ZIP 包内创建的子文件夹名称
/// * `options` - ZIP 文件选项
pub fn add_files_to_zip(
    zip: &mut ZipWriter<File>,
    source_dir: &Path,
    zip_folder: &str,
    options: FileOptions<'_, ()>,
) -> Result<(), std::io::Error> {
    // 遍历源目录中的条目
    for entry in std::fs::read_dir(source_dir)? {
        let entry = entry?;
        let path = entry.path();

        // 确保我们只处理文件
        if path.is_file() {
            let file_name = path.file_name().unwrap_or_else(|| path.as_os_str());

            // 在ZIP包中创建路径，例如 "Img/some_file.dat"
            let zip_path = Path::new(zip_folder).join(file_name);

            // 在ZIP中开始一个新文件
            zip.start_file(zip_path.to_str().unwrap(), options)?;

            // 打开源文件并将其内容写入ZIP
            let mut f = File::open(&path)?;
            let mut buffer = Vec::new();
            f.read_to_end(&mut buffer)?;
            zip.write_all(&buffer)?;
        }
    }
    Ok(())
}
