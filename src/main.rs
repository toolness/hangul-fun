use std::env::args;

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

fn get_final_no_next_vowel(ch: char) -> Option<&'static str> {
    match ch {
        // Final
        'ᆨ' => Some("k"),
        'ᆩ' => Some("k"),
        'ᆪ' => Some("?"),
        'ᆫ' => Some("n"),
        'ᆬ' => Some("?"),
        'ᆭ' => Some("?"),
        'ᆮ' => Some("t"),
        'ᆯ' => Some("l"),
        'ᆰ' => Some("?"),
        'ᆱ' => Some("?"),
        'ᆲ' => Some("?"),
        'ᆳ' => Some("?"),
        'ᆴ' => Some("?"),
        'ᆵ' => Some("?"),
        'ᆶ' => Some("?"),
        'ᆷ' => Some("m"),
        'ᆸ' => Some("p"),
        'ᆹ' => Some("?"),
        'ᆺ' => Some("t"),
        'ᆻ' => Some("t"),
        'ᆼ' => Some("ng"),
        'ᆽ' => Some("t"),
        'ᆾ' => Some("t"),
        'ᆿ' => Some("k"),
        'ᇀ' => Some("t"),
        'ᇁ' => Some("p"),
        'ᇂ' => Some("t"),
        _ => None,
    }
}

fn get_final_next_vowel(ch: char) -> Option<&'static str> {
    match ch {
        // Final
        'ᆨ' => Some("g"),
        'ᆩ' => Some("kk"),
        'ᆪ' => Some("?"),
        'ᆫ' => Some("n"),
        'ᆬ' => Some("?"),
        'ᆭ' => Some("?"),
        'ᆮ' => Some("d"),
        'ᆯ' => Some("l"),
        'ᆰ' => Some("?"),
        'ᆱ' => Some("?"),
        'ᆲ' => Some("?"),
        'ᆳ' => Some("?"),
        'ᆴ' => Some("?"),
        'ᆵ' => Some("?"),
        'ᆶ' => Some("?"),
        'ᆷ' => Some("m"),
        'ᆸ' => Some("b"),
        'ᆹ' => Some("?"),
        'ᆺ' => Some("s"),
        'ᆻ' => Some("ss"),
        'ᆼ' => Some("ng"),
        'ᆽ' => Some("j"),
        'ᆾ' => Some("ch"),
        'ᆿ' => Some("k"),
        'ᇀ' => Some("t"),
        'ᇁ' => Some("p"),
        'ᇂ' => Some("h"),
        _ => None,
    }
}

fn get_romanized(ch: char, is_next_vowel: bool) -> Option<&'static str> {
    match ch {
        // Initial
        'ᄀ' => Some("g"),
        'ᄁ' => Some("kk"),
        'ᄂ' => Some("n"),
        'ᄃ' => Some("d"),
        'ᄄ' => Some("tt"),
        'ᄅ' => Some("r"),
        'ᄆ' => Some("m"),
        'ᄇ' => Some("b"),
        'ᄈ' => Some("pp"),
        'ᄉ' => Some("s"),
        'ᄊ' => Some("ss"),
        'ᄋ' => Some(""), // silent
        'ᄌ' => Some("j"),
        'ᄍ' => Some("jj"),
        'ᄎ' => Some("ch"),
        'ᄏ' => Some("k"),
        'ᄐ' => Some("t"),
        'ᄑ' => Some("p"),
        'ᄒ' => Some("h"),

        // Medial
        'ᅡ' => Some("a"),
        'ᅢ' => Some("ae"),
        'ᅣ' => Some("ya"),
        'ᅤ' => Some("yae"),
        'ᅥ' => Some("eo"),
        'ᅦ' => Some("e"),
        'ᅧ' => Some("yeo"),
        'ᅨ' => Some("ye"),
        'ᅩ' => Some("o"),
        'ᅪ' => Some("wa"),
        'ᅫ' => Some("wae"),
        'ᅬ' => Some("oe"),
        'ᅭ' => Some("yo"),
        'ᅮ' => Some("u"),
        'ᅯ' => Some("wo"),
        'ᅰ' => Some("we"),
        'ᅱ' => Some("wi"),
        'ᅲ' => Some("yu"),
        'ᅳ' => Some("eu"),
        'ᅴ' => Some("ui"),
        'ᅵ' => Some("i"),
        
        _ => {
            if is_next_vowel {
                get_final_next_vowel(ch)
            } else {
                get_final_no_next_vowel(ch)
            }
        }
    }
}

fn decompose_hangul(ch: char) {
    let class = get_char_class(ch);
    let codepoint = ch as u32;
    let start = format!("ch={ch} ({codepoint:#x}) {class:?}");
    if class != CharClass::HangulSyllables {
        println!("{start}");
        return;
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
    let final_info = if let Some(final_ch) = maybe_final_ch {
        format!(" final={final_ch} ({final_codepoint:#x})")
    } else {
        String::default()
    };
    println!(
        "{start} initial={initial_ch} ({initial_codepoint:#x}) medial={medial_ch} ({medial_codepoint:#x}){final_info}"
    );
}

fn main() {
    if let Some(arg) = args().skip(1).next() {
        for ch in arg.chars() {
            decompose_hangul(ch);
        }
    } else {
        let chars = vec!['한', '가', '꿈'];
        for char in chars {
            decompose_hangul(char);
        }
    }
}
