use std::path::{Path};
use ra2_mix::{extract, XccPackage};

#[test]
fn test_write_and_read() {
    // 创建一个临时目录用于测试
    // let temp_dir = tempfile::tempdir().unwrap();
    let temp_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    
    let mix_path = temp_dir.join("tests/test.mix");
    
    println!("Test MIX file path: {:?}", mix_path);
    
    // 创建测试文件数据
    let mut file_map = XccPackage::default();
    file_map.add_any("test1.txt".to_string(), b"Hello, World!".to_vec());
    file_map.add_any("test2.bin".to_string(), vec![0, 1, 2, 3, 4, 5]);
    
    file_map.save(&mix_path).unwrap();
    
    // 确保文件已创建
    assert!(mix_path.exists());
    
    // 读取MIX文件
    let read_file_map = XccPackage::load(&mix_path).unwrap().files;
    
    // 验证文件内容
    assert!(read_file_map.contains_key("test1.txt"));
    assert!(read_file_map.contains_key("test2.bin"));
    assert_eq!(read_file_map.get("test1.txt").unwrap(), b"Hello, World!");
    assert_eq!(read_file_map.get("test2.bin").unwrap(), &vec![0, 1, 2, 3, 4, 5]);
    
    // 测试提取功能
    let extract_dir = temp_dir.join("tests/extracted");
    extract(&mix_path, &extract_dir).unwrap();
    
    // 验证文件已提取
    assert!(extract_dir.join("test1.txt").exists());
    assert!(extract_dir.join("test2.bin").exists());
    
    // 验证提取的文件内容
    let content1 = std::fs::read(extract_dir.join("test1.txt")).unwrap();
    let content2 = std::fs::read(extract_dir.join("test2.bin")).unwrap();
    assert_eq!(content1, b"Hello, World!");
    assert_eq!(content2, vec![0, 1, 2, 3, 4, 5]);
}