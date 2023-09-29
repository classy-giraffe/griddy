use crate::prelude::*;
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
    pub fn from_bit_depth_and_color_type(bit_depth: u8, color_type: u8) -> Self {
        match (bit_depth, color_type) {
            (1, 0) => ColorLayout::Gray1,
            (2, 0) => ColorLayout::Gray2,
            (4, 0) => ColorLayout::Gray4,
            (8, 0) => ColorLayout::Gray8,
            (16, 0) => ColorLayout::Gray16,
            (8, 2) => ColorLayout::Rgb8,
            (16, 2) => ColorLayout::Rgb16,
            (1, 3) => ColorLayout::Plt1,
            (2, 3) => ColorLayout::Plt2,
            (4, 3) => ColorLayout::Plt4,
            (8, 3) => ColorLayout::Plt8,
            (8, 4) => ColorLayout::GrayAlpha8,
            (16, 4) => ColorLayout::GrayAlpha16,
            (8, 6) => ColorLayout::RgbAlpha8,
            (16, 6) => ColorLayout::RgbAlpha16,
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

    pub fn from_chunk(chunk: &Chunk) -> Result<IHDRChunk, ParsingError> {
        use ParsingError as pe;
        let data = chunk.get_data();
        Ok(IHDRChunk {
            dimensions: (
                u32::from_be_bytes(data[..4].try_into().map_err(|_| pe::InvalidData)?),
                u32::from_be_bytes(data[4..8].try_into().map_err(|_| pe::InvalidData)?),
            ),
            color_layout: ColorLayout::from_bit_depth_and_color_type(data[8], data[9]),
            compression_method: data[10],
            filter_method: data[11],
            interlace_method: data[12],
        })
    }
}

impl Display for IHDRChunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Dimensions: {}x{}", self.dimensions.0, self.dimensions.1)?;
        writeln!(f, "Color layout: {:?}", self.color_layout)?;
        writeln!(f, "Compression method: {}", self.compression_method)?;
        writeln!(f, "Filter method: {}", self.filter_method)?;
        writeln!(f, "Interlace method: {}", self.interlace_method)?;
        Ok(())
    }
}
