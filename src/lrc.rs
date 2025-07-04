use anyhow::Result;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{take_until, take_while1},
    character::complete::{char, digit1, not_line_ending},
    combinator::{map, map_res, value},
    multi::many1,
    sequence::delimited,
};

/// Simple lyrics format.
///
/// Each entry is a tuple consisting of the time in milliseconds
/// at which the given line of lyrics (a string) is performed.
#[derive(Debug, Clone, PartialEq)]
pub struct SimpleLyrics(pub Vec<(u64, String)>);

/// Synced lyrics format.
///
/// Each entry is a tuple consisting of the time in milliseconds
/// at which the given line of lyrics is performed. The line
/// itself is broken up into its own list of tuples, specifying
/// the time in milliseconds at which each word or phrase is
/// performed.
#[derive(Debug, Clone, PartialEq)]
pub struct SyncedLyrics(pub Vec<(u64, Vec<(u64, String)>)>);

#[derive(Debug, Clone, PartialEq)]
pub enum Lyrics {
    SimpleLyrics(SimpleLyrics),
    SyncedLyrics(SyncedLyrics),
}

impl SyncedLyrics {
    /// Convert SyncedLyrics to SimpleLyrics by joining all words in each line
    pub fn to_simple(&self) -> SimpleLyrics {
        let simple_entries: Vec<(u64, String)> = self
            .0
            .iter()
            .map(|(timestamp, words)| {
                let joined_text = words
                    .iter()
                    .map(|(_, text)| text.as_str())
                    .collect::<Vec<&str>>()
                    .join("");
                (*timestamp, joined_text)
            })
            .collect();

        SimpleLyrics(simple_entries)
    }
}

/// Parse minutes:seconds.centiseconds or minutes:seconds.milliseconds format
fn parse_timestamp(input: &str) -> IResult<&str, u64> {
    map(
        (
            map_res(digit1, |s: &str| s.parse::<u64>()),
            char(':'),
            map_res(take_while1(|c: char| c.is_ascii_digit()), |s: &str| {
                s.parse::<u64>()
            }),
            char('.'),
            map_res(take_while1(|c: char| c.is_ascii_digit()), |s: &str| {
                s.parse::<u64>()
            }),
        ),
        |(minutes, _, seconds, _, fraction)| {
            let fraction_len = fraction.to_string().len();
            let milliseconds = if fraction_len == 2 {
                // Centiseconds (hundredths)
                fraction * 10
            } else if fraction_len == 3 {
                // Milliseconds (thousandths)
                fraction
            } else {
                // Handle other cases by padding or truncating to 3 digits
                if fraction_len < 3 {
                    fraction * 10_u64.pow((3 - fraction_len) as u32)
                } else {
                    fraction / 10_u64.pow((fraction_len - 3) as u32)
                }
            };
            minutes * 60 * 1000 + seconds * 1000 + milliseconds
        },
    )
    .parse(input)
}

/// Parse a timestamp tag [mm:ss.xx]
fn parse_timestamp_tag(input: &str) -> IResult<&str, u64> {
    delimited(char('['), parse_timestamp, char(']')).parse(input)
}

/// Parse multiple timestamp tags at the beginning of a line
fn parse_timestamp_tags(input: &str) -> IResult<&str, Vec<u64>> {
    many1(parse_timestamp_tag).parse(input)
}

/// Parse a word/phrase with its timestamp in synced format
fn parse_synced_word(input: &str) -> IResult<&str, (u64, String)> {
    let (input, _) = char('<')(input)?;
    let (input, timestamp) = parse_timestamp(input)?;
    let (input, _) = char('>')(input)?;

    // Try to find the next '<' or use the rest of the line
    let end_pos = input.find('<').unwrap_or(input.len());
    let text = &input[..end_pos];
    let remaining = &input[end_pos..];

    Ok((remaining, (timestamp, text.to_string())))
}

