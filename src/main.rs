use anyhow::{Result, anyhow};
use clap::{Parser, Subcommand};
use lrc::Lyrics;
use rodio::{Decoder, OutputStream, Source};
use std::{
    fs::{File, read_to_string},
    io::BufReader,
    path::Path,
    time::Duration,
};

use crate::{
    hangul::{decompose_all_hangul_syllables, decompose_hangul_syllable, get_hangul_char_class},
    romanize::romanize_decomposed_hangul,
};

mod hangul;
mod romanize;

#[derive(Parser)]
#[command(name = "hangul-fun")]
#[command(about = "A program to help one analyze and learn Hangul", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Decode a string
    Decode {
        /// The string to decode
        string: String,
    },
    /// Play a file
    Play {
        /// The filename to play
        filename: String,
    },
}

fn print_char_info(ch: char) {
    let class = get_hangul_char_class(ch);
    let codepoint = ch as u32;
    let start = format!("ch={ch} ({codepoint:#x}) {class:?}");
    let Some((initial_ch, medial_ch, maybe_final_ch)) = decompose_hangul_syllable(ch) else {
        println!("{start}");
        return;
    };
    let final_info = if let Some(final_ch) = maybe_final_ch {
        format!(" final={final_ch} ({:#x})", final_ch as u32)
    } else {
        String::default()
    };
    let initial_codepoint = initial_ch as u32;
    let medial_codepoint = medial_ch as u32;
    println!(
        "{start} initial={initial_ch} ({initial_codepoint:#x}) medial={medial_ch} ({medial_codepoint:#x}){final_info}"
    );
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Decode { string } => {
            for ch in string.chars() {
                print_char_info(ch);
            }
            let decomposed = decompose_all_hangul_syllables(&string);
            println!(
                "decomposed: {decomposed} (original length={}, decomposed length={})",
                string.len(),
                decomposed.len()
            );
            println!("romanized: {}", romanize_decomposed_hangul(&decomposed));
        }
        Commands::Play { filename } => {
            let lrc_filename = Path::new(filename).with_extension("lrc");
            if !lrc_filename.exists() {
                return Err(anyhow!(
                    "LRC file does not exist: {}",
                    lrc_filename.to_string_lossy()
                ));
            }
            let lyrics = Lyrics::from_str(read_to_string(lrc_filename)?)?;
            for (time_tag, line) in lyrics.get_timed_lines() {
                let millis = time_tag.get_timestamp();
                println!("{time_tag} {millis} {line}");
            }
            let (_stream, stream_handle) = OutputStream::try_default()?;
            let file = BufReader::new(File::open(filename)?);
            let mut source = Decoder::new(file)?;
            source.try_seek(Duration::from_secs_f32(0.0)).unwrap();
            stream_handle.play_raw(source.convert_samples())?;
            std::thread::sleep(Duration::from_secs(10));
        }
    }
    Ok(())
}
