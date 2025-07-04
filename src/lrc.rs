use anyhow::Result;

/// Simple lyrics format.
///
/// Each entry is a tuple consisting of the time in milliseconds
/// at which the given line of lyrics (a string) is performed.
pub struct SimpleLyrics(Vec<(u64, String)>);

/// Synced lyrics format.
///
/// Each entry is a tuple consisting of the time in milliseconds
/// at which the given line of lyrics is performed. The line
/// itself is broken up into its own list of tuples, specifying
/// the time in milliseconds at which each word or phrase is
/// performed.
pub struct SyncedLyrics(Vec<(u64, Vec<(u64, String)>)>);

pub enum Lyrics {
    SimpleLyrics(SimpleLyrics),
    SyncedLyrics(SyncedLyrics),
}

/// Parse the given LRC file. Detects if it is in simple or
/// synced format and parses it, returning the result.
///
/// Only lines of lyrics are parsed. Any line that doesn't
/// represent lyrics is ignored.
pub fn parse_lrc(lyrics: String) -> Result<Lyrics> {
    todo!()
}
