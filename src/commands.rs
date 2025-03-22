use crate::args::{Encode, Decode, Remove, Print};
use crate::errors::Errors;
use crate::Result;
use crate::{png::Png, chunk::Chunk, chunk_type::ChunkType};
use std::fs::{read, write};

/// Encodes a message into a PNG file and saves the result
pub fn encode(args: Encode) -> Result<()> {
    let file = read(&args.path)?;
    let chunk = Chunk::new(args.chunk_type, args.message.as_bytes().to_vec());
    let png = Png::try_from(&file[..]);

    let _ = match png {
        Err(_) => Err(()),
        Ok(mut png) => {
            png.append_chunk(chunk);
            let bytes = png.as_bytes();
            Ok(write(&args.path, &bytes))
        }
    };

    Ok(())
}

/// Searches for a message hidden in a PNG file and prints the message if one is found
pub fn decode(args: Decode) -> Result<()> {
    let file = read(&args.path)?;
    let png = Png::try_from(&file[..])?;
    if let Some(chunk) = png.chunk_by_type(&args.chunk_type) {
        let msg = chunk.data_as_string()?;

        println!("Secret: {}", msg);
    }
    else {
        return Err(Box::new(Errors::GenericError("Chunk not found".to_string())));
    }

    Ok(())
}

/// Removes a chunk from a PNG file and saves the result
pub fn remove(args: Remove) -> Result<()> {
    let file = read(&args.path)?;
    let mut png= Png::try_from(&file[..])?;

    png.remove_first_chunk(&args.chunk_type)?;
    write(&args.path, &png.as_bytes())?;

    Ok(())
}

/// Prints all of the chunks in a PNG file
pub fn print_chunks(args: Print) -> Result<()> {
    let file = read(args.path)?;

    let png = Png::try_from(&file[..]);

    let _ = match png {
        Err(_) => Err(()),
        Ok(png) => Ok(print!("PNG: {:?}\n", png))
    };

    Ok(())
}



