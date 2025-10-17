/// This module encapsulates the conversation from
/// Unit 2, "Greetings & Introductions", of
/// Active Korean 1 by the Language Education Institute
/// of Seoul National University, pg. 42.
use anyhow::{Result, anyhow};
use rand::seq::SliceRandom;
use rand::{Rng, thread_rng};
use rustyline::Editor;
use rustyline::history::FileHistory;
use tts::{Tts, Voice};

use crate::hangul::{
    HangulCharClass, compose_all_hangul_jamos, decompose_all_hangul_syllables,
    decompose_hangul_syllable_to_jamos,
};

const NAMES: [&str; 2] = ["김재민", "이미자"];

const COUNTRIES: [&str; 11] = [
    "미국",
    "중국",
    "일본",
    "인도",
    "호주",
    "영국",
    "독일",
    "프랑스",
    "캐나다",
    "한국",
    "러시아",
];

const OCCUPATIONS: [&str; 8] = [
    "선생님",
    "학생",
    "의사",
    "요리사",
    "은행원",
    "기자",
    "회사원",
    "연구원",
];

const REPEAT_COMMAND: &str = "뭐라고";
const SKIP_COMMAND: &str = "다음";

trait Speaker {
    fn speak(&mut self, text: &str) -> Result<()>;
}

struct StdoutSpeaker {
    name: String,
}

impl Speaker for StdoutSpeaker {
    fn speak(&mut self, text: &str) -> Result<()> {
        println!("{}: {}", self.name, text);
        Ok(())
    }
}

struct TtsSpeaker {
    name: String,
    tts: Tts,
    voice: Voice,
    rate: f32,
}

impl Speaker for TtsSpeaker {
    fn speak(&mut self, text: &str) -> Result<()> {
        println!("{}: {}", self.name, text);
        self.tts.set_rate(self.rate)?;
        self.tts.set_voice(&self.voice)?;
        self.tts.speak(text, true)?;
        #[cfg(target_os = "macos")]
        {
            use objc2_foundation::NSDate;
            let run_loop = objc2_foundation::NSRunLoop::currentRunLoop();
            loop {
                let future = NSDate::dateWithTimeIntervalSinceNow(2.0);
                run_loop.runUntilDate(&future);
                if !self.tts.is_speaking()? {
                    break;
                }
            }
        }
        Ok(())
    }
}

fn create_speaker<T: AsRef<str>>(
    name: String,
    preferred_voices: &[T],
    rate: Option<f32>,
) -> Box<dyn Speaker> {
    if let Ok(tts) = Tts::default() {
        let features = tts.supported_features();
        if features.is_speaking && features.voice && features.rate {
            if let Ok(voices) = tts.voices() {
                if let Some(voice) = preferred_voices.iter().find_map(|preferred_voice| {
                    for voice in &voices {
                        if voice.language() != "ko-KR" {
                            continue;
                        }
                        if preferred_voice.as_ref() == "*" {
                            return Some(voice.clone());
                        }
                        if voice.id() == preferred_voice.as_ref() {
                            return Some(voice.clone());
                        }
                    }
                    return None;
                }) {
                    let mut rate = rate.unwrap_or(tts.min_rate());
                    if rate < tts.min_rate() {
                        rate = tts.min_rate();
                    } else if rate > tts.max_rate() {
                        rate = tts.max_rate();
                    }
                    println!(
                        "Initializing TTS voice '{}' at rate {}.",
                        voice.name(),
                        rate
                    );
                    return Box::new(TtsSpeaker {
                        name,
                        tts,
                        voice,
                        rate,
                    });
                }
            }
        }
    }
    Box::new(StdoutSpeaker { name })
}

struct Conversation {
    is_interactive: bool,
    rl: Editor<(), FileHistory>,
    a: Box<dyn Speaker>,
    b: Box<dyn Speaker>,
}

impl Conversation {
    fn converse(&mut self, a_text: String, b_text: String) -> Result<()> {
        loop {
            self.a.speak(&a_text)?;
            if self.is_interactive {
                let line = get_hangul(self.rl.readline("> ")?);
                if line == REPEAT_COMMAND {
                    continue;
                } else if line == SKIP_COMMAND {
                    break;
                }
                let expected_line = get_hangul(&b_text);
                if line == expected_line {
                    println!("CORRECT RESPONSE!");
                } else {
                    println!("INCORRECT RESPONSE!");
                    println!("Expected: {expected_line}");
                    println!("Received: {line}");
                    self.a.speak(REPEAT_COMMAND)?;
                    continue;
                }
                println!("");
            } else {
                self.b.speak(&b_text)?;
            }
            break;
        }
        Ok(())
    }
}

