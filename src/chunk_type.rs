use std::fmt::{Display, Formatter};
use std::ops::RangeInclusive;
use std::str::{ FromStr};
use anyhow::{ensure, Error, Result};
use anyhow::Context;

const FIFTH_BIT_ONLY:u8 = 0b00100000;
const LOWER_CASE: RangeInclusive<u8> = 65..=90;
const UPPER_CASE: RangeInclusive<u8> = 97..=122;

#[derive(PartialEq,Eq,Debug)]
pub struct ChunkType(pub(crate) [u8;4]);

impl ChunkType{
  fn bytes(&self)->[u8;4]{ self.0 }
  fn is_valid(&self)->bool{ self.0.iter().all(|x| LOWER_CASE.contains(x) || UPPER_CASE.contains(x)) && self.is_reserved_bit_valid() }
  fn is_critical(&self)->bool{ (self.0[0] & FIFTH_BIT_ONLY) != FIFTH_BIT_ONLY }
  fn is_public(&self)->bool{
    (self.0[1]& FIFTH_BIT_ONLY) != FIFTH_BIT_ONLY
  }
  fn is_reserved_bit_valid(&self)->bool{
    (self.0[2]& FIFTH_BIT_ONLY) != FIFTH_BIT_ONLY
  }
  fn is_safe_to_copy(&self)->bool{
    (self.0[3] & FIFTH_BIT_ONLY) == FIFTH_BIT_ONLY
  }
  pub fn from_str(s: &str) -> Result<Self> {
    ensure!(s.len()==4,"Str of a length different than 4,got provided a length of {}",s.len());
    let new_chunk_type = ChunkType(s.as_bytes().try_into().context("Couldn't convert from str to ChunkType")?);
    ensure!(s.bytes().all(|x| LOWER_CASE.contains(&x) || UPPER_CASE.contains(&x)),"Got wrong bytes");
    Ok(new_chunk_type)
  }
}

impl TryFrom<[u8;4]> for ChunkType {
  type Error = Error;
  fn try_from(value: [u8; 4]) -> Result<Self> {
    let new_chunk_type = ChunkType(value);
    ensure!(new_chunk_type.is_valid(),"Got provided an incorrect 4 bytes array as chunk slice");

    Ok(new_chunk_type)
  }
}
impl TryFrom<&[u8]> for ChunkType{
  type Error = Error;
  fn try_from(value: &[u8]) -> Result<Self> {
    let new_chunk_type = ChunkType(value.try_into().context("Invalid slice")?);
    ensure!(new_chunk_type.is_valid(),"Got provided an incorrect 4 bytes array");

    Ok(new_chunk_type)

  }
}

impl FromStr for ChunkType {
  type Err = Error;
  fn from_str(s: &str) -> Result<Self> {
    ChunkType::from_str(s)
  }
}

impl Display for ChunkType {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f,"{}{}{}{}",self.0[0] as char,self.0[1] as char,self.0[2] as char,self.0[3] as char)
  }
}


#[cfg(test)]
mod tests {
  use super::*;
  use std::convert::TryFrom;
  use std::str::FromStr;

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

  #[test]
  pub fn test_valid_chunk_is_valid() {
    let chunk = ChunkType::from_str("RuSt").unwrap();
    assert!(chunk.is_valid());
  }

  #[test]
  pub fn test_invalid_chunk_is_valid() {
    let chunk = ChunkType::from_str("Rust").unwrap();
    assert!(!chunk.is_valid());

    let chunk = ChunkType::from_str("Ru1t");
    assert!(chunk.is_err());
  }

  #[test]
  pub fn test_chunk_type_string() {
    let chunk = ChunkType::from_str("RuSt").unwrap();
    assert_eq!(&chunk.to_string(), "RuSt");
  }

  #[test]
  pub fn test_chunk_type_trait_impls() {
    let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
    let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
    let _chunk_string = format!("{}", chunk_type_1);
    let _are_chunks_equal = chunk_type_1 == chunk_type_2;
  }
}