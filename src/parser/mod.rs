use clap::Parser;

/// Command line tool to download Youtube videos
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Video URL
    #[arg(short, long)]
    pub url: String,

    /// Output file name
    #[arg(short, long)]
    pub output_file: String,

    /// Chunk size for partial requests
    #[arg(short, long)]
    pub chunk_size: Option<u32>,
}
