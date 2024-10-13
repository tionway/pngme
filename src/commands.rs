use core::str;
use std::convert::TryFrom;
use std::fs;
use std::str::FromStr;

use crate::args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;
use crate::Result;

/// Encodes a message into a PNG file and saves the result
pub fn encode(args: EncodeArgs) -> Result<()> {
    let mut png = Png::from_file(&args.file_path)?;
    let chunk_type = ChunkType::from_str(&args.chunk_type)?;
    let data = args.message.into_bytes();
    let chunk = Chunk::new(chunk_type, data);

    png.append_chunk(chunk);

    let out_path = args.output_file.unwrap_or(args.file_path);

    fs::write(&out_path, png.as_bytes())?;

    println!("Wrote message to: {:?}", &out_path);

    Ok(())
}

/// Searches for a message hidden in a PNG file and prints the message if one is found
pub fn decode(args: DecodeArgs) -> Result<()> {
    let png = Png::from_file(&args.file_path)?;

    match png.chunk_by_type(&args.chunk_type) {
        Some(chunk) => {
            let message = str::from_utf8(chunk.data())?;
            println!("{}", message);
        }
        None => println!("No such chunk type {}", &args.chunk_type),
    }

    Ok(())
}

/// Removes a chunk from a PNG file and saves the result
pub fn remove(args: RemoveArgs) -> Result<()> {
    let mut png = Png::from_file(&args.file_path)?;

    png.remove_first_chunk(&args.chunk_type)?;

    fs::write(&args.file_path, png.as_bytes())?;

    println!("Removed message from: {:?}", &args.file_path);

    Ok(())
}

/// Prints all of the chunks in a PNG file
pub fn print(args: PrintArgs) -> Result<()> {
    let bytes = fs::read(&args.file_path)?;
    let png = Png::try_from(bytes.as_ref())?;
    println!("{}", png);

    Ok(())
}
