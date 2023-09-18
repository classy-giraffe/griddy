use std::fmt;
use std::fmt::Display;

#[repr(u32)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ChunkType {
    Ihdr = 1229472850,
    Iccp = 1766015824,
    Phys = 1883789683,
    Idat = 1229209940,
    Iend = 1229278788,
    Time = 1950960965,
}

impl ChunkType {
    pub fn as_slice(&self) -> &[u8] {
        match self {
            ChunkType::Ihdr => &[73, 72, 68, 82],
            ChunkType::Iccp => &[105, 67, 67, 80],
            ChunkType::Phys => &[112, 72, 89, 115],
            ChunkType::Idat => &[73, 68, 65, 84],
            ChunkType::Iend => &[73, 69, 78, 68],
            ChunkType::Time => &[116, 73, 77, 69],
        }
    }
}
impl From<u32> for ChunkType {
    fn from(class: u32) -> Self {
        match class {
            1229472850 => ChunkType::Ihdr,
            1766015824 => ChunkType::Iccp,
            1883789683 => ChunkType::Phys,
            1229209940 => ChunkType::Idat,
            1229278788 => ChunkType::Iend,
            1950960965 => ChunkType::Time,
            _ => panic!("Invalid chunk type"),
        }
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ChunkType::Ihdr => "IHDR",
                ChunkType::Iccp => "ICCP",
                ChunkType::Phys => "PHYS",
                ChunkType::Idat => "IDAT",
                ChunkType::Iend => "IEND",
                ChunkType::Time => "TIME",
            }
        )
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Chunk {
    length: usize,
    class: ChunkType,
    data: Vec<u8>,
    crc: u32,
}

impl Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Chunk: length: {}, type: {}, crc: {}",
            self.length, self.class, self.crc
        )
    }
}

impl Chunk {
    pub fn new(length: usize, class: ChunkType, data: Vec<u8>, crc: u32) -> Chunk {
        Chunk {
            length,
            class,
            data,
            crc,
        }
    }

    pub fn get_length(&self) -> usize {
        self.length
    }

    pub fn get_class(&self) -> u32 {
        self.class as u32
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    pub fn get_crc(&self) -> u32 {
        self.crc
    }
}
