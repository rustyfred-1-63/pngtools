mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;
use args::{Cli, Commands};
use clap::Parser;
use anyhow::{Result};
fn main()->Result<()> {
  let args = Cli::parse();
  match args.command {
    Commands::Encode {filepath,chunktype,message,output_file}=>commands::encode(&filepath,&chunktype,&message,&output_file),
    Commands::Decode {filepath,chunktype}=>commands::decode(&filepath,&chunktype),
    Commands::Print {filepath}=>commands::print_chunks(&filepath),
    Commands::Remove {filepath,chunktype}=>commands::remove(&filepath,&chunktype),
  }
}

