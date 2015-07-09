# RA2-Mix 库

这是一个用于读取和写入红色警戒2（Red Alert 2）MIX文件格式的Rust库。MIX文件是Westwood Studios游戏中使用的资源归档格式。

## 功能特性

- 读取MIX文件并提取其中的内容
- 创建新的MIX文件
- 支持加密和未加密的MIX文件格式
- 支持文件名到ID的映射
- 提供简单易用的API

## 使用示例

### 读取MIX文件

```rust
use ra2_mix::read;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 从文件读取
    let file_map = read(Some(Path::new("example.mix")), None)?;
    
    // 打印文件列表
    for (filename, data) in &file_map {
        println!("文件: {}, 大小: {} 字节", filename, data.len());
    }
    
    Ok(())
}
```

### 创建MIX文件

```rust
use ra2_mix::{write, XCCGame};
use std::collections::HashMap;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建文件映射
    let mut file_map = HashMap::new();
    file_map.insert("test.txt".to_string(), b"Hello, World!".to_vec());
    file_map.insert("data.bin".to_string(), vec![0, 1, 2, 3, 4, 5]);
    
    // 写入MIX文件
    write(
        Some(Path::new("output.mix")),
        XCCGame::RA2,
        Some(file_map),
        None::<&Path>,
        None,
    )?;
    
    println!("MIX文件已创建!");
    Ok(())
}
```

### 提取MIX文件到目录

```rust
use ra2_mix::extract;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 提取MIX文件到目录
    extract(
        Path::new("example.mix"),
        Path::new("extracted_files"),
    )?;
    
    println!("文件已提取到 'extracted_files' 目录");
    Ok(())
}
```

## 安装

将以下内容添加到你的`Cargo.toml`文件中：

```toml
[dependencies]
ra2-mix = "0.1.0"
```

## 功能标志

- `serde-support` - 启用对全局MIX数据库的序列化/反序列化支持

## 许可证

此库使用MPL-2.0许可证。
