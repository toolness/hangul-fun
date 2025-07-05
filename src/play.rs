use anyhow::{Result, anyhow};
use crossterm::{
    QueueableCommand,
    cursor::{Hide, MoveTo, MoveToColumn, MoveToNextLine, Show},
    event::{Event, KeyCode, KeyEvent, KeyModifiers, poll, read},
    execute,
    style::{Attribute, Color, Print, PrintStyledContent, SetAttribute, Stylize},
    terminal::{
        Clear, ClearType, DisableLineWrap, EnableLineWrap, EnterAlternateScreen,
        LeaveAlternateScreen, disable_raw_mode, enable_raw_mode, size,
    },
};
use rodio::{Decoder, OutputStream, Sink};
use std::{
    fs::{File, read_to_string},
    io::{BufReader, Stdout, Write, stdout},
    path::Path,
    time::Duration,
};

use crate::{
    hangul::{
        HangulCharClass, decompose_all_hangul_syllables, decompose_hangul_syllable_to_jamos,
        hangul_jamo_to_compat_with_fallback,
    },
    lrc::{Lyrics, parse_lrc},
    pronunciation::get_jamo_pronunciation,
    romanize::{get_romanized_jamo, romanize_decomposed_hangul},
};

/// Amount to rewind, in seconds, when user presses the
/// hotkey. If you change this, be sure to change `HELP_LINES`!
const REWIND_SECS: u64 = 2;

const NUM_HELP_LINES: usize = 6;

const HELP_LINES: [&'static str; NUM_HELP_LINES] = [
    "↑/↓   - prev/next lines",
    "←/→   - prev/next syllable",
    "Enter - play current line",
    "Space - pause/unpause",
    "B     - rewind 2 seconds",
    "Esc   - quit",
];

struct App {
    lyrics_lines_to_show: usize,
    first_lyrics_line: usize,
    curr_lyrics_line: usize,
    curr_word: usize,
    curr_syllable: usize,
    lyrics: Vec<(Duration, String)>,
    sink: Sink,
}

impl App {
    pub fn run(&mut self) -> Result<()> {
        loop {
            self.render()?;
            let event = if self.sink.is_paused() {
                read()?
            } else {
                // We're playing music, and parts of our UI
                // depend on the playback state, so don't wait
                // forever for an event before we force a
                // re-render.
                if !poll(Duration::from_millis(100))? {
                    continue;
                }
                read()?
            };

            // If these lines are changed, be sure to change
            // `HELP_LINES` too.
            if event == key(KeyCode::Esc) {
                break;
            } else if event == key(KeyCode::Char(' ')) {
                self.toggle_pause();
            } else if event == key(KeyCode::Down) || event == key_ctrl(KeyCode::Char('n')) {
                self.go_to_next_line();
            } else if event == key(KeyCode::Up) || event == key_ctrl(KeyCode::Char('p')) {
                self.go_to_prev_line();
            } else if event == key(KeyCode::Left) || event == key_ctrl(KeyCode::Char('b')) {
                self.select_prev_syllable();
            } else if event == key(KeyCode::Right) || event == key_ctrl(KeyCode::Char('f')) {
                self.select_next_syllable();
            } else if event == key(KeyCode::Enter) {
                self.seek_to_current_lyric()?;
            } else if event == key(KeyCode::Char('b')) {
                self.seek_backward()?;
            }
        }

        Ok(())
    }

    fn get_selection(&self) -> Option<(&str, char, &str)> {
        if let Some((_, line)) = self.lyrics.get(self.curr_lyrics_line) {
            let mut word_idx = 0;
            for (class, word) in HangulCharClass::split(&line) {
                if class == HangulCharClass::Syllables {
                    if word_idx == self.curr_word {
                        let mut syllable_idx = 0;
                        for (idx, char) in word.char_indices() {
                            if syllable_idx == self.curr_syllable {
                                return Some((word, char, &word[idx..idx + char.len_utf8()]));
                            }
                            syllable_idx += 1;
                        }
                    }
                    word_idx += 1;
                }
            }
        }
        None
    }

    fn get_playback_line_idx(&self) -> Option<usize> {
        let sink_pos = self.sink.get_pos();
        let mut latest_idx = None;
        for (idx, (pos, _)) in self.lyrics.iter().enumerate() {
            if pos <= &sink_pos {
                latest_idx = Some(idx);
            } else {
                return latest_idx;
            }
        }
        None
    }

