use crate::hangul::compose_hangul_jamos_to_syllable;

#[derive(PartialEq, Debug)]
pub struct JamoInStream {
    pub curr: char,
    pub prev: Option<char>,
    pub next: Option<char>,
    pub next_syllable: Option<char>,
}

impl JamoInStream {
    pub fn is_final_consonant_followed_by_vowel(&self) -> bool {
        // This assumes our stream is a well-formed sequence of Jamos.
        self.next == Some('ᄋ')
    }
}

pub struct JamoStream {
    jamos: Vec<char>,
    syllable_indices: Vec<usize>,
    index: usize,
    syllable_index: usize,
}

impl JamoStream {
    pub fn from_hangul_syllables<T: AsRef<str>>(value: T) -> Self {
        use crate::hangul::decompose_all_hangul_syllables;
        Self::from_jamos(decompose_all_hangul_syllables(value))
    }

    pub fn from_jamos<T: AsRef<str>>(value: T) -> Self {
        let jamos: Vec<char> = value.as_ref().chars().collect();
        let mut syllable_indices = Vec::with_capacity(jamos.len() / 2);
        for (index, jamo) in jamos.iter().enumerate() {
            if ModernJamo::is_initial_consonant(*jamo) {
                syllable_indices.push(index);
            }
        }

        Self {
            jamos,
            syllable_indices,
            index: 0,
            syllable_index: 0,
        }
    }

    pub fn seek_to_syllable(&mut self, index: usize) {
        if let Some(&jamo_index) = self.syllable_indices.get(index) {
            self.index = jamo_index;
        }
    }

    fn get_syllable_at(&mut self, index: usize) -> Option<char> {
        let Some(&jamo_start_index) = self.syllable_indices.get(index) else {
            return None;
        };
        let slice = match self.syllable_indices.get(index + 1) {
            Some(&jamo_end_index) => &self.jamos[jamo_start_index..jamo_end_index],
            None => &self.jamos[jamo_start_index..],
        };
        compose_hangul_jamos_to_syllable(slice.iter().cloned())
    }
}

impl Iterator for JamoStream {
    type Item = JamoInStream;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(&curr) = self.jamos.get(self.index) else {
            return None;
        };
        let prev = if self.index == 0 {
            None
        } else {
            self.jamos.get(self.index - 1).cloned()
        };
        let (next, next_syllable) = match self.jamos.get(self.index + 1) {
            Some(&next) => {
                let next_syllable = self.get_syllable_at(self.syllable_index + 1);
                if let Some(ModernJamo::InitialConsonant(_)) = ModernJamo::try_from_char(next) {
                    self.syllable_index += 1;
                }
                (Some(next), next_syllable)
            }
            None => {
                self.syllable_index += 1;
                (None, None)
            }
        };
        self.index += 1;
        Some(JamoInStream {
            curr,
            prev,
            next,
            next_syllable,
        })
    }
}

/**
 * Represents a character from the Hangul Jamo unicode block.
 *
 * Specifically, it only includes the modern characters, and
 * ignores the archaic ones:
 *
 * https://en.wikipedia.org/wiki/Hangul_Jamo_(Unicode_block)
 */
#[derive(Copy, Clone)]
pub enum ModernJamo {
    InitialConsonant(char),
    Vowel(char),
    FinalConsonant(char),
}

impl ModernJamo {
    pub fn try_from_char(char: char) -> Option<Self> {
        match char {
            'ᄀ'..='ᄒ' => Some(ModernJamo::InitialConsonant(char)),
            'ᅡ'..='ᅵ' => Some(ModernJamo::Vowel(char)),
            'ᆨ'..='ᇂ' => Some(ModernJamo::FinalConsonant(char)),
            _ => None,
        }
    }

    pub fn is_initial_consonant(char: char) -> bool {
        match Self::try_from_char(char) {
            Some(ModernJamo::InitialConsonant(_)) => true,
            _ => false,
        }
    }
}

impl Into<char> for ModernJamo {
    fn into(self) -> char {
        match self {
            ModernJamo::InitialConsonant(ch) => ch,
            ModernJamo::Vowel(ch) => ch,
            ModernJamo::FinalConsonant(ch) => ch,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::jamo_stream::{JamoInStream, JamoStream};

    #[test]
    fn test_it_works() {
        let mut stream = JamoStream::from_hangul_syllables("밥이");

        assert_eq!(
            stream.next().unwrap(),
            JamoInStream {
                prev: None,
                curr: 'ᄇ',
                next: Some('ᅡ'),
                next_syllable: Some('이')
            }
        );

        assert_eq!(
            stream.next().unwrap(),
            JamoInStream {
                prev: Some('ᄇ'),
                curr: 'ᅡ',
                next: Some('ᆸ'),
                next_syllable: Some('이')
            }
        );

        assert_eq!(
            stream.next().unwrap(),
            JamoInStream {
                prev: Some('ᅡ'),
                curr: 'ᆸ',
                next: Some('ᄋ'),
                next_syllable: Some('이')
            }
        );

        assert_eq!(
            stream.next().unwrap(),
            JamoInStream {
                prev: Some('ᆸ'),
                curr: 'ᄋ',
                next: Some('ᅵ'),
                next_syllable: None
            }
        );

        assert_eq!(
            stream.next().unwrap(),
            JamoInStream {
                prev: Some('ᄋ'),
                curr: 'ᅵ',
                next: None,
                next_syllable: None
            }
        );

        assert_eq!(stream.next(), None);
        assert_eq!(stream.next(), None);
        assert_eq!(stream.next(), None);
    }
}
