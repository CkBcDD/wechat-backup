# WeChat Backup Utility

A simple, fast command-line tool written in Rust to back up WeChat (PC version) image and video files. It automatically finds files from the current and previous months and archives them into organized ZIP files.

## Features

-   **Automatic Monthly Backup**: Creates separate `.zip` archives for each month's data.
-   **Smart Date Logic**: If run within the first 7 days of a month, it automatically backs up the previous month's data as well.
-   **Targeted Search**: Specifically looks for `Img` and `Vid` folders within WeChat's complex directory structure.
-   **Simple & Fast**: Built with Rust for performance and reliability, with a minimal command-line interface.
-   **Overwrite by Default**: Re-running the tool for the same month will overwrite the existing backup, ensuring it's always up-to-date.

## Building from Source

First, ensure you have the Rust toolchain installed on your system. You can get it from [rustup.rs](https://rustup.rs/).

1.  **Clone the repository or download the source code.**
2.  **Navigate to the project directory:**
    ```bash
    cd path\to\wechat-backup
    ```
3.  **Build the project in release mode:**
    ```bash
    cargo build --release
    ```
4.  The executable will be available at `target\release\wechat-backup.exe`.

## Usage

The tool requires two main arguments: a source directory (`--from`) and a destination directory (`--to`).

### Command Syntax

```bash
wechat-backup.exe --from <WECHAT_FILES_ROOT> --to <BACKUP_DESTINATION>
```

### Arguments

-   `--from <PATH>`: **(Required)** The root directory of your WeChat files. This is typically a folder named `xwechat_files` or similar, containing a subfolder with a long random name (e.g., `D:\Documents\xwechat_files\wxid_xxxxxxxxxxxxxx`).
-   `--to <PATH>`: **(Required)** The directory where the generated `.zip` backup files will be saved.
-   `-s`, `--silent`: (Optional) Run in silent mode. No output will be printed to the console, except for critical errors.
-   `-h`, `--help`: Display the help message with all available options.
-   `-V`, `--version`: Display the version information.

### Example

Let's say your WeChat files are stored in `D:\MyDocs\xwechat_files\wxid_abcdef123456` and you want to save backups to `E:\Backups\WeChat`.

You would run the following command:

```bash
.\target\release\wechat-backup.exe --from "D:\MyDocs\xwechat_files\wxid_abcdef123456" --to "E:\Backups\WeChat"
```

If today is **July 25, 2025**, the tool will:
1.  Look for image and video files from `2025-07`.
2.  Create a file named `2025-07_backup.zip` in `E:\Backups\WeChat`.

If today is **August 3, 2025** (within the first 7 days of the month), the tool will:
1.  Create `2025-07_backup.zip` for the previous month's data.
2.  Create `2025-08_backup.zip` for the current month's data.

## How It Works

The utility scans for files in the following specific paths within the `--from` directory:

-   **Images**: `...\msg\attach\<32_char_hash_dir>\YYYY-MM\Img\`
-   **Videos**: `...\msg\video\YYYY-MM\`

It then packages all files found in these locations into a ZIP archive with the following structure:

```
YYYY-MM_backup.zip
├── Img/
│   ├── file1.dat
│   └── file2.dat
└── Vid/
    ├── file3.dat
    └── file4.dat
```

**Note**: The tool backs up all files as-is, without attempting to decrypt them or change their file extensions.