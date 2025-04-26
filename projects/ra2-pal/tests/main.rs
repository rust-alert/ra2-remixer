use ra2_pal::{convert_6bit_to_8bit, Palette};

#[test]
fn ready() {
    println!("it works!")
}



fn main() {
    // 示例用法：创建一个包含纯红色调色板的 PAL 文件
    let mut pal_data = [0u8; 256 * 3];
    for i in 0..256 {
        pal_data[i * 3] = 63; // 纯红色 (63 是 6 位颜色值的最大值)
        pal_data[i * 3 + 1] = 0;
        pal_data[i * 3 + 2] = 0;
    }

    let pal_file_result = Palette::from_bytes(&pal_data);

    match pal_file_result {
        Ok(pal_file) => {
            println!("PAL 文件创建成功: {:?}", pal_file);

            // 获取第一个颜色 (索引 0)
            if let Some(first_color) = pal_file.get_color(0) {
                println!("第一个颜色 (RGB): {:?}", first_color);
                println!(
                    "8 位 RGB: {},{},{}",
                    convert_6bit_to_8bit(first_color.red),
                    convert_6bit_to_8bit(first_color.green),
                    convert_6bit_to_8bit(first_color.blue)
                );
            }
            else {
                println!("索引 0 超出范围");
            }
        }
        Err(err) => {
            println!("创建 PAL 文件时出错: {}", err);
        }
    }
}
