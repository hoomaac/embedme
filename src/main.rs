use args::EmbedMeCli;
use clap::Parser;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;
mod errors;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let embed_args = args::EmbedMeCli::parse();

    let res = match embed_args {
        EmbedMeCli::Decode(decode) => commands::decode(decode),
        EmbedMeCli::Encode(encode) => commands::encode(encode),
        EmbedMeCli::Remove(remove) => commands::remove(remove),
        EmbedMeCli::Print(print) => commands::print_chunks(print),
    };

    res
}
