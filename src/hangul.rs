#[derive(Debug, PartialEq)]
pub enum HangulCharClass {
    HangulCompatibilityJamo,
    HangulJamoExtendedA,
    HangulJamoExtendedB,
    HangulJamo,
    HangulSyllables,
    Other,
}

pub fn get_hangul_char_class(ch: char) -> HangulCharClass {
    match ch {
        '\u{ac00}'..='\u{d7af}' => HangulCharClass::HangulSyllables,
        '\u{1100}'..='\u{11ff}' => HangulCharClass::HangulJamo,
        '\u{3130}'..='\u{318f}' => HangulCharClass::HangulCompatibilityJamo,
        '\u{a960}'..='\u{a97f}' => HangulCharClass::HangulJamoExtendedA,
        '\u{d7b0}'..='\u{d7ff}' => HangulCharClass::HangulJamoExtendedB,
        _ => HangulCharClass::Other,
    }
}

impl From<char> for HangulCharClass {
    fn from(value: char) -> Self {
        get_hangul_char_class(value)
    }
}

pub fn decompose_hangul_syllable(ch: char) -> Option<(char, char, Option<char>)> {
    let class = get_hangul_char_class(ch);
    let codepoint = ch as u32;
    if class != HangulCharClass::HangulSyllables {
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
    assert_eq!(HangulCharClass::from(initial_ch), HangulCharClass::HangulJamo);
    assert_eq!(HangulCharClass::from(medial_ch), HangulCharClass::HangulJamo);
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

pub fn decompose_all_hangul_syllables<T: AsRef<str>>(value: T) -> String {
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
