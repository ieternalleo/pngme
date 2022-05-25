use std::{io::Read, str::FromStr};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct ChunkType {
    data: u32,
}

impl ChunkType {
    pub fn bytes(self) -> [u8; 4] {
        self.data.to_be_bytes()
    }

    pub fn is_critical(self) -> bool {
        self.data
            .to_be_bytes()
            .first()
            .unwrap()
            .is_ascii_uppercase()
    }

    pub fn is_public(self) -> bool {
        self.data
            .to_be_bytes()
            .iter()
            .skip(1)
            .next()
            .unwrap()
            .is_ascii_uppercase()
    }

    pub fn is_reserved_bit_valid(self) -> bool {
        self.data
            .to_be_bytes()
            .iter()
            .skip(2)
            .next()
            .unwrap()
            .is_ascii_uppercase()
    }
    pub fn is_safe_to_copy(self) -> bool {
        self.data
            .to_be_bytes()
            .iter()
            .skip(3)
            .next()
            .unwrap()
            .is_ascii_uppercase()
    }
    // pub fn is_public(self) -> bool {}
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = &'static str;
    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        let data = u32::from_be_bytes(value);
        let res = ChunkType { data };
        Ok(res)
    }
}

impl FromStr for ChunkType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.as_bytes();
        let byte_array = <[u8; 4]>::try_from(bytes).unwrap();
        Ok(ChunkType {
            data: u32::from_be_bytes(byte_array),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use std::convert::TryFrom;
    // use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    
    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    
    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    
    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }


    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    
    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    
    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    // #[ignore]
    // #[test]
    // pub fn test_valid_chunk_is_valid() {
    //     let chunk = ChunkType::from_str("RuSt").unwrap();
    //     assert!(chunk.is_valid());
    // }

    // #[ignore]
    // #[test]
    // pub fn test_invalid_chunk_is_valid() {
    //     let chunk = ChunkType::from_str("Rust").unwrap();
    //     assert!(!chunk.is_valid());

    //     let chunk = ChunkType::from_str("Ru1t");
    //     assert!(chunk.is_err());
    // }

    // #[ignore]
    // #[test]
    // pub fn test_chunk_type_string() {
    //     let chunk = ChunkType::from_str("RuSt").unwrap();
    //     assert_eq!(&chunk.to_string(), "RuSt");
    // }

    // #[ignore]
    // #[test]
    // pub fn test_chunk_type_trait_impls() {
    //     let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
    //     let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
    //     let _chunk_string = format!("{}", chunk_type_1);
    //     let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    // }
}
