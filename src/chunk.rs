#[derive(Debug)]
pub struct Chunk {
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