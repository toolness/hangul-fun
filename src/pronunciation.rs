use crate::jamo_stream::{JamoInStream, JamoStream, ModernJamo};

/// Return advice on the pronunciation of the given jamo.
///
/// Returns an empty string if there is no advice.
///
/// Many of these hints are taken from the book "Hangeul
/// Master" by Talk to Me in Korean.
pub fn get_jamo_pronunciation(jamo: &JamoInStream) -> &'static str {
    // TODO: Support compound final consonants
    // TODO: Support linking sounds??
    match jamo.curr {
        // Initial, some finals
        'ᄀ' => "'g' as in 'go', not as in 'giraffe'",
        'ᄁ' | 'ᆩ' => "'ch' after 's', as in 'school'",
        'ᄂ' => "'n' as in 'no', with tongue touching back of teeth",
        'ᄃ' => "'d' as in 'study' or first 't' in start, tongue on back of teeth",
        'ᄄ' => "'t' after 's', as in 'steal'",
        'ᄅ' => "'l' at word begin, Spanish 'r' in middle, tongue back on palate",
        'ᄆ' => "'m' as in 'map'",
        'ᄇ' => "'b' as in 'busy', same mouth shape as ㅁ",
        'ᄈ' => "'p' after 's', as in 'speech'",
        'ᄉ' => "'s' as in 'slow' or 'sh' as in 'sheep'",
        'ᄊ' | 'ᆻ' => "'s' as in 'sit'",
        'ᄋ' => "silent",
        'ᄌ' => "'j' as in 'Jill'",
        'ᄍ' => "tighten throat while pronouncing ㅈ",
        'ᄎ' => "'ch' as in 'chain', like ㅈ aspirated",
        'ᄏ' => "'k' as in 'korea', like ㄱ aspirated",
        'ᄐ' => "'t' as in 'teeth', like ㄷ aspirated",
        'ᄑ' => "'p' as in 'power', like ㅂ aspirated",
        'ᄒ' => "'h' as in 'hat'",

        // Medial (vowel)
        'ᅡ' => "'a' as in 'father'",
        'ᅢ' => "'a' as in 'sad' or 'care', indistinct from ㅔ",
        'ᅣ' => "'ya' as in 'yarn', like ㅣ+ㅏ",
        'ᅤ' => "'ye' as in 'yes', like ㅣ+ㅐ, indistinct from ㅖ",
        'ᅥ' => "'u' as in 'bus', 'gut', 'cup'",
        'ᅦ' => "'e' as in 'bed' or 'a' as in 'take', indistinct from ㅐ",
        'ᅧ' => "'yu' as in 'yummy', like ㅣ+ㅓ",
        'ᅨ' => "'ye' as in 'yes', like ㅣ+ㅖ, indistinct from ㅒ",
        'ᅩ' => "'o' as in 'ago'",
        'ᅪ' => "'wa' as in 'swan', like ㅗ+ㅏ",
        'ᅫ' => "'we' as in 'wet', indistinct from ㅞ and ㅘ",
        'ᅬ' => "'we' as in 'wet', not 'oy', indistinct from ㅙ and ㅞ",
        'ᅭ' => "'yo' as in 'yogurt', like ㅣ+ㅗ",
        'ᅮ' => "'oo' as in 'food'",
        'ᅯ' => "'wo' as in 'wonderful' or 'work', like ㅗ+ㅓ",
        'ᅰ' => "'we' as in 'wet', indistinct from ㅘ and ㅙ",
        'ᅱ' => "'we' as in 'we' or 'weekend'",
        'ᅲ' => "'u' as in 'USA', like ㅣ+ㅜ",
        'ᅳ' => "'uh' with upper/lower teeth close and yucky face",
        'ᅴ' => "ㅣ with any consonant except ㅇ, otherwise ㅡ then ㅣ",
        'ᅵ' => "'ee' as in 'feet'",

        // Final (bat-chim)
        'ᆨ' | 'ᆿ' => "no sound, stop air like 'doc' in 'doctor'",
        'ᆫ' => "'n' as in 'can'",
        'ᆮ' | 'ᆺ' | 'ᆽ' | 'ᆾ' | 'ᇀ' | 'ᇂ' => "'t' as in 'cat', no puff of air at all",
        'ᆯ' => "'l' with tongue touching roof of mouth",
        'ᆷ' => "'m' as in 'beam'",
        'ᆸ' | 'ᇁ' => "'p' as in 'cap'",
        'ᆼ' => "'ng' as in 'ring'",

        _ => "",
    }
}

/// Compound consonant rules are defined in Talk To Me in Korean's
/// "Hangul Master" pg. 57-59.
///
/// Takes a final consonant and the next initial consonant after it
/// and returns the effective new final consonant and next initial
/// one.
fn apply_compound_consonant_rules(
    final_consonant: ModernJamo,
    next_initial_consonant: Option<ModernJamo>,
) -> (ModernJamo, Option<ModernJamo>) {
    use ModernJamo::*;
    match (final_consonant, next_initial_consonant) {
        (FinalConsonant('ᆪ'), Some(InitialConsonant('ᄋ'))) => {
            (FinalConsonant('ᆨ'), Some(InitialConsonant('ᄊ')))
        }
        (FinalConsonant('ᆪ'), _) => (FinalConsonant('ᆨ'), next_initial_consonant),
        // TODO: Add the rest of them.
        _ => (final_consonant, next_initial_consonant),
    }
}

pub fn apply_pronunciation_rules_to_jamos<T: AsRef<str>>(value: T) -> String {
    let mut result = String::with_capacity(value.as_ref().len());
    let mut skip_next_initial_consonant = false;
    for jamo in JamoStream::from_jamos(value) {
        match ModernJamo::try_from_char(jamo.curr) {
            Some(ModernJamo::InitialConsonant(ch)) => {
                if skip_next_initial_consonant {
                    skip_next_initial_consonant = false;
                } else {
                    result.push(ch);
                }
            }
            Some(ModernJamo::Vowel(ch)) => {
                result.push(ch);
            }
            Some(ModernJamo::FinalConsonant(ch)) => {
                let (final_consonant, next_initial_consonant) = apply_compound_consonant_rules(
                    ModernJamo::FinalConsonant(ch),
                    jamo.next
                        .map(|char| ModernJamo::try_from_char(char))
                        .flatten(),
                );
                result.push(final_consonant.into());
                if let Some(next_initial_consonant) = next_initial_consonant {
                    result.push(next_initial_consonant.into());
                    skip_next_initial_consonant = true;
                }
            }
            None => {
                result.push(jamo.curr);
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::pronunciation::apply_pronunciation_rules_to_jamos;

    #[test]
    fn test_compound_consonant_rules_work() {
        assert_eq!(
            apply_pronunciation_rules_to_jamos("넋을"),
            "넉쓸".to_owned()
        );
    }
}
