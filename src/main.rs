use std::env::args;

use crate::romanize::romanize_decomposed_hangul;

mod romanize;

#[derive(Debug, PartialEq)]
enum CharClass {
    HangulCompatibilityJamo,
    HangulJamoExtendedA,
    HangulJamoExtendedB,
    HangulJamo,
    HangulSyllables,
    Other,
}

fn get_char_class(ch: char) -> CharClass {
    match ch {
        '\u{ac00}'..='\u{d7af}' => CharClass::HangulSyllables,
        '\u{1100}'..='\u{11ff}' => CharClass::HangulJamo,
        '\u{3130}'..='\u{318f}' => CharClass::HangulCompatibilityJamo,
        '\u{a960}'..='\u{a97f}' => CharClass::HangulJamoExtendedA,
        '\u{d7b0}'..='\u{d7ff}' => CharClass::HangulJamoExtendedB,
        _ => CharClass::Other,
    }
}

impl From<char> for CharClass {
    fn from(value: char) -> Self {
        get_char_class(value)
    }
}

fn print_char_info(ch: char) {
    let class = get_char_class(ch);
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

fn decompose_hangul_syllable(ch: char) -> Option<(char, char, Option<char>)> {
    let class = get_char_class(ch);
    let codepoint = ch as u32;
    if class != CharClass::HangulSyllables {
        return None;
    }
    let base_codepoint = codepoint - 0xac00;
    let initial_codepoint_idx = base_codepoint / 588;
    let medial_codepoint_idx = (base_codepoint - (initial_codepoint_idx * 588)) / 28;
    let final_codepoint_idx =
        base_codepoint - (initial_codepoint_idx * 588) - (medial_codepoint_idx * 28);
    let initial_codepoint = 0x1100 + initial_codepoint_idx;
    let medial_codepoint = 0x1161 + medial_codepoint_idx;
    let final_codepoint = 0x11a7 + final_codepoint_idx;
    let initial_ch = char::from_u32(initial_codepoint).unwrap();
    let medial_ch = char::from_u32(medial_codepoint).unwrap();
    let maybe_final_ch = if final_codepoint_idx == 0 {
        None
    } else {
        char::from_u32(final_codepoint)
    };
    assert_eq!(CharClass::from(initial_ch), CharClass::HangulJamo);
    assert_eq!(CharClass::from(medial_ch), CharClass::HangulJamo);
    Some((initial_ch, medial_ch, maybe_final_ch))
}

fn hangul_syllable_to_jamos(ch: char) -> Option<String> {
    if let Some((initial_ch, medial_ch, maybe_final_ch)) = decompose_hangul_syllable(ch) {
        if let Some(final_ch) = maybe_final_ch {
            Some(format!("{initial_ch}{medial_ch}{final_ch}"))
        } else {
            Some(format!("{initial_ch}{medial_ch}"))
        }
    } else {
        None
    }
}

fn decompose_all_hangul_syllables<T: AsRef<str>>(value: T) -> String {
    let str = value.as_ref();
    let mut result = String::with_capacity(str.len());

    for ch in str.chars() {
        if let Some(jamos) = hangul_syllable_to_jamos(ch) {
            result.push_str(&jamos);
        } else {
            result.push(ch);
        }
    }

    result
}

fn main() {
    let str = args().skip(1).next().unwrap_or("밥을".to_owned());
    for ch in str.chars() {
        print_char_info(ch);
    }
    let decomposed = decompose_all_hangul_syllables(&str);
    println!(
        "{decomposed} (original length={}, decomposed length={})",
        str.len(),
        decomposed.len()
    );
    println!("romanized: {}", romanize_decomposed_hangul(&decomposed));
}
