#[derive(PartialEq, Debug)]
pub struct JamoInStream {
    pub curr: char,
    pub prev: Option<char>,
    pub next: Option<char>,
}

impl JamoInStream {
    pub fn is_final_consonant_followed_by_vowel(&self) -> bool {
        // This assumes our stream is a well-formed sequence of Jamos.
        self.next == Some('ᄋ')
    }
}

pub struct JamoStream {
    jamos: Vec<char>,
    index: usize,
}

impl JamoStream {
    pub fn from_hangul_syllables<T: AsRef<str>>(value: T) -> Self {
        use crate::hangul::decompose_all_hangul_syllables;
        Self::from_jamos(decompose_all_hangul_syllables(value))
    }

    pub fn from_jamos<T: AsRef<str>>(value: T) -> Self {
        let jamos = value.as_ref().chars().collect();
        Self { jamos, index: 0 }
    }

    pub fn seek(&mut self, index: usize) {
        self.index = index;
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
        self.index += 1;
        Some(JamoInStream { curr, prev, next })
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
            }
        );

        assert_eq!(
            stream.next().unwrap(),
            JamoInStream {
                prev: Some('ᄇ'),
                curr: 'ᅡ',
                next: Some('ᆸ'),
            }
        );

        assert_eq!(
            stream.next().unwrap(),
            JamoInStream {
                prev: Some('ᅡ'),
                curr: 'ᆸ',
                next: None,
            }
        );

        assert_eq!(stream.next(), None);
        assert_eq!(stream.next(), None);
        assert_eq!(stream.next(), None);
    }
}
