use crate::chunk_type::ChunkType;

use std::convert::TryFrom;
use std::error;
use std::fmt;
use std::fmt::Display;
use std::io::Read;

use crate::{Error, Result};
use crc::{Crc, CRC_32_ISO_HDLC};

#[derive(Debug)]
struct CRCMismatch;

impl error::Error for CRCMismatch {}
impl Display for CRCMismatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CRC mismatch")
    }
}
struct Chunk {
    chunk_type: ChunkType,
    data: Vec<u8>,
    crc: u32,
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        pub const PNG_CRC: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);
        let chunk_data_bytes = chunk_type
            .bytes()
            .iter()
            .chain(data.iter())
            .copied()
            .collect::<Vec<u8>>();
        Chunk {
            chunk_type,
            data,
            crc: PNG_CRC.checksum(&chunk_data_bytes),
        }
    }
    pub fn length(&self) -> u32 {
        self.data.len() as u32
    }
    fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }
    fn data(&self) -> &[u8] {
        &self.data
    }
    fn crc(&self) -> u32 {
        self.crc.clone()
    }
    fn data_as_string(&self) -> Result<String> {
        Ok(String::from_utf8(self.data.clone())?)
    }
    fn as_bytes(&self) -> Vec<u8> {
        let chunk_data = self
            .length()
            .to_be_bytes()
            .iter()
            .chain(self.chunk_type.bytes().iter())
            .chain(self.data().iter())
            .chain(self.crc().to_be_bytes().iter())
            .copied()
            .collect();

        chunk_data
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;
    fn try_from(value: &[u8]) -> Result<Self> {
        let mut chunk_lbuffer = [0u8; 4];

        let _ = (&value[0..4]).read(&mut chunk_lbuffer[..]);
        let chunk_length: u32 = u32::from_be_bytes(chunk_lbuffer);
        // let ct_array: [u8; 4] = value[4..8].try_into().unwrap();
        // let chunk_type: ChunkType = ChunkType::try_from(ct_array).unwrap();
        // let chunk_data: Vec<u8> = value[8..(length_bytes) as usize].to_owned();

        let mut chunk_type_buf = [0u8; 4];
        let _ = (&value[4..8]).read(&mut chunk_type_buf[..])?;
        let chunk_type = ChunkType::try_from(chunk_type_buf)?;

        let data_start: usize = 8;
        let data_end: usize = (8 + chunk_length) as usize;
        let chunk_data: Vec<_> = value[data_start..data_end].to_owned();

        let mut chunk_crc_buff = [0u8; 4];
        let _ = (&value[data_end..data_end + 4]).read(&mut chunk_crc_buff);
        let chunk_crc: u32 = u32::from_be_bytes(chunk_crc_buff);
        let chunk = Chunk::new(chunk_type, chunk_data);
        if chunk.crc() == chunk_crc {
            Ok(chunk)
        } else {
            Err(Box::new(CRCMismatch))
        }

        // let chunk_length =
        //     u32::from_be_bytes(value[0..4].clone_from_slice(src).try_into().expect("Entered Chunk is too short"));
        // let chunk_type: ChunkType = ChunkType::try_from(value[4..8].try_into().unwrap()).unwrap();
    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Chunk {{",)?;
        writeln!(f, "    Length: {}", self.length())?;
        writeln!(f, "    Type: {}", self.chunk_type())?;
        writeln!(f, "    Data: {} bytes", self.data().len())?;
        writeln!(f, "    CRC: {}", self.crc())?;
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
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
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
