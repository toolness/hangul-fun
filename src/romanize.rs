use crate::jamo_stream::{JamoInStream, JamoStream};

/// Get the romanization of a final consonant, when there is no vowel following it.
fn get_final_with_no_next_vowel(ch: char) -> Option<&'static str> {
    match ch {
        // Final
        'ᆨ' => Some("k"),
        'ᆩ' => Some("k"),
        'ᆫ' => Some("n"),
        'ᆮ' => Some("t"),
        'ᆯ' => Some("l"),
        'ᆷ' => Some("m"),
        'ᆸ' => Some("p"),
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
        'ᆫ' => Some("n"),
        'ᆮ' => Some("d"),
        'ᆯ' => Some("l"),
        'ᆷ' => Some("m"),
        'ᆸ' => Some("b"),
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
///
/// Note that compound consonants are unsupported;
/// pronunciation rules should first process the
/// jamos, which will convert compound consonants
/// to standard jamos.
pub fn get_romanized_jamo(jamo: &JamoInStream) -> Option<&'static str> {
    match jamo.curr {
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

        // Medial (vowel)
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
            if jamo.is_final_consonant_followed_by_vowel() {
                get_final_with_next_vowel(jamo.curr)
            } else {
                get_final_with_no_next_vowel(jamo.curr)
            }
        }
    }
}

/// Romanizes the given sequence of Hangul jamos.
///
/// (These should _not_ be Hangul syllables!)
pub fn romanize_decomposed_hangul<T: AsRef<str>>(value: T) -> String {
    let mut result = String::with_capacity(value.as_ref().len());
    let stream = JamoStream::from_jamos(value);
    for jamo in stream {
        if let Some(romanized) = get_romanized_jamo(&jamo) {
            result.push_str(romanized);
        } else {
            result.push(jamo.curr);
        }
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
