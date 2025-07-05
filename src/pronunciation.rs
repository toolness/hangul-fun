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
        'ᄇ' | 'ᆸ' => "'b' as in 'busy'",
        'ᄉ' | 'ᆺ' => "'s' as in 'slow' or 'sh' as in 'sheep'",
        'ᄋ' | 'ᆼ' => "silent in front of vowel, after vowel 'ng' as in 'song'",
        'ᄌ' | 'ᆽ' => "'j' as in 'Jill'",

        // Medial (vowel)
        'ᅡ' => "'a' as in 'father'",
        'ᅢ' => "'a' as in 'sad' or 'care', indistinct from ㅔ",
        'ᅣ' => "",
        'ᅤ' => "",
        'ᅥ' => "'u' as in 'bus', 'gut', 'cup'",
        'ᅦ' => "'e' as in 'bed' or 'a' as in 'take', indistinct from ㅐ",
        'ᅧ' => "",
        'ᅨ' => "",
        'ᅩ' => "'o' as in 'ago'",
        'ᅪ' => "",
        'ᅫ' => "",
        'ᅬ' => "",
        'ᅭ' => "",
        'ᅮ' => "'oo' as in 'food'",
        'ᅯ' => "",
        'ᅰ' => "",
        'ᅱ' => "",
        'ᅲ' => "",
        'ᅳ' => "'uh' with upper/lower teeth close and yucky face",
        'ᅴ' => "",
        'ᅵ' => "'ee' as in 'feet'",
        _ => "",
    }
}
