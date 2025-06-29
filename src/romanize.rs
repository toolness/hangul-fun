/// Used for any jamos we don't know how to pronounce/romanize.
///
/// We should get rid of these but I don't actually know how to
/// pronounce/romanize them!
///
/// Claude says these are archaic and no longer used, but they're
/// definitely present in LRC files, and they're also highlighted
/// in green on this page, indicating that they are modern-usage
/// characters:
///
///     https://en.wikipedia.org/wiki/Hangul#Unicode
///
/// However, none of the Hangul tutorials actually have anything
/// to say about them.
const UNKNOWN: &'static str = "?";

/// Get the romanization of a final consonant, when there is no vowel following it.
fn get_final_with_no_next_vowel(ch: char) -> Option<&'static str> {
    match ch {
        // Final
        'ᆨ' => Some("k"),
        'ᆩ' => Some("k"),
        'ᆪ' => Some(UNKNOWN),
        'ᆫ' => Some("n"),
        'ᆬ' => Some(UNKNOWN),
        'ᆭ' => Some(UNKNOWN),
        'ᆮ' => Some("t"),
        'ᆯ' => Some("l"),
        'ᆰ' => Some(UNKNOWN),
        'ᆱ' => Some(UNKNOWN),
        'ᆲ' => Some(UNKNOWN),
        'ᆳ' => Some(UNKNOWN),
        'ᆴ' => Some(UNKNOWN),
        'ᆵ' => Some(UNKNOWN),
        'ᆶ' => Some(UNKNOWN),
        'ᆷ' => Some("m"),
        'ᆸ' => Some("p"),
        'ᆹ' => Some(UNKNOWN),
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

/// Get the romanization of a final consonant, when there is a vowel following it.
fn get_final_with_next_vowel(ch: char) -> Option<&'static str> {
    match ch {
        // Final
        'ᆨ' => Some("g"),
        'ᆩ' => Some("kk"),
        'ᆪ' => Some(UNKNOWN),
        'ᆫ' => Some("n"),
        'ᆬ' => Some(UNKNOWN),
        'ᆭ' => Some(UNKNOWN),
        'ᆮ' => Some("d"),
        'ᆯ' => Some("l"),
        'ᆰ' => Some(UNKNOWN),
        'ᆱ' => Some(UNKNOWN),
        'ᆲ' => Some(UNKNOWN),
        'ᆳ' => Some(UNKNOWN),
        'ᆴ' => Some(UNKNOWN),
        'ᆵ' => Some(UNKNOWN),
        'ᆶ' => Some(UNKNOWN),
        'ᆷ' => Some("m"),
        'ᆸ' => Some("b"),
        'ᆹ' => Some(UNKNOWN),
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

/// Get the romanization of a Hangul jamo.
///
/// `is_next_vowel` represents whether the syllable
/// following the final consonant of this syllable is
/// a vowel.
pub fn get_romanized_jamo(ch: char, is_next_vowel: bool) -> Option<&'static str> {
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
                get_final_with_next_vowel(ch)
            } else {
                get_final_with_no_next_vowel(ch)
            }
        }
    }
}

/// Romanizes the given sequence of Hangul jamos.
///
/// (These should _not_ be Hangul syllables!)
pub fn romanize_decomposed_hangul<T: AsRef<str>>(value: T) -> String {
    let mut result = String::with_capacity(value.as_ref().len());
    let mut prev_char: Option<char> = None;
    for char in value.as_ref().chars().chain(" ".chars()) {
        if let Some(prev_char) = prev_char {
            let is_next_vowel = char == 'ᄋ';
            if let Some(romanized) = get_romanized_jamo(prev_char, is_next_vowel) {
                result.push_str(romanized);
            } else {
                result.push(prev_char);
            }
        }
        prev_char = Some(char);
    }
    result
}

#[cfg(test)]
mod test {
    use crate::romanize::romanize_decomposed_hangul;

    #[test]
    fn test_romanize_works() {
        assert_eq!(romanize_decomposed_hangul("밥"), "bap".to_owned());
        // Liason/linking converts the 'p' to a 'b'.
        assert_eq!(romanize_decomposed_hangul("밥을"), "babeul".to_owned());
    }

    #[test]
    fn test_non_hangul_is_unchanged() {
        assert_eq!(romanize_decomposed_hangul("hi"), "hi".to_owned());
    }
}