    pub fn render(&self) -> Result<()> {
        let mut stdout = stdout();
        stdout.queue(MoveTo(0, 0))?;
        self.render_status_bar(&mut stdout)?;
        self.render_lyrics(&mut stdout)?;
        self.render_selection_info(&mut stdout)?;
        stdout.queue(MoveTo(0, size()?.1 - help_lines_two_column_height() as u16))?;
        self.render_help(&mut stdout)?;
        stdout.flush()?;
        Ok(())
    }

    fn playback_icon(&self) -> &'static str {
        if self.sink.is_paused() {
            "⏸︎"
        } else {
            "⏵︎"
        }
    }

    fn render_status_bar(&self, stdout: &mut Stdout) -> Result<()> {
        stdout.queue(SetAttribute(Attribute::Reverse))?;
        let columns = size()?.0 as usize;
        stdout.queue(Print(format!(
            " HANGUL-FUN{:>width$} ",
            self.playback_icon(),
            width = columns - 11
        )))?;
        stdout.queue(SetAttribute(Attribute::NoReverse))?;
        stdout.queue(MoveToNextLine(1))?;
        Ok(())
    }

    fn render_lyrics(&self, stdout: &mut Stdout) -> Result<()> {
        let lyrics = &self.lyrics;
        let mut i = self.first_lyrics_line;
        let playback_line_idx = self.get_playback_line_idx();
        loop {
            let Some((_, line)) = lyrics.get(i) else {
                break;
            };
            if i == self.curr_lyrics_line {
                stdout.queue(Print("> "))?;
                let mut word_idx = 0;
                for (class, str) in HangulCharClass::split(&line) {
                    if class == HangulCharClass::Syllables {
                        if word_idx == self.curr_word {
                            let mut syllable_idx = 0;
                            for (idx, char) in str.char_indices() {
                                let syllable = (&str[idx..idx + char.len_utf8()]).on(Color::Grey);
                                if syllable_idx == self.curr_syllable {
                                    stdout.queue(PrintStyledContent(syllable.with(Color::Blue)))?;
                                } else {
                                    stdout
                                        .queue(PrintStyledContent(syllable.with(Color::Black)))?;
                                }
                                syllable_idx += 1;
                            }
                        } else {
                            stdout.queue(Print(str))?;
                        }
                        word_idx += 1;
                    } else {
                        stdout.queue(Print(str))?;
                    }
                }
            } else {
                if Some(i) == playback_line_idx {
                    stdout.queue(PrintStyledContent(self.playback_icon().with(Color::Grey)))?;
                    stdout.queue(Print(" "))?;
                } else {
                    stdout.queue(Print("  "))?;
                }
                stdout.queue(Print(&line))?;
            }
            stdout.queue(Clear(ClearType::UntilNewLine))?;
            stdout.queue(MoveToNextLine(1))?;
            i += 1;
            if i >= self.first_lyrics_line + self.lyrics_lines_to_show {
                break;
            }
        }
        Ok(())
    }

    fn render_horizontal_line(&self, stdout: &mut Stdout) -> Result<()> {
        let cols = size()?.0 as usize;
        let mut line = String::with_capacity(cols);
        for _ in 0..cols {
            line.push('⎯');
        }
        stdout.queue(Print(line))?;
        stdout.queue(MoveToNextLine(1))?;
        Ok(())
    }

    fn render_cleared_lines(&self, stdout: &mut Stdout, count: usize) -> Result<()> {
        for _ in 0..count {
            stdout.queue(Clear(ClearType::CurrentLine))?;
            stdout.queue(MoveToNextLine(1))?;
        }
        Ok(())
    }

    fn render_selection_info(&self, stdout: &mut Stdout) -> Result<()> {
        if let Some((selected_word, selected_syllable, syllable_str)) = self.get_selection() {
            let mut clear_extra_lines = 0;
            self.render_horizontal_line(stdout)?;
            stdout.queue(Print("Selected word: "))?;
            stdout.queue(Print(selected_word))?;
            let decomposed = decompose_all_hangul_syllables(selected_word);
            let romanized = romanize_decomposed_hangul(&decomposed);
            stdout.queue(Print(format!(" ({romanized})")))?;
            stdout.queue(Clear(ClearType::UntilNewLine))?;
            stdout.queue(MoveToNextLine(1))?;

            stdout.queue(Print(format!("Selected syllable: ")))?;
            stdout.queue(Print(syllable_str))?;
            stdout.queue(Clear(ClearType::UntilNewLine))?;
            stdout.queue(MoveToNextLine(1))?;
            if let Some((initial_ch, medial_ch, maybe_final_ch)) =
                decompose_hangul_syllable_to_jamos(selected_syllable)
            {
                let initial_compat = hangul_jamo_to_compat_with_fallback(initial_ch);
                let mut initial_rom = get_romanized_jamo(initial_ch, false).unwrap_or("?");
                if initial_rom == "" {
                    initial_rom = "silent";
                }
                let initial_hint = get_jamo_pronunciation(initial_ch);
                let medial_compat = hangul_jamo_to_compat_with_fallback(medial_ch);
                let medial_rom = get_romanized_jamo(medial_ch, false).unwrap_or("?");
                let medial_hint = get_jamo_pronunciation(medial_ch);
                stdout.queue(Print(format!(
                    "  Initial: {initial_compat} ({initial_rom}) {initial_hint}"
                )))?;
                stdout.queue(Clear(ClearType::UntilNewLine))?;
                stdout.queue(MoveToNextLine(1))?;
                stdout.queue(Print(format!(
                    "  Medial : {medial_compat} ({medial_rom}) {medial_hint}"
                )))?;
                stdout.queue(Clear(ClearType::UntilNewLine))?;
                stdout.queue(MoveToNextLine(1))?;
                if let Some(final_ch) = maybe_final_ch {
                    let final_compat = hangul_jamo_to_compat_with_fallback(final_ch);
                    let final_rom_no_vowel = get_romanized_jamo(final_ch, false).unwrap_or("?");
                    let final_rom_vowel = get_romanized_jamo(final_ch, true).unwrap_or("?");
                    let final_hint = get_jamo_pronunciation(final_ch);

                    if final_rom_no_vowel == final_rom_vowel {
                        stdout.queue(Print(format!(
                            "  Final  : {final_compat} ({final_rom_no_vowel}) {final_hint}"
                        )))?;
                    } else {
                        stdout.queue(Print(format!(
                            "  Final  : {final_compat} ({final_rom_no_vowel}/{final_rom_vowel}) {final_hint}"
                        )))?;
                    }
                    stdout.queue(Clear(ClearType::UntilNewLine))?;
                    stdout.queue(MoveToNextLine(1))?;
                } else {
                    clear_extra_lines += 1;
                }
            }
            self.render_horizontal_line(stdout)?;
            self.render_cleared_lines(stdout, clear_extra_lines)?;
        } else {
            self.render_cleared_lines(stdout, 7)?;
        }
        Ok(())
    }

    fn render_help(&self, stdout: &mut Stdout) -> Result<()> {
        let col_2 = size()?.0 / 2;
        let height = help_lines_two_column_height();
        for i in 0..height {
            let first_col = HELP_LINES[i];
            stdout.queue(PrintStyledContent(first_col.with(Color::DarkGrey)))?;
            if let Some(&second_col) = HELP_LINES.get(height + i) {
                stdout.queue(MoveToColumn(col_2))?;
                stdout.queue(PrintStyledContent(second_col.with(Color::DarkGrey)))?;
            }
            stdout.queue(Clear(ClearType::UntilNewLine))?;
            stdout.queue(MoveToNextLine(1))?;
        }
        Ok(())
    }

    pub fn go_to_next_line(&mut self) {
        if self.curr_lyrics_line + 1 < self.lyrics.len() {
            self.curr_lyrics_line += 1;
            self.curr_word = 0;
            self.curr_syllable = 0;
            if self.first_lyrics_line + self.lyrics_lines_to_show <= self.curr_lyrics_line {
                self.first_lyrics_line += 1;
            }
        }
    }

    pub fn go_to_prev_line(&mut self) {
        if self.curr_lyrics_line > 0 {
            self.curr_lyrics_line -= 1;
            self.curr_word = 0;
            self.curr_syllable = 0;
            if self.first_lyrics_line > self.curr_lyrics_line {
                self.first_lyrics_line = self.curr_lyrics_line;
            }
        }
    }

    fn get_curr_line_word_lengths(&self) -> Vec<usize> {
        HangulCharClass::split(&self.lyrics[self.curr_lyrics_line].1)
            .into_iter()
            .filter_map(|(class, str)| {
                if class != HangulCharClass::Syllables {
                    None
                } else {
                    Some(str.chars().count())
                }
            })
            .collect()
    }

    fn select_next_syllable(&mut self) {
        let word_lengths = self.get_curr_line_word_lengths();
        if let Some(&num_syllables) = word_lengths.get(self.curr_word) {
            if self.curr_syllable + 1 < num_syllables {
                self.curr_syllable += 1;
            } else if self.curr_word + 1 < word_lengths.len() {
                self.curr_word += 1;
                self.curr_syllable = 0;
            }
        }
    }

    fn select_prev_syllable(&mut self) {
        let word_lengths = self.get_curr_line_word_lengths();
        if let Some(_) = word_lengths.get(self.curr_word) {
            if self.curr_syllable > 0 {
                self.curr_syllable -= 1;
            } else if self.curr_word > 0 {
                self.curr_word -= 1;
                self.curr_syllable = word_lengths[self.curr_word] - 1;
            }
        }
    }

    pub fn toggle_pause(&mut self) {
        if self.sink.is_paused() {
            self.sink.play();
        } else {
            self.sink.pause();
        }
    }

    fn seek_to(&self, pos: Duration) -> Result<()> {
        if let Err(err) = self.sink.try_seek(pos.clone()) {
            return Err(anyhow!("Failed to seek: {err}"));
        }
        self.sink.play();
        Ok(())
    }

    fn seek_to_current_lyric(&self) -> Result<()> {
        if let Some((pos, _)) = self.lyrics.get(self.curr_lyrics_line) {
            self.seek_to(pos.clone())?;
        }
        Ok(())
    }

    fn seek_backward(&self) -> Result<()> {
        let curr_pos = self.sink.get_pos();
        self.seek_to(curr_pos.saturating_sub(Duration::from_secs(REWIND_SECS)))
    }
}

