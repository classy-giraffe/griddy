use crate::prelude::*;
use crc32fast::Hasher;
use std::fmt;
use std::fmt::Display;
use ParsingError as pe;

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

            pub fn as_str(&self) -> String {
                match self {
                    $(ChunkType::$name => stringify!($name).to_string(),)*
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
    IHDR, PLTE, IDAT, IEND, tRNS, cHRM, gAMA, iCCP, sBIT, sRGB, cICP, mDCv, iTXt, tEXt, zTXt, bKGD,
    hIST, pHYs, sPLT, eXIF, tIME, acTL, fcTL, fdAT
);

#[derive(Debug, PartialEq, Eq, Clone)]

pub struct Chunk {
    length: usize,
    class: ChunkType,
    data: Box<[u8]>,
    crc: u32,
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

    pub fn parse(bytes: &[u8], mut crc32: Hasher) -> Result<Self, ParsingError> {
        let length = u32::from_be_bytes(
            bytes[..LENGTH_SIZE_OFFSET]
                .try_into()
                .map_err(|_| pe::InvalidLength)?,
        ) as usize;

        let class = ChunkType::from(&bytes[LENGTH_SIZE_OFFSET..TYPE_SIZE_OFFSET]);

        let data = Box::from(&bytes[TYPE_SIZE_OFFSET..TYPE_SIZE_OFFSET + length]);

        let crc = u32::from_be_bytes(
            bytes[TYPE_SIZE_OFFSET + length..CRC_SIZE_OFFSET + length]
                .try_into()
                .map_err(|_| pe::InvalidCrc)?,
        );

        crc32.update(class.as_slice());
        crc32.update(&data);
        let finalized_crc = crc32.finalize();

        if crc != finalized_crc {
            return Err(pe::CrcMismatch);
        }

        Ok(Self::new(length, class, data, crc))
    }

    pub fn get_length(&self) -> usize {
        self.length
    }

    pub fn get_class(&self) -> ChunkType {
        self.class
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    pub fn get_crc(&self) -> u32 {
        self.crc
    }
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
