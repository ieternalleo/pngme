use crate::{chunk_type::ChunkType, Error, Result};

use clap::{Args, Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub png_args: PngArgs,
}

#[derive(Debug, Subcommand)]
pub enum PngArgs {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

#[derive(clap::Args, Debug)]
pub struct EncodeArgs {
    #[clap(value_parser)]
    pub file_path: PathBuf, // File Path

    #[clap(value_parser)]
    pub chunk_type: String, // ChunkType

    #[clap(value_parser)]
    pub msg: String, // Message

    #[clap(value_parser)]
    pub outfile_path: Option<PathBuf>, // Output File  (optional)
}

#[derive(clap::Args, Debug)]
pub struct DecodeArgs {
    #[clap(value_parser)]
    pub file_path: PathBuf, // File Path

    #[clap(value_parser)]
    pub chunk_type: String, // Chunk Type
}

#[derive(clap::Args, Debug)]
pub struct RemoveArgs {
    #[clap(value_parser)]
    pub file_path: PathBuf, // File Path
    #[clap(value_parser)]
    pub chunk_type: String, // Chunk Type
}

#[derive(clap::Args, Debug)]
pub struct PrintArgs {
    #[clap(value_parser)]
    pub file_path: PathBuf, // File Path
}
