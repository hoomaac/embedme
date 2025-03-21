use std::path::PathBuf;

use clap::Parser;
use crate::chunk_type::ChunkType;


#[derive(Parser)]
#[command(version, about)]
pub enum EmbedMeCli {
    Encode(Encode),
    Decode(Decode),
    Remove(Remove),
    Print(Print),
}

#[derive(clap::Args, Debug)]
pub struct Encode {
    pub path: PathBuf,
    pub chunk_type: ChunkType,
    pub message: String,
}

#[derive(clap::Args, Debug)]
pub struct Decode {
    pub path: PathBuf,
    pub chunk_type: String,
}

#[derive(clap::Args, Debug)]
pub struct Remove {
    pub path: PathBuf,
    pub chunk_type: String,
}

#[derive(clap::Args, Debug)]
pub struct Print {
    pub path: PathBuf,
}