fn get_hangul<T: AsRef<str>>(value: T) -> String {
    let normalized = compose_all_hangul_jamos(decompose_all_hangul_syllables(value.as_ref()));
    HangulCharClass::split(&normalized)
        .into_iter()
        .map(|(class, str)| {
            if class == HangulCharClass::None {
                ""
            } else {
                str
            }
        })
        .collect::<Vec<_>>()
        .join("")
}

fn run_introduction(c: &mut Conversation) -> Result<()> {
    let mut rng = thread_rng();

    let name = *NAMES.choose(&mut rng).unwrap();
    let country = *COUNTRIES.choose(&mut rng).unwrap();
    let occupation = *OCCUPATIONS.choose(&mut rng).unwrap();

    println!("Name: {name}");
    println!("Country: {country}");
    println!("Occupation: {occupation}");
    println!("\nTo repeat last line, say '뭐라고'.\n");

    c.converse(
        "안녕하세요?".into(),
        format!("안녕하세요? 저는 {name}{}.", get_copula(name)?),
    )?;

    let guessed_country = *guess(&COUNTRIES, &country)?;
    c.converse(
        format!("{name} 씨는 {guessed_country} 사람이에요?"),
        if guessed_country == country {
            format!("네, 저는 {country} 사람이에요.")
        } else {
            format!("아니요, 저는 {country} 사람이에요.")
        },
    )?;

    let guessed_occupation = *guess(&OCCUPATIONS, &occupation)?;
    c.converse(
        format!(
            "{name} 씨는 {guessed_occupation}{}?",
            get_copula(guessed_occupation)?
        ),
        if guessed_occupation == occupation {
            format!("네, 저는 {occupation}{}.", get_copula(occupation)?)
        } else {
            format!("아니요, 저는 {occupation}{}.", get_copula(occupation)?)
        },
    )?;

    Ok(())
}

pub fn run_introductions(rate: Option<f32>) -> Result<()> {
    let mut c = Conversation {
        a: create_speaker(
            "A".to_owned(),
            &[
                "com.apple.voice.premium.ko-KR.Yuna",
                "com.apple.voice.enhanced.ko-KR.Yuna",
                "com.apple.voice.compact.ko-KR.Yuna",
                "com.apple.eloquence.ko-KR.Grandma",
                "*",
            ],
            rate,
        ),
        b: create_speaker(
            "B".to_owned(),
            &[
                "com.apple.voice.enhanced.ko-KR.Minsu",
                "com.apple.voice.compact.ko-KR.Minsu",
                "com.apple.eloquence.ko-KR.Grandpa",
                "*",
            ],
            rate,
        ),
        rl: rustyline::DefaultEditor::new()?,
        is_interactive: true,
    };

    loop {
        run_introduction(&mut c)?;
        println!("LET'S DO ANOTHER ROUND.\n");
    }
}

fn guess<'a, T: AsRef<str> + PartialEq>(items: &'a [T], correct: &'a T) -> Result<&'a T> {
    let mut rng = thread_rng();
    let guess_correctly = rng.gen_bool(0.5);
    if guess_correctly {
        Ok(correct)
    } else {
        guess_other(items, correct)
    }
}

fn guess_other<'a, T: AsRef<str> + PartialEq>(items: &'a [T], except: &T) -> Result<&'a T> {
    let mut rng = thread_rng();
    let mut i = 0;
    loop {
        let Some(choice) = items.choose(&mut rng) else {
            return Err(anyhow!("items is empty"));
        };
        if choice != except {
            return Ok(choice);
        }
        i += 1;
        if i > 5000 {
            return Err(anyhow!("exceeded maximum attempts"));
        }
    }
}

fn ends_in_vowel<T: AsRef<str>>(value: T) -> Result<bool> {
    let Some(last_char) = value.as_ref().chars().last() else {
        return Err(anyhow!("string is empty"));
    };
    let Some((_initial, _vowel, final_consonant)) = decompose_hangul_syllable_to_jamos(last_char)
    else {
        return Err(anyhow!("final character is not a hangul syllable"));
    };
    Ok(final_consonant.is_none())
}

fn get_copula<T: AsRef<str>>(value: T) -> Result<&'static str> {
    if ends_in_vowel(value)? {
        Ok("예요")
    } else {
        Ok("이에요")
    }
}

#[cfg(test)]
mod tests {
    use crate::introductions::{ends_in_vowel, get_hangul};

    #[test]
    fn test_ends_in_vowel() {
        assert_eq!(ends_in_vowel("한").unwrap(), false);
        assert_eq!(ends_in_vowel("네").unwrap(), true);
    }

    #[test]
    fn test_get_hangul_works() {
        assert_eq!(get_hangul("네, 저는 의사예요"), "네저는의사예요");
    }
}
