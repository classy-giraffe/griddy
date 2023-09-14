use std::fmt;

#[derive(Debug)]
pub struct Chunk {
    pub length: usize,
    pub class: u32,
    pub data: Vec<u8>,
    pub crc: u32,
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Chunk: length: {}, type: {}, data: {:?}, crc: {}",
            self.length, self.class, self.data, self.crc
        )
    }
}

impl Chunk {
    pub fn new(length: usize, class: u32, data: Vec<u8>, crc: u32) -> Chunk {
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
        self.class
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    pub fn get_crc(&self) -> u32 {
        self.crc
    }
}