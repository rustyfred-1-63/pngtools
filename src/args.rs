use std::path::PathBuf;
use clap::{Parser, Subcommand};

#[derive(Parser,Debug)]
#[command(author,version,about,long_about = None)]
pub struct Cli {
  #[command(subcommand)]
   pub command:Commands,
}
#[derive(Subcommand,Debug)]
pub enum Commands{
  Encode{
    #[arg(value_name="FILEPATH",required=true)]
    filepath:PathBuf,
    #[arg(value_name="CHUNKTYPE",required=true)]
    chunktype:String,
    #[arg(value_name="MESSAGE",required=true)]
    message:String,
    #[arg(value_name="FILEPATH")]
    output_file:Option<PathBuf>
  },
  Decode{
    #[arg(value_name="FILEPATH",required=true)]
    filepath:PathBuf,
    #[arg(value_name="CHUNKTYPE",required=true)]
    chunktype:String,
  },
  Remove{
    #[arg(value_name="FILEPATH",required=true)]
    filepath:PathBuf,
    #[arg(value_name="CHUNKTYPE",required=true)]
    chunktype:String,
  },
  Print{
    #[arg(value_name="FILEPATH",required=true)]
    filepath:PathBuf
  },
}