fn key(code: KeyCode) -> Event {
    Event::Key(code.into())
}

fn key_ctrl(code: KeyCode) -> Event {
    Event::Key(KeyEvent::new(code, KeyModifiers::CONTROL))
}

fn help_lines_two_column_height() -> usize {
    (HELP_LINES.len() as f32 / 2.0).ceil() as usize
}

fn lyrics_to_vec(lyrics: Lyrics) -> Vec<(Duration, String)> {
    let simple_vec = match lyrics {
        Lyrics::SimpleLyrics(simple_lyrics) => simple_lyrics.0,
        Lyrics::SyncedLyrics(synced_lyrics) => synced_lyrics.to_simple().0,
    };

    simple_vec
        .into_iter()
        .filter_map(|(millis, line)| {
            let trimmed_line = line.trim();
            if trimmed_line.len() == 0 {
                None
            } else {
                Some((Duration::from_millis(millis), trimmed_line.to_owned()))
            }
        })
        .collect()
}

pub fn play(
    filename: &String,
    use_alternate_screen: bool,
    lrc_filename: &Option<String>,
) -> Result<()> {
    let lrc_filename = match lrc_filename {
        Some(lrc_path) => Path::new(lrc_path).to_path_buf(),
        None => Path::new(filename).with_extension("lrc"),
    };
    if !lrc_filename.exists() {
        return Err(anyhow!(
            "LRC file does not exist: {}",
            lrc_filename.to_string_lossy()
        ));
    }
    let lyrics = lyrics_to_vec(parse_lrc(read_to_string(lrc_filename)?)?);
    if lyrics.is_empty() {
        return Err(anyhow!("LRC file contains no lyrics!"));
    }
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;
    let file = BufReader::new(File::open(filename)?);
    let source = Decoder::new(file)?;
    sink.append(source);
    sink.pause();
    let mut app = App {
        lyrics,
        sink,
        lyrics_lines_to_show: size()?.1 as usize / 2,
        first_lyrics_line: 0,
        curr_lyrics_line: 0,
        curr_word: 0,
        curr_syllable: 0,
    };
    if use_alternate_screen {
        execute!(stdout(), EnterAlternateScreen)?;
    }
    execute!(stdout(), Hide, DisableLineWrap)?;
    enable_raw_mode()?;
    let result = app.run();
    disable_raw_mode()?;
    execute!(stdout(), EnableLineWrap, Show)?;
    if use_alternate_screen {
        execute!(stdout(), LeaveAlternateScreen)?;
    }
    result
}
