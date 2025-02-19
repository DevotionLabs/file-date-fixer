# 🖼️ File Date Fixer

**File Date Fixer** is a tool that allows you to reset the creation or modification date of images based on their filenames. This is useful when metadata timestamps are lost due to file transfers or storage operations.

> ⚠ **Note on Linux:** Linux does not allow modifying the creation date (`btime`) because the kernel lacks an API for it, even if some filesystems (e.g., `ext4`, `XFS`) store it. Unlike Windows, which supports modifying `btime` via `SetFileTime()`, Linux only allows changing `mtime` (modification time).

---

## 🔧 Features

- ✅ **Cross-platform:** Works on both **Windows** and **Linux**.
- 🏷 **Extracts Dates:** Detects dates in filenames using a standard pattern.
- 📅 **Modifies Metadata Dates:** Updates file creation date (Windows) or modification date (Linux).
- 📂 **Recursive Processing:** Optionally processes directories recursively.
- 📜 **Detailed Logging:** Provides logs for debugging and tracking changes.
- 🔍 **Supported Filename Formats:**  
  - ✅ `IMG_YYYYMMDD_<description>.<extension>` or `IMG-YYYYMMDD-<description>.<extension>`
  - ✅ `VID_YYYYMMDD_<description>.<extension>` or `VID-YYYYMMDD-<description>.<extension>`  
  - ✅ `PANO_YYYYMMDD_<description>.<extension>` or `PANO-YYYYMMDD-<description>.<extension>`

---

## 🚩 Available Flags

| Flag                    | Description                                      |
|-------------------------|--------------------------------------------------|
| `-d, --dir <DIRECTORY>` | Sets the **working directory** (Required).      |
| `-r, --recursive`       | Enables **recursive** directory processing.     |

---

## 🚀 Execution

### **🐧 Running on Linux**
1. **Download** the Linux binary from the [latest release](https://github.com/DevotionLabs/file-date-fixer/releases).
2. **Give execution permission:**
```shell!
chmod +x file_date_fixer-linux
```
3. **Run with the required flags:**
```shell!
./file_date_fixer-linux -d /path/to/images -r
```

### **🪟  Running on Windows**
1. **Download** the Windows binary from the [latest release](https://github.com/DevotionLabs/file-date-fixer/releases).
2. Run in the command prompt:
```shell!
file_date_fixer-windows.exe -d C:\path\to\images -r
```

## 🛠 Source
### 📦 Building from Source

Ensure you have Rust installed. If not, install it from [rustup.rs](https://rustup.rs/).
```shell!
git clone https://github.com/DevotionLabs/file-date-fixer.git
cd file-date-fixer
cargo build --release
```

The binary will be located in:

- Linux: `target/release/file_date_fixer`
- Windows: `target/release/file_date_fixer.exe`

### 🧪 Testing

Unit tests and integration tests can be run separately:
```shell!
cargo test-unit   # Runs only unit tests
cargo test-int    # Runs only integration tests
```

To run them all at the same time:
```shell!
cargo test
```

### ▶️ Running from Source

After building, execute the binary:
```shell!
cargo run -- -d /path/to/images -r
```
