use std::fmt::Display;

#[derive(Debug)]
pub struct IHDRChunk {
    dimensions: (u32, u32),
    bit_depth: u8,
    color_type: u8,
    compression_method: u8,
    filter_method: u8,
    interlace_method: u8,
}

impl IHDRChunk {
    pub fn new(
        dimensions: (u32, u32),
        bit_depth: u8,
        color_type: u8,
        compression_method: u8,
        filter_method: u8,
        interlace_method: u8,
    ) -> IHDRChunk {
        IHDRChunk {
            dimensions,
            bit_depth,
            color_type,
            compression_method,
            filter_method,
            interlace_method,
        }
    }
    pub fn get_dimensions(&self) -> (u32, u32) {
        self.dimensions
    }

    pub fn get_bit_depth(&self) -> u8 {
        self.bit_depth
    }

    pub fn get_color_type(&self) -> u8 {
        self.color_type
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
            self.dimensions, self.bit_depth, self.color_type, self.compression_method, self.filter_method, self.interlace_method
        )
    }
}
