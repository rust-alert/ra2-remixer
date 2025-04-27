use ra2_pal::Palette;
use ra2_types::{Ra2Error, Rgba, RgbaImage};

/// 帧头结构体
#[derive(Clone, Debug, Default)]
pub struct ShpFrame {
    /// The starting coordinate of the x-axis
    pub x: u16,
    /// The starting coordinate of the y-axis
    pub y: u16,
    /// The width of this frame
    pub width: u16,
    /// The height of this frame
    pub height: u16,
    /// 
    pub flags: u8, 
    /// unused
    pub reserved1: [u8; 3],
    /// unused
    pub color: u32,
    /// unused
    pub reserved2: [u8; 4], 
    /// Offset start
    pub offset: u32,
    /// The index buffer
    pub buffer: Vec<u8>,
}

impl ShpFrame {
    /// Render frame as rgba image buffer
    /// 
    /// # Arguments 
    /// 
    /// * `palette`: 
    /// * `width`: 
    /// * `depth`: 
    /// 
    /// returns: Result<ImageBuffer<Rgba<u8>, Vec<u8, Global>>, Ra2Error> 
    /// 
    /// # Examples 
    /// 
    /// ```
    /// 
    /// ```
    pub fn render(&self, palette: &Palette, width: u32, depth: u32) -> Result<RgbaImage, Ra2Error> {
        let mut image = RgbaImage::new(width, depth);
        let mut index = 0;
        for dy in 0..self.height {
            for dx in 0..self.width {
                let pixel = image.get_pixel_mut((self.x + dx) as u32, (self.y + dy) as u32);
                let color = self.buffer[index];
                if color == 0 {
                    *pixel = Rgba([0, 0, 0, 0]);
                }
                else {
                    *pixel = palette.get_color(color)?.into();
                }
                index += 1;
            }
        }
        Ok(image)
    }
}
