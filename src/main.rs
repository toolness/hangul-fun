use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::{
    hangul::{
        HangulCharClass, decompose_all_hangul_syllables, decompose_hangul_syllable_to_jamos,
        hangul_jamo_to_compat_with_fallback,
    },
    romanize::romanize_decomposed_hangul,
};

mod hangul;
mod play;
mod pronunciation;
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
        /// Disable alternate screen mode
        #[arg(long = "no-alt", default_value_t = false)]
        no_alt: bool,
    },
}

fn print_char_info(ch: char) {
    let class = HangulCharClass::from(ch);
    let codepoint = ch as u32;
    let start = format!("ch={ch} ({codepoint:#x}) {class:?}");
    let Some((initial_ch, medial_ch, maybe_final_ch)) = decompose_hangul_syllable_to_jamos(ch)
    else {
        println!("{start}");
        return;
    };
    let final_info = if let Some(final_ch) = maybe_final_ch {
        let final_compat = hangul_jamo_to_compat_with_fallback(final_ch);
        format!(" final={final_compat} ({:#x})", final_ch as u32)
    } else {
        String::default()
    };
    let initial_compat = hangul_jamo_to_compat_with_fallback(initial_ch);
    let medial_compat = hangul_jamo_to_compat_with_fallback(medial_ch);
    let initial_codepoint = initial_ch as u32;
    let medial_codepoint = medial_ch as u32;
    println!(
        "{start} initial={initial_compat} ({initial_codepoint:#x}) medial={medial_compat} ({medial_codepoint:#x}){final_info}"
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
        Commands::Play { filename, no_alt } => {
            play::play(filename, !no_alt)?;
        }
    }
    Ok(())
}
