use crate::{chunk_type::ChunkType, errors::Errors};

pub struct Crc32 {
    table: [u32; 256],
}

impl Crc32 {
    /// Create a new CRC-32 calculator with pre-computed table
    pub fn new() -> Self {
        Crc32 {
            table: Self::make_crc_table(),
        }
    }

    /// Pre-compute the CRC table for byte-by-byte processing
    fn make_crc_table() -> [u32; 256] {
        let mut table = [0u32; 256];
        
        for n in 0..256 {
            let mut c = n as u32;
            for _ in 0..8 {
                if c & 1 != 0 {
                    c = 0xedb88320 ^ (c >> 1);
                } else {
                    c = c >> 1;
                }
            }
            table[n] = c;
        }
        
        table
    }

    /// Update a running CRC with new data
    pub fn update(&self, crc: u32, buf: &[u8]) -> u32 {
        let mut c = crc;
        
        for &byte in buf {
            c = self.table[(c as u8 ^ byte) as usize] ^ (c >> 8);
        }
        
        c
    }

    /// Calculate the CRC-32 of a buffer
    pub fn checksum(&self, buf: &[u8]) -> u32 {
        self.update(0xffffffff, buf) ^ 0xffffffff
    }
}

macro_rules! crc {
    ($buffer:expr) => {
        {
            let _crc = Crc32::new();
            _crc.checksum($buffer)
        }
    }
}

#[derive(Clone, Debug)]
pub struct Chunk {
    length: usize,
    chunk_type: ChunkType,
    data: Vec<u8>,
    crc: u32,    
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let buffer: Vec<u8> = chunk_type.bytes()
            .iter()
            .chain(data.iter())
            .copied()
            .collect();

        Chunk {
            length: data.len(),
            chunk_type: chunk_type,
            data: data,
            crc: crc!(&buffer),
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let bytes = (self.length as u32).to_be_bytes();
        bytes.iter()
            .chain(self.chunk_type.bytes().iter())
            .chain(self.data.iter())
            .chain(self.crc.to_be_bytes().iter())
            .copied()
            .collect()
    }

    pub fn crc(&self) -> u32 {
        self.crc
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn chunk_type(&self) -> ChunkType {
        self.chunk_type
    }

    pub fn data(&self) -> Vec<u8> {
        self.data.clone()
    }

    pub fn data_as_string(&self) -> Result<String, std::string::FromUtf8Error> {
        String::from_utf8(self.data.to_vec())
    }

}

impl TryFrom<&[u8]> for Chunk {
    type Error = Errors;
    
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {

        if value.len() < 12 {
            return Err(Errors::GenericError("Value is invalid".to_string()));
        }

        let crc_pos = value.len() - 4;
        let length = u32::from_be_bytes([value[0], value[1], value[2], value[3]]) as usize;
        let chunk_type = ChunkType::try_from([value[4], value[5], value[6], value[7]])?;
        let data = value[8..crc_pos].to_vec();
        let crc: u32 = u32::from_be_bytes([value[crc_pos], value[crc_pos+1], value[crc_pos+2], value[crc_pos+3]]);

        let buffer: Vec<u8> = chunk_type.bytes()
            .iter()
            .chain(data.iter())
            .copied()
            .collect();

        if crc!(&buffer) != crc {
            return Err(Errors::GenericError("Crc is invalid".to_string()));
        }

        Ok(
            Self { length: length, chunk_type: chunk_type, data: data, crc: crc }
        )

    }
}

impl std::fmt::Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Chunk {{",)?;
        writeln!(f, "  Length: {}", self.length())?;
        writeln!(f, "  Type: {}", self.chunk_type())?;
        writeln!(f, "  Data: {:?} bytes", self.data())?;
        writeln!(f, "  Crc: {}", self.crc())?;
        writeln!(f, "}}",)?;
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;
    
    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!".as_bytes().to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        
        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();
        
        let _chunk_string = format!("{}", chunk);
    }
    
}
