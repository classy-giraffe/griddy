use std::fs::File;
use std::io::Read;
use crate::chunk::Chunk;

pub(crate) struct Image {
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

    pub fn get_bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn parse_png(&self) {
        if !self.is_png() {
            println!("not a png");
        }
        let data_chunks = Chunk::parse_chunk(&self.bytes[8..]);
        todo!();
    }

    pub fn is_png(&self) -> bool {
        self.bytes.starts_with(&Image::PNG_SIGNATURE)
    }
}