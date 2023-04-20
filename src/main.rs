//! Vivace 
//!
//! A command line tool for downloading songs from Youtube 
use errors::*;
use std::{env, str::FromStr, process::Output};
use ytb_downloader::*;
use clap::Parser;
use parser::Args;
use log::info;
use std::process::Command;

mod parser;

#[macro_use]
extern crate error_chain;

/// Error types of Vivace 
pub mod errors {
    use std::io;

    error_chain! {
        foreign_links {
            Io(io::Error) #[doc = "Error during IO"];
        }

        errors {
            UnsupportedOperatingSystemError {
                description("I'm sorry, your operating system is unsupported at the moment :("),
                display("I'm sorry, your operating system is unsupported at the moment :("),
            } 
        }
    }
}

#[derive(Debug, PartialEq)]
/// Types of operating systems
pub enum OS {
    /// Linux operating systems
    LINUX,
    /// Windows operating systems
    WINDOWS
}

impl FromStr for OS {
    type Err = ErrorKind;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "linux"   => Ok(OS::LINUX),
            "windows" => Ok(OS::WINDOWS),
            _         => Err(ErrorKind::UnsupportedOperatingSystemError)
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info")
    }
    env_logger::init();

    if let Err(ref e) = run(args).await {
        println!("error: {}", e);

        for e in e.iter().skip(1) {
            println!("caused by: {}", e);
        }

        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            println!("backtrace: {:?}", backtrace);
        }

        ::std::process::exit(1);
    }

    Ok(())
}

async fn run(args: Args) -> Result<()> {
    // Url of video
    let url = args.url;

    // File to write to
    let outfile = args.output_file;

    let outfile_parts = outfile.split_once(".").unwrap();
    let outfile_name = outfile_parts.0;
    let outfile_extension = outfile_parts.1;

    // Chunk size for partial download
    let chunk_size = args.chunk_size;

    // Gets the first available audio format
    let source = get_available_sources(&url)
        .await.chain_err(|| "Could not get any available video formats from Youtube")?
        .into_iter().filter(|s| s.mime_type.contains("audio")).next().chain_err(|| "No audio formats found")?;
    info!("Found audio source.");

    // Encoding of audio format
    let file_extension = source.mime_type.rsplit_once(";")
        .chain_err(|| "Error while parsing file extension")?
        .0.split_once("/")
        .chain_err(|| "Error while parsing file extension")?
        .1;

    // Temp file to write video with original encoding to
    let temp_file = format!("{}.{}", outfile_name, file_extension);
    download_video(&source, &temp_file, chunk_size).await.chain_err(|| "Could not download video")?;

    // Convert to encoding of output file
    info!("Converting audio from {file_extension} type to {outfile_extension}...");
    convert(&temp_file, &outfile).chain_err(|| "Error while converting file")?;
    info!("Done.");

    Ok(())
}

// Converts a file to another file with different encoding
fn convert(file: &str, outfile: &str) -> Result<Output> {
    match OS::from_str(env::consts::OS)? {
        OS::LINUX => Command::new("ffmpeg")
                        .arg("-i")
                        .arg(file)
                        .arg(outfile)
                        .output()
                        .chain_err(|| "Error while converting file"),
        OS::WINDOWS => Command::new("ffmpeg")
                       .arg("-i")
                       .arg(file)
                       .arg(outfile)
                       .output()
                       .chain_err(|| "Error while converting file"),
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn file_is_converted() {
        const INPUT_FILE: &str = "assets/ducksong.mp4"; 
        const OUTPUT_FILE: &str = "assets/ducksong.mp3"; 
        const TEST_OUTPUT_FILE: &str = "assets/ducksong-test.mp3"; 

        let output = convert(INPUT_FILE, OUTPUT_FILE);

        let output_file = File::open(OUTPUT_FILE).unwrap();
        let test_output_file = File::open(TEST_OUTPUT_FILE).unwrap();

        assert!(output.is_ok()); 
        assert_eq!(file_length(&test_output_file), file_length(&output_file));
    }

    fn file_length(file: &File) -> u64 {
        file.metadata().unwrap().len() 
    }
}

