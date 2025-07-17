use crate::jamo_stream::{JamoInStream, JamoStream, ModernJamo};
use ModernJamo::*;

/// Return advice on the pronunciation of the given jamo.
///
/// Returns an empty string if there is no advice.
///
/// Many of these hints are taken from the book "Hangeul
/// Master" by Talk to Me in Korean.
pub fn get_jamo_pronunciation(jamo: &JamoInStream) -> &'static str {
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

struct RuleContext {
    /// The final consonant of one syllable.
    final_consonant: ModernJamo,
    /// The initial consonant of the next syllable.
    next_initial_consonant: Option<ModernJamo>,
}

impl RuleContext {
    fn consonants(&self) -> (ModernJamo, Option<ModernJamo>) {
        (self.final_consonant, self.next_initial_consonant)
    }
}

enum RuleResult {
    /// The rule doesn't apply to the given context.
    NoChange,
    /// The initial consonant of the next syllable should be changed.
    ChangeNextInitial(ModernJamo),
    /// The final consonant of the current syllable should be changed.
    ChangeFinal(ModernJamo),
    /// Both the final consonant of the current syllable and the initial
    /// consonant of the next one should be changed.
    ChangeBoth(ModernJamo, ModernJamo),
    /// The final consonant of the current syllable should be removed.
    RemoveFinal,
    /// The final consonant of the current syllable should be removed,
    /// and the initial consonant of the next syllable should be
    /// changed.
    RemoveFinalAndChangeNextInitial(ModernJamo),
}

/// Encapsulates a Hangul pronunciation rule.
type PronunciationRule = fn(&RuleContext) -> RuleResult;

/// Reinforcement/intensification rule as described here:
///
/// https://www.missellykorean.com/korean-sound-change-rules-pdf/
fn reinforcement_rule(ctx: &RuleContext) -> RuleResult {
    match ctx.consonants() {
        (
            // Note: it's very confusing which final consonants work
            // here, the instructions have conflicting information.
            FinalConsonant('ᆸ' | 'ᆨ' | 'ᆿ' | 'ᆮ' | 'ᆺ' | 'ᆽ' | 'ᆾ' | 'ᇀ'),
            Some(InitialConsonant(initial)),
        ) => {
            let strengthened = match initial {
                'ᄀ' => 'ᄁ',
                'ᄃ' => 'ᄄ',
                'ᄇ' => 'ᄈ',
                'ᄉ' => 'ᄊ',
                'ᄌ' => 'ᄍ',
                _ => return RuleResult::NoChange,
            };
            RuleResult::ChangeNextInitial(InitialConsonant(strengthened))
        }
        (FinalConsonant('ᇂ'), Some(InitialConsonant('ᄉ'))) => {
            // Note that the instructions only say to strengthen the initial
            // consonant, but the example given _removes_ the final, so
            // that's what I'll do here.
            RuleResult::RemoveFinalAndChangeNextInitial(InitialConsonant('ᄊ'))
        }
        _ => RuleResult::NoChange,
    }
}

/// Re-syllabification rule as described here:
///
/// https://www.missellykorean.com/korean-sound-change-rules-pdf/
fn resyllabification_rule(ctx: &RuleContext) -> RuleResult {
    match ctx.consonants() {
        (FinalConsonant(ch), Some(InitialConsonant('ᄋ'))) => {
            let new_initial = match ch {
                'ᆨ' => 'ᄀ',
                'ᆩ' => 'ᄁ',
                'ᆫ' => 'ᄂ',
                'ᆮ' => 'ᄃ',
                'ᆯ' => 'ᄅ',
                'ᆷ' => 'ᄆ',
                'ᆸ' => 'ᄇ',
                'ᆺ' => 'ᄉ',
                'ᆻ' => 'ᄊ',
                'ᆼ' => return RuleResult::NoChange,
                'ᆽ' => 'ᄌ',
                'ᆾ' => 'ᄎ',
                'ᆿ' => 'ᄏ',
                'ᇀ' => 'ᄐ',
                'ᇁ' => 'ᄑ',
                'ᇂ' => return RuleResult::RemoveFinal,
                _ => return RuleResult::NoChange,
            };
            RuleResult::RemoveFinalAndChangeNextInitial(ModernJamo::InitialConsonant(new_initial))
        }
        _ => RuleResult::NoChange,
    }
}

