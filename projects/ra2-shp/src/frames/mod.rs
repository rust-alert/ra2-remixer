use ra2_pal::Palette;
use ra2_types::{Ra2Error, Rgba, RgbaImage};

// 帧头结构体
#[derive(Clone, Debug, Default)]
pub struct ShpFrame {
    pub x: u16,             // 水平位置 (0,0)
    pub y: u16,             // 垂直位置 (0,0)
    pub width: u16,         // 帧宽度
    pub height: u16,        // 帧高度
    pub flags: u8,          // 特殊标志
    pub reserved1: [u8; 3], // 对齐 (3 字节)
    pub color: u32,         // 颜色 (可以是透明色)
    pub reserved2: [u8; 4], // 保留字2 (未使用)
    pub offset: u32,        // 帧数据在文件中的偏移量
    pub buffer: Vec<u8>,
}

impl ShpFrame {
    pub fn render(&self, palette: Palette) -> Result<RgbaImage, Ra2Error> {
        let mut image = RgbaImage::new(self.width as u32, self.height as u32);
        for (x, y, pixel) in image.enumerate_pixels_mut() {
            let index = x + y * self.width as u32;
            let color = self.buffer[index as usize];
            if color == 0 {
                *pixel = Rgba([0, 0, 0, 0]);
            }
            else {
                let rgb565 = palette.get_color(color)?;
                *pixel = rgb565.into();
            }
        }
        Ok(image)
    }
}
