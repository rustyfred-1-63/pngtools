pub const MINIMUM_CHUNK_SIZE:usize = 12;
use std::fmt::{Display, Formatter};


use crc;
use crate::chunk_type::ChunkType;
use anyhow::{Context, ensure, Error};
use anyhow::Result;
use crc::{Crc, CRC_32_ISO_HDLC};
#[derive(Debug)]
pub struct Chunk{
  pub datalenght:u32,
  pub chunktype:ChunkType,
  pub chunkdata:Vec<u8>,
  pub crc:u32
}

impl Chunk {
  pub fn new(chunk_type:ChunkType,data:Vec<u8>)->Chunk{
    let datalenght = data.len() as u32;
    let crc = Chunk::compute_png_crc(&chunk_type,&data);
    Chunk{datalenght,chunkdata:data,chunktype:chunk_type,crc}

  }
  pub fn length(&self) ->u32{
    self.datalenght
  }
  pub fn chunk_type(&self)->&ChunkType{
    &self.chunktype
  }
  pub fn data(&self)->&[u8]{
    &self.chunkdata
  }
  pub fn crc(&self)->u32{
    self.crc
  }
  pub fn data_as_string(&self)->Result<String>{
    Ok(String::from_utf8(self.data().to_vec())?)
  }
  pub fn as_bytes(&self)->Vec<u8>{
    let mut bytes = self.length().to_be_bytes().to_vec();
    bytes.extend_from_slice(self.chunk_type().0.as_slice());
    bytes.extend_from_slice(self.chunkdata.as_slice());
    bytes.extend(self.crc().to_be_bytes());
    bytes
  }
  fn compute_png_crc(chunk_type:&ChunkType,data:&Vec<u8>)->u32{
    let mut hashing_bytes = chunk_type.0.to_vec();
    hashing_bytes.extend_from_slice(data.as_slice());
    let hasher = &Crc::<u32>::new(&CRC_32_ISO_HDLC);
    Crc::<u32>::checksum(hasher,hashing_bytes.as_slice())
  }
}
impl TryFrom<&[u8]> for Chunk{
  type Error = Error;
  fn try_from(value: &[u8]) -> Result<Self> {
    ensure!(value.len()>=MINIMUM_CHUNK_SIZE,"The supplied array is to short please provide a chunk is of a minimum of 12 bytes");
    let (length_bytes,remainder) = value.split_at(4);
    let datalenght = u32::from_be_bytes(length_bytes.try_into().context("Couldn't parse the 4 first bytes as a U32 number")?);
    let (chunktype_bytes,remainder)= remainder.split_at(4);
    let chunktype = ChunkType::try_from(chunktype_bytes)?;
    let (chunkdata_bytes,remainder) = remainder.split_at(datalenght as usize);
    let chunkdata = chunkdata_bytes.to_vec();
    ensure!(remainder.len() == 4,"Wrong size CRC");
    let crc = u32::from_be_bytes(remainder.try_into().context("Couldn't extract the last 4 bytes as a crc u32")?);
    ensure!(crc == Chunk::compute_png_crc(&chunktype,&chunkdata),"Incorrect check sum");

    Ok(Chunk{datalenght,chunktype,chunkdata,crc})
  }
}
impl Display for Chunk {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f,"length:{}\ndType:{}\n,Data{:?}\n,CRC:{}\n",self.datalenght,self.chunktype,self.chunkdata,self.crc)
  }
}


#[cfg(test)]
mod tests {
  use super::*;
  use crate::chunk_type::ChunkType;

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