/// Parse the last word/phrase in a synced line (no trailing timestamp)
fn parse_last_synced_word(input: &str) -> IResult<&str, String> {
    // This function is no longer needed with the new parse_synced_word
    Ok(("", input.to_string()))
}

/// Parse a complete synced lyrics line
fn parse_synced_line(input: &str) -> IResult<&str, Vec<(u64, Vec<(u64, String)>)>> {
    let (input, timestamps) = parse_timestamp_tags(input)?;
    let (input, words) = many1(parse_synced_word).parse(input)?;

    Ok((
        input,
        timestamps
            .into_iter()
            .map(|ts| (ts, words.clone()))
            .collect(),
    ))
}

/// Parse a simple lyrics line
fn parse_simple_line(input: &str) -> IResult<&str, Vec<(u64, String)>> {
    let (input, timestamps) = parse_timestamp_tags(input)?;
    let (input, text) = not_line_ending(input)?;

    Ok((
        input,
        timestamps
            .into_iter()
            .map(|ts| (ts, text.to_string()))
            .collect(),
    ))
}

/// Parse a metadata line (to be ignored)
fn parse_metadata_line(input: &str) -> IResult<&str, ()> {
    value((), delimited(char('['), take_until("]"), char(']'))).parse(input)
}

/// Parse any line that should be ignored
fn parse_ignored_line(input: &str) -> IResult<&str, ()> {
    alt((value((), parse_metadata_line), value((), not_line_ending))).parse(input)
}

