use std::fs::File;
use std::io::Read;

#[derive(Debug)]
struct Pixel {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

#[derive(Debug)]
struct Chunk {
    length: usize,
    chunk_type: u32,
    chunk_data: Vec<u8>,
    crc: u32,
}

impl Chunk {
    const LENGTH_SIZE_OFFSET: usize = 4;
    const TYPE_SIZE_OFFSET: usize = 8;
    const CRC_SIZE_OFFSET: usize = 12;

    pub fn parse_chunk(bytes: &[u8]) -> Vec<Chunk> {
        if bytes.len() < 16 {
            panic!("Bytes length is less than 16");
        }

        let mut data_chunks = vec![];
        let mut offset = 0;

        while offset < bytes.len() {
            let chunk_length = u32::from_be_bytes(
                bytes[offset..offset + Chunk::LENGTH_SIZE_OFFSET]
                    .try_into()
                    .unwrap_or_else(|error| {
                        panic!("Failed to convert chunk length bytes to u32: {}", error)
                    }),
            ) as usize;
            data_chunks.push(Chunk {
                length: chunk_length,
                chunk_type: u32::from_be_bytes(
                    bytes[offset + Chunk::LENGTH_SIZE_OFFSET..offset + Chunk::TYPE_SIZE_OFFSET]
                        .try_into()
                        .unwrap_or_else(|error| {
                            panic!("Failed to convert chunk type bytes to u32: {}", error)
                        }),
                ),
                chunk_data: bytes[offset + Chunk::TYPE_SIZE_OFFSET
                    ..offset + Chunk::TYPE_SIZE_OFFSET + chunk_length]
                    .to_vec(),
                crc: u32::from_be_bytes(
                    bytes[offset + Chunk::TYPE_SIZE_OFFSET + chunk_length
                        ..offset + Chunk::CRC_SIZE_OFFSET + chunk_length]
                        .try_into()
                        .unwrap_or_else(|error| {
                            panic!("Failed to convert chunk crc bytes to u32: {}", error)
                        }),
                ),
            });
            offset += Chunk::CRC_SIZE_OFFSET + chunk_length;
        }
        data_chunks
    }
}

struct Image {
    bytes: Vec<u8>,
}

impl Image {
    const PNG_SIGNATURE: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];

    pub fn new(path: &str) -> Image {
        let mut file = File::open(path).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        Image { bytes: buffer }
    }

    pub fn parse_png(&self) {
        if !self.is_png() {
            println!("not a png");
        }

        let data = &self.bytes[8..];
        let data_chunks = Chunk::parse_chunk(data);
        println!("{:?}", data_chunks);
    }

    pub fn is_png(&self) -> bool {
        self.bytes.starts_with(&Image::PNG_SIGNATURE)
    }
}

fn main() {
    let image = Image::new(r".\samples\sample.png");
    image.parse_png();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_png() {
        let image = Image::new(r".\samples\sample.png");
        assert!(image.is_png());
    }
}
