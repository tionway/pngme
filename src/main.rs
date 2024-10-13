#![allow(dead_code)]

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod errors;
mod png;

use clap::Parser;
use commands::decode;
use commands::encode;
use commands::print;
use commands::remove;
use errors::Result;

fn main() -> Result<()> {
    let args = args::Args::parse();

    match args.command {
        args::Commands::Encode(encode_args) => encode(encode_args),
        args::Commands::Decode(decode_args) => decode(decode_args),
        args::Commands::Remove(remove_args) => remove(remove_args),
        args::Commands::Print(print_args) => print(print_args),
    }
}
