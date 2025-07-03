/// Return advice on the pronunciation of the given jamo.
///
/// Returns an empty string if there is no advice.
///
/// Many of these hints are taken from the book "Hangeul
/// Master" by Talk to Me in Korean.
pub fn get_jamo_pronunciation(ch: char) -> &'static str {
    // TODO: Add other jamos
    // TODO: Might need to add `is_next_vowel` as arg.
    match ch {
        // Medial (vowel)
        'ᅡ' => "'a' as in 'father'",
        'ᅢ' => "'a' as in 'sad' or 'pan', indistinct fromᅦ",
        'ᅣ' => "",
        'ᅤ' => "",
        'ᅥ' => "'u' as in 'bus', 'gut', 'cup'",
        'ᅦ' => "'e' as in 'bed' or 'pet', indistinct fromᅢ",
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
