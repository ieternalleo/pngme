use crate::args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};
use crate::{Chunk, ChunkType, Error, Png, Result};
use std::convert::TryFrom;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::str::FromStr;

pub fn write_file<P: AsRef<Path>>(outpath: P, data: &[u8]) {}

pub fn encode(args: EncodeArgs) -> Result<()> {
    let mut src_png: Png = Png::from_file(args.file_path)?;

    let chunk_type: ChunkType = ChunkType::from_str(&args.chunk_type)?;
    let chunk_msg: Vec<u8> = args.msg.into_bytes();

    let new_chunk: Chunk = Chunk::new(chunk_type, chunk_msg);
    src_png.append_chunk(new_chunk);

    let outfile_path = args.outfile_path.unwrap_or(PathBuf::from(r"encoded.png"));

    let mut out_file: File = File::open(outfile_path)?;
    out_file.write_all(&src_png.as_bytes())?;
    // let
    Ok(())
}

// Takes a PNG file and prints the data for a chunk specified by its chunktype
pub fn decode(args: DecodeArgs) -> Result<()> {
    let src_png: Png = Png::from_file(args.file_path)?;

    let chunk_type: ChunkType = ChunkType::from_str(&args.chunk_type)?;
    let retrieved_chunk: Option<&Chunk> = src_png.chunk_by_type(&chunk_type.to_string());
    let mut msg: String = String::from("");
    if let Some(chunk_ref) = retrieved_chunk {
        msg = format!(
            "{{ChunkType:{}\n ChunkData:{}}}",
            chunk_type.to_string(),
            chunk_ref.data_as_string()?,
        )
    } else {
        println!("Unable to locate the chunk with the specified type");
    }
    println!("{}", msg);
    Ok(())
}

/// Removes a chunk from a PNG file and saves the result
pub fn remove(args: RemoveArgs) -> Result<()> {
    let mut src_png: Png = Png::from_file(&args.file_path)?;

    let chunk_type: ChunkType = ChunkType::from_str(&args.chunk_type)?;
    let _ = src_png.remove_chunk(&chunk_type.to_string())?;
    let mut out_file: File = File::options().write(true).open(&args.file_path)?;
    out_file.write_all(&src_png.as_bytes())?;

    out_file.sync_all()?;
    // let
    Ok(())
}

/// Prints all of the chunks in a PNG file
pub fn print(args: PrintArgs) -> Result<()> {
    let src_png: Png = Png::from_file(args.file_path)?;
    println!("{}", src_png);

    Ok(())
}
