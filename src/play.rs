use anyhow::{Result, anyhow};
use crossterm::{
    QueueableCommand,
    cursor::{Hide, MoveTo, MoveToNextLine, Show},
    event::{Event, KeyCode, read},
    execute,
    style::Print,
    terminal::{
        Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode,
        enable_raw_mode, size,
    },
};
use lrc::Lyrics;
use rodio::{Decoder, OutputStream, Sink};
use std::{
    fs::{File, read_to_string},
    io::{BufReader, Write, stdout},
    path::Path,
    time::Duration,
};

struct App {
    lyrics_lines_to_show: usize,
    first_lyrics_line: usize,
    curr_lyrics_line: usize,
    lyrics: Lyrics,
    sink: Sink,
}

impl App {
    pub fn run(&mut self) -> Result<()> {
        loop {
            self.render()?;
            let event = read()?;
            if event == Event::Key(KeyCode::Esc.into()) {
                break;
            } else if event == Event::Key(KeyCode::Char(' ').into()) {
                self.toggle_pause();
            } else if event == Event::Key(KeyCode::Down.into()) {
                self.go_to_next_line();
            } else if event == Event::Key(KeyCode::Up.into()) {
                self.go_to_prev_line();
            }
        }

        Ok(())
    }

    pub fn render(&self) -> Result<()> {
        let mut stdout = stdout();
        let width = size()?.0;
        stdout.queue(Clear(ClearType::All))?;
        stdout.queue(MoveTo(0, 0))?;
        let lyrics = self.lyrics.get_timed_lines();
        let mut i = self.first_lyrics_line;
        loop {
            let Some((_, line)) = lyrics.get(i) else {
                break;
            };
            stdout.queue(Print(if i == self.curr_lyrics_line {
                "> "
            } else {
                "  "
            }))?;
            stdout.queue(Print(&line))?;
            stdout.queue(MoveToNextLine(1))?;
            i += 1;
            if i >= self.first_lyrics_line + self.lyrics_lines_to_show {
                break;
            }
        }
        stdout.flush()?;
        Ok(())
    }

    pub fn go_to_next_line(&mut self) {
        if self.curr_lyrics_line + 1 < self.lyrics.get_timed_lines().len() {
            self.curr_lyrics_line += 1;
            if self.first_lyrics_line + self.lyrics_lines_to_show <= self.curr_lyrics_line {
                self.first_lyrics_line += 1;
            }
        }
    }

    pub fn go_to_prev_line(&mut self) {
        if self.curr_lyrics_line > 0 {
            self.curr_lyrics_line -= 1;
            if self.first_lyrics_line > self.curr_lyrics_line {
                self.first_lyrics_line = self.curr_lyrics_line;
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
}

pub fn play(filename: &String) -> Result<()> {
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
    let sink = Sink::try_new(&stream_handle)?;
    let file = BufReader::new(File::open(filename)?);
    let source = Decoder::new(file)?;
    sink.append(source);
    sink.try_seek(Duration::from_secs_f32(0.0)).unwrap();
    execute!(stdout(), EnterAlternateScreen, Hide)?;
    enable_raw_mode()?;
    let mut app = App {
        lyrics,
        sink,
        lyrics_lines_to_show: size()?.1 as usize / 2,
        first_lyrics_line: 0,
        curr_lyrics_line: 0,
    };
    let result = app.run();
    disable_raw_mode()?;
    execute!(stdout(), Show, LeaveAlternateScreen)?;
    if let Err(err) = &result {
        eprintln!("{}", err);
    }
    result
}