/// Compound consonant rules are defined in Talk To Me in Korean's
/// "Hangul Master" pg. 57-59.
///
/// Takes a final consonant and the next initial consonant after it
/// and returns the effective new final consonant and next initial
/// one.
fn compound_consonant_rule(ctx: &RuleContext) -> RuleResult {
    let orig_next_initial = ctx.next_initial_consonant;
    let (new_final, new_next_initial) = match ctx.consonants() {
        // Rules for ㄳ
        (FinalConsonant('ᆪ'), Some(InitialConsonant('ᄋ'))) => {
            (FinalConsonant('ᆨ'), Some(InitialConsonant('ᄉ')))
        }
        (FinalConsonant('ᆪ'), _) => (FinalConsonant('ᆨ'), orig_next_initial),

        // Rules for ㄵ
        (FinalConsonant('ᆬ'), Some(InitialConsonant('ᄋ'))) => {
            (FinalConsonant('ᆫ'), Some(InitialConsonant('ᄌ')))
        }
        (FinalConsonant('ᆬ'), _) => (FinalConsonant('ᆫ'), orig_next_initial),

        // Rules for ㄶ
        (FinalConsonant('ᆭ'), Some(InitialConsonant('ᄀ'))) => {
            (FinalConsonant('ᆫ'), Some(InitialConsonant('ᄏ')))
        }
        (FinalConsonant('ᆭ'), Some(InitialConsonant('ᄃ'))) => {
            (FinalConsonant('ᆫ'), Some(InitialConsonant('ᄐ')))
        }
        (FinalConsonant('ᆭ'), Some(InitialConsonant('ᄌ'))) => {
            (FinalConsonant('ᆫ'), Some(InitialConsonant('ᄎ')))
        }
        (FinalConsonant('ᆭ'), _) => (FinalConsonant('ᆫ'), orig_next_initial),

        // Rules for ㄺ
        (FinalConsonant('ᆰ'), Some(InitialConsonant('ᄋ'))) => {
            (FinalConsonant('ᆯ'), Some(InitialConsonant('ᄀ')))
        }
        (FinalConsonant('ᆰ'), Some(InitialConsonant('ᄀ'))) => {
            (FinalConsonant('ᆯ'), Some(InitialConsonant('ᄁ')))
        }
        (FinalConsonant('ᆰ'), _) => (FinalConsonant('ᆨ'), orig_next_initial),

        // Rules for ㄻ
        (FinalConsonant('ᆱ'), Some(InitialConsonant('ᄋ'))) => {
            (FinalConsonant('ᆯ'), Some(InitialConsonant('ᄆ')))
        }
        (FinalConsonant('ᆱ'), _) => (FinalConsonant('ᆷ'), orig_next_initial),

        // Rules for ㄼ
        (FinalConsonant('ᆲ'), Some(InitialConsonant('ᄋ'))) => {
            (FinalConsonant('ᆯ'), Some(InitialConsonant('ᄇ')))
        }
        (FinalConsonant('ᆲ'), Some(InitialConsonant('ᄃ'))) => {
            (FinalConsonant('ᆸ'), Some(InitialConsonant('ᄃ')))
        }
        (FinalConsonant('ᆲ'), _) => (FinalConsonant('ᆯ'), orig_next_initial),

        // Rules for ㄾ
        (FinalConsonant('ᆴ'), Some(InitialConsonant('ᄋ'))) => {
            (FinalConsonant('ᆯ'), Some(InitialConsonant('ᄐ')))
        }
        (FinalConsonant('ᆴ'), _) => (FinalConsonant('ᆯ'), orig_next_initial),

        // Rules for ㄽ
        (FinalConsonant('ᆳ'), Some(InitialConsonant('ᄋ'))) => {
            // It's unclear whether the reinforcement rule applies here; since
            // we don't currently match it on ᆯ, we'll do it here manually,
            // because that's what the example in the book has.
            (FinalConsonant('ᆯ'), Some(InitialConsonant('ᄊ')))
        }
        (FinalConsonant('ᆳ'), _) => (FinalConsonant('ᆯ'), orig_next_initial),

        // Rules for ㄿ
        (FinalConsonant('ᆵ'), Some(InitialConsonant('ᄋ'))) => {
            (FinalConsonant('ᆯ'), Some(InitialConsonant('ᄑ')))
        }
        (FinalConsonant('ᆵ'), _) => (FinalConsonant('ᆸ'), orig_next_initial),

        // Rules for ㅀ
        (FinalConsonant('ᆶ'), Some(InitialConsonant('ᄀ'))) => {
            (FinalConsonant('ᆯ'), Some(InitialConsonant('ᄏ')))
        }
        (FinalConsonant('ᆶ'), Some(InitialConsonant('ᄃ'))) => {
            (FinalConsonant('ᆯ'), Some(InitialConsonant('ᄐ')))
        }
        (FinalConsonant('ᆶ'), Some(InitialConsonant('ᄌ'))) => {
            (FinalConsonant('ᆯ'), Some(InitialConsonant('ᄎ')))
        }
        (FinalConsonant('ᆶ'), _) => (FinalConsonant('ᆯ'), orig_next_initial),

        // Rules for ㅄ
        (FinalConsonant('ᆹ'), Some(InitialConsonant('ᄋ'))) => {
            (FinalConsonant('ᆸ'), Some(InitialConsonant('ᄉ')))
        }
        (FinalConsonant('ᆹ'), _) => (FinalConsonant('ᆸ'), orig_next_initial),

        _ => return RuleResult::NoChange,
    };

    // TODO: Change all of the above code to return RuleResult directly. It
    // was written before the introduction of RuleResult and was easier to just
    // add the below logic than fix everything, especially since I don't know if I'll
    // stick with RuleResult in the long term.
    if new_next_initial == orig_next_initial {
        RuleResult::ChangeFinal(new_final)
    } else if let Some(new_next_initial) = new_next_initial {
        RuleResult::ChangeBoth(new_final, new_next_initial)
    } else {
        RuleResult::ChangeFinal(new_final)
    }
}

