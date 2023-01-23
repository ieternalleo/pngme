pub mod args;
pub mod chunk;
pub mod chunk_type;
pub mod commands;
pub mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

use crate::{chunk::*, chunk_type::*, png::*};
use args::{Cli, PngArgs};
use clap::Parser;
use commands::*;
use png::Png;
use std::convert::TryFrom;
use std::fs::File;
use std::io::{Read, Write};
use std::str::FromStr;
use std::{io::BufReader, path::PathBuf};
/// Usage:
///pngme encode ./dice.png ruSt "This is a secret message!
///pngme decode ./dice.png ruSt
///pngme remove ./dice.png ruSt
//pngme print ./dice.png
///
#[allow(unused_variables)]
fn main() -> Result<()> {
    let args = Cli::parse();

    match args.png_args {
        PngArgs::Encode(args) => encode(args)?,
        PngArgs::Decode(args) => decode(args)?,
        PngArgs::Remove(args) => remove(args)?,
        PngArgs::Print(args) => print(args)?,
    }
    Ok(())
}
