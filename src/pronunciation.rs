/// Return advice on the pronunciation of the given jamo.
///
/// Returns an empty string if there is no advice.
///
/// Many of these hints are taken from the book "Hangeul
/// Master" by Talk to Me in Korean.
pub fn get_jamo_pronunciation(ch: char) -> &'static str {
    // TODO: Add other jamos
    match ch {
        // Initial/final
        'ᄀ' | 'ᆨ' => "'g' as in 'go', not as in 'giraffe'",
        'ᄂ' | 'ᆫ' => "'n' as in 'no', with tongue touching back of teeth",
        'ᄃ' | 'ᆮ' => "'d' as in 'study' or first 't' in start, tongue on back of teeth",
        'ᄅ' | 'ᆯ' => "'l' at word begin, Spanish 'r' in middle, tongue back on palate",
        'ᄆ' | 'ᆷ' => "'m' as in 'map'",
        'ᄇ' | 'ᆸ' => "'b' as in 'busy', same mouth shape as ㅁ",
        'ᄉ' | 'ᆺ' => "'s' as in 'slow' or 'sh' as in 'sheep'",
        'ᄋ' | 'ᆼ' => "silent in front of vowel, after vowel 'ng' as in 'song'",
        'ᄌ' | 'ᆽ' => "'j' as in 'Jill'",
        'ᄎ' | 'ᆾ' => "'ch' as in 'chain', like ㅈ aspirated",
        'ᄏ' | 'ᆿ' => "'k' as in 'korea', like ㄱ aspirated",
        'ᄐ' | 'ᇀ' => "'t' as in 'teeth', like ㄷ aspirated",
        'ᄑ' | 'ᇁ' => "'p' as in 'power', like ㅂ aspirated",
        'ᄒ' | 'ᇂ' => "'h' as in 'hat'",

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

        _ => "",
    }
}