/// Parse the given LRC file. Detects if it is in simple or
/// synced format and parses it, returning the result.
///
/// Only lines of lyrics are parsed. Any line that doesn't
/// represent lyrics is ignored.
pub fn parse_lrc(lyrics: String) -> Result<Lyrics> {
    let lines: Vec<&str> = lyrics.lines().collect();

    // First, check if any line contains synced format
    let is_synced = lines
        .iter()
        .any(|line| line.contains("<") && line.contains(">"));

    if is_synced {
        let mut synced_lyrics = Vec::new();

        for line in lines {
            if line.trim().is_empty() {
                continue;
            }

            if let Ok((_, entries)) = parse_synced_line(line) {
                synced_lyrics.extend(entries);
            }
            // Ignore lines that don't parse as synced lyrics
        }

        // Sort by timestamp
        synced_lyrics.sort_by_key(|(ts, _)| *ts);

        Ok(Lyrics::SyncedLyrics(SyncedLyrics(synced_lyrics)))
    } else {
        let mut simple_lyrics = Vec::new();

        for line in lines {
            if line.trim().is_empty() {
                continue;
            }

            // Check if it's a metadata line (has brackets but no timestamp format)
            if line.starts_with('[') && line.contains(']') && !line.contains(':') {
                continue; // Skip metadata lines
            }

            if let Ok((_, entries)) = parse_simple_line(line) {
                simple_lyrics.extend(entries);
            }
            // Ignore lines that don't parse as simple lyrics
        }

        // Sort by timestamp
        simple_lyrics.sort_by_key(|(ts, _)| *ts);

        Ok(Lyrics::SimpleLyrics(SimpleLyrics(simple_lyrics)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_timestamp() {
        assert_eq!(parse_timestamp("00:12.34"), Ok(("", 12340)));
        assert_eq!(parse_timestamp("01:23.45"), Ok(("", 83450)));
        assert_eq!(parse_timestamp("00:12.345"), Ok(("", 12345)));
        assert_eq!(parse_timestamp("01:00.000"), Ok(("", 60000)));
        assert_eq!(parse_timestamp("00:00.1"), Ok(("", 100)));
        assert_eq!(parse_timestamp("00:00.12"), Ok(("", 120)));
        assert_eq!(parse_timestamp("00:00.123"), Ok(("", 123)));
        assert_eq!(parse_timestamp("00:00.1234"), Ok(("", 123)));
    }

    #[test]
    fn test_parse_simple_lyrics() {
        let lrc = r#"[ar:Artist Name]
[ti:Song Title]
[00:12.34]First line of lyrics
[00:15.67]Second line of lyrics
[00:20.00]Third line of lyrics"#;

        let result = parse_lrc(lrc.to_string()).unwrap();

        match result {
            Lyrics::SimpleLyrics(SimpleLyrics(lyrics)) => {
                assert_eq!(lyrics.len(), 3);
                assert_eq!(lyrics[0], (12340, "First line of lyrics".to_string()));
                assert_eq!(lyrics[1], (15670, "Second line of lyrics".to_string()));
                assert_eq!(lyrics[2], (20000, "Third line of lyrics".to_string()));
            }
            _ => panic!("Expected SimpleLyrics"),
        }
    }

    #[test]
    fn test_parse_simple_lyrics_with_milliseconds() {
        let lrc = r#"[00:12.345]First line with milliseconds
[00:15.999]Second line with milliseconds"#;

        let result = parse_lrc(lrc.to_string()).unwrap();

        match result {
            Lyrics::SimpleLyrics(SimpleLyrics(lyrics)) => {
                assert_eq!(lyrics.len(), 2);
                assert_eq!(
                    lyrics[0],
                    (12345, "First line with milliseconds".to_string())
                );
                assert_eq!(
                    lyrics[1],
                    (15999, "Second line with milliseconds".to_string())
                );
            }
            _ => panic!("Expected SimpleLyrics"),
        }
    }

    #[test]
    fn test_parse_synced_lyrics() {
        let lrc = r#"[ar:Artist Name]
[00:12.34]<00:12.34>First <00:13.00>word <00:13.50>synced
[00:15.67]<00:15.67>Second <00:16.00>line"#;

        let result = parse_lrc(lrc.to_string()).unwrap();

        match result {
            Lyrics::SyncedLyrics(SyncedLyrics(lyrics)) => {
                assert_eq!(lyrics.len(), 2);

                let (ts1, words1) = &lyrics[0];
                assert_eq!(*ts1, 12340);
                assert_eq!(words1.len(), 3);
                assert_eq!(words1[0], (12340, "First ".to_string()));
                assert_eq!(words1[1], (13000, "word ".to_string()));
                assert_eq!(words1[2], (13500, "synced".to_string()));

                let (ts2, words2) = &lyrics[1];
                assert_eq!(*ts2, 15670);
                assert_eq!(words2.len(), 2);
                assert_eq!(words2[0], (15670, "Second ".to_string()));
                assert_eq!(words2[1], (16000, "line".to_string()));
            }
            _ => panic!("Expected SyncedLyrics"),
        }
    }

    #[test]
    fn test_parse_multiple_timestamps() {
        let lrc = r#"[00:12.34][00:15.67]Line with multiple timestamps
[00:20.00]Normal line"#;

        let result = parse_lrc(lrc.to_string()).unwrap();

        match result {
            Lyrics::SimpleLyrics(SimpleLyrics(lyrics)) => {
                assert_eq!(lyrics.len(), 3);
                assert_eq!(
                    lyrics[0],
                    (12340, "Line with multiple timestamps".to_string())
                );
                assert_eq!(
                    lyrics[1],
                    (15670, "Line with multiple timestamps".to_string())
                );
                assert_eq!(lyrics[2], (20000, "Normal line".to_string()));
            }
            _ => panic!("Expected SimpleLyrics"),
        }
    }

    #[test]
    fn test_ignore_metadata_lines() {
        let lrc = r#"[ar:Artist Name]
[ti:Song Title]
[al:Album Name]
[by:Creator]
[offset:1000]
[00:12.34]Only lyrics line"#;

        let result = parse_lrc(lrc.to_string()).unwrap();

        match result {
            Lyrics::SimpleLyrics(SimpleLyrics(lyrics)) => {
                assert_eq!(lyrics.len(), 1);
                assert_eq!(lyrics[0], (12340, "Only lyrics line".to_string()));
            }
            _ => panic!("Expected SimpleLyrics"),
        }
    }

    #[test]
    fn test_empty_and_invalid_lines() {
        let lrc = r#"[ar:Artist Name]

[00:12.34]Valid line
Invalid line without timestamp
[00:15.67]Another valid line
"#;

        let result = parse_lrc(lrc.to_string()).unwrap();

        match result {
            Lyrics::SimpleLyrics(SimpleLyrics(lyrics)) => {
                assert_eq!(lyrics.len(), 2);
                assert_eq!(lyrics[0], (12340, "Valid line".to_string()));
                assert_eq!(lyrics[1], (15670, "Another valid line".to_string()));
            }
            _ => panic!("Expected SimpleLyrics"),
        }
    }

    #[test]
    fn test_sorting_by_timestamp() {
        let lrc = r#"[00:15.67]Second line
[00:12.34]First line
[00:20.00]Third line"#;

        let result = parse_lrc(lrc.to_string()).unwrap();

        match result {
            Lyrics::SimpleLyrics(SimpleLyrics(lyrics)) => {
                assert_eq!(lyrics.len(), 3);
                assert_eq!(lyrics[0], (12340, "First line".to_string()));
                assert_eq!(lyrics[1], (15670, "Second line".to_string()));
                assert_eq!(lyrics[2], (20000, "Third line".to_string()));
            }
            _ => panic!("Expected SimpleLyrics"),
        }
    }

    #[test]
    fn test_synced_to_simple_conversion() {
        // Create a SyncedLyrics instance
        let synced_lyrics = SyncedLyrics(vec![
            (
                12340,
                vec![
                    (12340, "First ".to_string()),
                    (13000, "word ".to_string()),
                    (13500, "synced".to_string()),
                ],
            ),
            (
                15670,
                vec![(15670, "Second ".to_string()), (16000, "line".to_string())],
            ),
        ]);

        // Convert to SimpleLyrics
        let simple_lyrics = synced_lyrics.to_simple();

        // Verify the conversion
        assert_eq!(simple_lyrics.0.len(), 2);
        assert_eq!(simple_lyrics.0[0], (12340, "First word synced".to_string()));
        assert_eq!(simple_lyrics.0[1], (15670, "Second line".to_string()));
    }

    #[test]
    fn test_synced_to_simple_with_empty_lines() {
        // Test with some empty word lists
        let synced_lyrics = SyncedLyrics(vec![
            (10000, vec![]), // Empty line
            (
                20000,
                vec![(20000, "Hello ".to_string()), (20500, "world".to_string())],
            ),
            (30000, vec![(30000, "".to_string())]), // Line with empty string
        ]);

        let simple_lyrics = synced_lyrics.to_simple();

        assert_eq!(simple_lyrics.0.len(), 3);
        assert_eq!(simple_lyrics.0[0], (10000, "".to_string()));
        assert_eq!(simple_lyrics.0[1], (20000, "Hello world".to_string()));
        assert_eq!(simple_lyrics.0[2], (30000, "".to_string()));
    }

    #[test]
    fn test_synced_to_simple_preserves_timestamps() {
        // Ensure timestamps are preserved correctly
        let synced_lyrics = SyncedLyrics(vec![
            (
                5000,
                vec![
                    (5000, "A ".to_string()),
                    (5500, "B ".to_string()),
                    (6000, "C".to_string()),
                ],
            ),
            (10000, vec![(10000, "D".to_string())]),
        ]);

        let simple_lyrics = synced_lyrics.to_simple();

        // The line timestamp should be preserved, not the individual word timestamps
        assert_eq!(simple_lyrics.0[0].0, 5000);
        assert_eq!(simple_lyrics.0[1].0, 10000);
    }
}