/// All pronunciation rules required for Hangul, in the order that they
/// should be applied.
const PRONUNCIATION_RULES: [PronunciationRule; 3] = [
    compound_consonant_rule,
    resyllabification_rule,
    reinforcement_rule,
];

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
                let mut ctx = RuleContext {
                    final_consonant: ModernJamo::FinalConsonant(ch),
                    next_initial_consonant: jamo
                        .next
                        .map(|char| ModernJamo::try_from_char(char))
                        .flatten(),
                };
                let mut keep_final_consonant = true;
                for rule in PRONUNCIATION_RULES {
                    let result = rule(&ctx);
                    match result {
                        RuleResult::NoChange => {}
                        RuleResult::ChangeNextInitial(next_initial_consonant) => {
                            ctx.next_initial_consonant = Some(next_initial_consonant);
                        }
                        RuleResult::ChangeFinal(final_consonant) => {
                            ctx.final_consonant = final_consonant;
                        }
                        RuleResult::ChangeBoth(final_consonant, next_initial_consonant) => {
                            ctx.final_consonant = final_consonant;
                            ctx.next_initial_consonant = Some(next_initial_consonant);
                        }
                        RuleResult::RemoveFinal => {
                            keep_final_consonant = false;
                            break;
                        }
                        RuleResult::RemoveFinalAndChangeNextInitial(next_initial_consonant) => {
                            keep_final_consonant = false;
                            ctx.next_initial_consonant = Some(next_initial_consonant);
                            break;
                        }
                    }
                }
                if keep_final_consonant {
                    result.push(ctx.final_consonant.into());
                }
                if let Some(next_initial_consonant) = ctx.next_initial_consonant {
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
    use crate::{
        hangul::{compose_all_hangul_jamos, decompose_all_hangul_syllables},
        pronunciation::apply_pronunciation_rules_to_jamos,
    };

    fn apply_syllables(value: &'static str) -> String {
        let jamos = decompose_all_hangul_syllables(value);
        compose_all_hangul_jamos(apply_pronunciation_rules_to_jamos(jamos))
    }

    fn test_pronounce(original: &'static str, pronounced: &'static str) {
        assert_eq!(apply_syllables(original), pronounced.to_owned())
    }

    #[test]
    fn test_compound_consonant_rules_work() {
        assert_eq!(
            apply_pronunciation_rules_to_jamos("넋을"),
            "넉쓸".to_owned()
        );
    }

    #[test]
    fn test_reinforcement_rules_work() {
        test_pronounce("학교", "학꾜");
        test_pronounce("학생", "학쌩");
        test_pronounce("잡지", "잡찌");
        test_pronounce("먹다", "먹따");
        test_pronounce("좋습니다", "조씁니다");
    }

    #[test]
    fn test_resyllibification_rules_work() {
        test_pronounce("십오", "시보");
        // Ensure ng does not carry over.
        test_pronounce("생일", "생일");
        // Ensure h is silent.
        test_pronounce("좋아", "조아");
    }
}
