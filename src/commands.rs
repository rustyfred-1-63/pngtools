use std::fs;
use std::io::Write;
use std::path::PathBuf;
use anyhow::{Context, Result};

use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;

pub fn encode(filepath:&PathBuf, chunktype:&str, message:&str, outfile:&Option<PathBuf>) ->Result<()>{
  let new_chunk = chunk_from_arg(chunktype,message)?;
  return if outfile.is_none() {
    let mut output = fs::OpenOptions::new().append(true).open(filepath).context("Cant access the input file")?;
    output.write(&new_chunk.as_bytes()).context("Cant append to the existing file")?;
    Ok(())
  } else {
    let mut new_png = read_to_png(filepath)?;
    new_png.append_chunk(new_chunk);
    fs::write(outfile.as_ref().unwrap(), new_png.as_bytes()).context("Cant write to the output file")?;
    Ok(())
  }
}
pub fn decode(filepath:&PathBuf,chunktype:&str)->Result<()>{
  let png = read_to_png(filepath)?;
  let chunk = png.chunk_by_type(chunktype).context("Cant find the required chunktype")?;
  let message = chunk.data_as_string()?;
  println!("{}",message);
  Ok(())
}
pub fn remove(filepath:&PathBuf,chunktype:&str)->Result<()>{
  let mut png = read_to_png(filepath)?;
  png.remove_chunk(chunktype)?;
  fs::write(&filepath,png.as_bytes())?;
  Ok(())
}
pub fn print_chunks(filepath:&PathBuf)->Result<()>{
  let png = read_to_png(filepath)?;
  println!("{}",png);
  Ok(())
}
fn read_to_png(filepath:&PathBuf)->Result<Png>{
  let file_content = fs::read(filepath).context("Cant access the input file")?;
  let new_png = Png::try_from(file_content.as_slice())?;
  Ok(new_png)
}
fn chunk_from_arg(string_chunktype:&str,content:&str)->Result<Chunk>{
  let chunktype = ChunkType::from_str(string_chunktype)?;
  let new_chunk = Chunk::new(chunktype, content.into());
  Ok(new_chunk)
}