use std::fmt::Display;
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[repr(u8)]
#[allow(clippy::identity_op, clippy::eq_op)]
pub enum ColorLayout {
    Gray1 = 0 << 4 | 0,
    Gray2 = 0 << 4 | 1,
    Gray4 = 0 << 4 | 2,
    Gray8 = 0 << 4 | 3,
    Gray16 = 0 << 4 | 4,
    Rgb8 = 2 << 4 | 3,
    Rgb16 = 2 << 4 | 4,
    Plt1 = 3 << 4 | 0,
    Plt2 = 3 << 4 | 1,
    Plt4 = 3 << 4 | 2,
    Plt8 = 3 << 4 | 3,
    GrayAlpha8 = 4 << 4 | 3,
    GrayAlpha16 = 4 << 4 | 4,
    RgbAlpha8 = 6 << 4 | 3,
    RgbAlpha16 = 6 << 4 | 4,
}
impl ColorLayout {
    pub fn from_color_type_and_bit_depth(bit_depth: u8, color_type: u8) -> Option<ColorLayout> {
        match (bit_depth, color_type) {
            (1, 0) => Some(ColorLayout::Gray1),
            (2, 0) => Some(ColorLayout::Gray2),
            (4, 0) => Some(ColorLayout::Gray4),
            (8, 0) => Some(ColorLayout::Gray8),
            (16, 0) => Some(ColorLayout::Gray16),
            (8, 2) => Some(ColorLayout::Rgb8),
            (16, 2) => Some(ColorLayout::Rgb16),
            (1, 3) => Some(ColorLayout::Plt1),
            (2, 3) => Some(ColorLayout::Plt2),
            (4, 3) => Some(ColorLayout::Plt4),
            (8, 3) => Some(ColorLayout::Plt8),
            (8, 4) => Some(ColorLayout::GrayAlpha8),
            (16, 4) => Some(ColorLayout::GrayAlpha16),
            (8, 6) => Some(ColorLayout::RgbAlpha8),
            (16, 6) => Some(ColorLayout::RgbAlpha16),
            _ => panic!("Invalid color type and bit depth combination"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct IHDRChunk {
    dimensions: (u32, u32),
    color_layout: ColorLayout,
    compression_method: u8,
    filter_method: u8,
    interlace_method: u8,
}

impl IHDRChunk {
    pub fn new(
        dimensions: (u32, u32),
        color_layout: ColorLayout,
        compression_method: u8,
        filter_method: u8,
        interlace_method: u8,
    ) -> IHDRChunk {
        IHDRChunk {
            dimensions,
            color_layout,
            compression_method,
            filter_method,
            interlace_method,
        }
    }

    pub fn get_dimensions(&self) -> (u32, u32) {
        self.dimensions
    }

    pub fn get_bit_depth(&self) -> u8 {
        1 << (self.color_layout as u8 & 0b1111)
    }

    pub fn get_color_type(&self) -> u8 {
        self.color_layout as u8 >> 4
    }

    pub fn get_compression_method(&self) -> u8 {
        self.compression_method
    }

    pub fn get_filter_method(&self) -> u8 {
        self.filter_method
    }

    pub fn get_interlace_method(&self) -> u8 {
        self.interlace_method
    }
}

impl Display for IHDRChunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(dimensions: {:?}, bit_depth: {}, color_type: {}, compression_method: {}, filter_method: {}, interlace_method: {})",
            self.dimensions, self.get_bit_depth(), self.get_color_type(), self.compression_method, self.filter_method, self.interlace_method
        )
    }
}
