#[derive(PartialEq, Debug)]
pub struct JamoInStream {
    pub curr: char,
    pub prev: Option<char>,
    pub next: Option<char>,
    pub after_next: Option<char>,
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
        }
    }

    pub fn seek_to_syllable(&mut self, index: usize) {
        if let Some(&jamo_index) = self.syllable_indices.get(index) {
            self.index = jamo_index;
        }
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
        let next = self.jamos.get(self.index + 1).cloned();
        let after_next = self.jamos.get(self.index + 2).cloned();
        self.index += 1;
        Some(JamoInStream {
            curr,
            prev,
            next,
            after_next,
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
        let mut stream = JamoStream::from_hangul_syllables("밥");

        assert_eq!(
            stream.next().unwrap(),
            JamoInStream {
                prev: None,
                curr: 'ᄇ',
                next: Some('ᅡ'),
                after_next: Some('ᆸ')
            }
        );

        assert_eq!(
            stream.next().unwrap(),
            JamoInStream {
                prev: Some('ᄇ'),
                curr: 'ᅡ',
                next: Some('ᆸ'),
                after_next: None
            }
        );

        assert_eq!(
            stream.next().unwrap(),
            JamoInStream {
                prev: Some('ᅡ'),
                curr: 'ᆸ',
                next: None,
                after_next: None
            }
        );

        assert_eq!(stream.next(), None);
        assert_eq!(stream.next(), None);
        assert_eq!(stream.next(), None);
    }
}
