use std::fmt;
use std::fmt::Display;

macro_rules! chunk_type_impl {
    ($($name:ident),*) => {
        #[derive(Debug, PartialEq, Eq, Copy, Clone)]
        #[allow(non_camel_case_types)]
        pub enum ChunkType {
           $($name,)*
        }

        impl ChunkType {
            pub fn as_slice(&self) -> &[u8] {
                match self {
                    $(ChunkType::$name => stringify!($name).as_bytes(),)*
                }
            }
        }

        impl Display for ChunkType {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", match self {
                    $(ChunkType::$name => stringify!($name),)*
                })
            }
        }

        impl From<&[u8]> for ChunkType {
            fn from(bytes: &[u8]) -> Self {
                 $(
                    if bytes == stringify!($name).as_bytes() {
                        return ChunkType::$name;
                    }
                )*
                panic!("Invalid ChunkType");
            }
        }
    };
}

chunk_type_impl!(
    IHDR, PLTE, IDAT, IEND, cHRM, gAMA, iCCP, sBIT, sRGB, bKGD, hIST, tRNS, pHYs, tIME, iTXt, tEXt,
    zTXt
);

#[derive(Debug, PartialEq, Eq)]
pub struct Chunk {
    length: usize,
    class: ChunkType,
    data: Box<[u8]>,
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
    pub fn new(length: usize, class: ChunkType, data: Box<[u8]>, crc: u32) -> Self {
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
