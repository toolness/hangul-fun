/// This module encapsulates the conversation from
/// Unit 2, "Greetings & Introductions", of
/// Active Korean 1 by the Language Education Institute
/// of Seoul National University, pg. 42.
use anyhow::{Result, anyhow};
use rand::seq::SliceRandom;
use rand::{Rng, thread_rng};

use crate::hangul::decompose_hangul_syllable_to_jamos;

const NAMES: [&str; 4] = ["양양", "키샨", "마이클", "크리스"];

const COUNTRIES: [&str; 4] = ["미국", "중국", "일분", "인도"];

const OCCUPATIONS: [&str; 4] = ["선생님", "학생", "의사", "요리사"];

pub fn run_introductions() -> Result<()> {
    let mut rng = thread_rng();
    let name = *NAMES.choose(&mut rng).unwrap();
    let country = *COUNTRIES.choose(&mut rng).unwrap();
    let occupation = *OCCUPATIONS.choose(&mut rng).unwrap();

    println!("안녕하세요?");
    println!("안녕하세요? 저는 {name}{}.", get_copula(name)?);

    let guessed_country = *guess(&COUNTRIES, &country)?;
    println!("{name} 씨는 {guessed_country} 사람이에요?");
    if guessed_country == country {
        println!("네, 저는 {country} 사람이에요.");
    } else {
        println!("아니요, 저는 {country} 사람이에요.");
    }

    let guessed_occupation = *guess(&OCCUPATIONS, &occupation)?;
    println!(
        "{name} 씨는 {guessed_occupation}{}?",
        get_copula(guessed_occupation)?
    );
    if guessed_occupation == occupation {
        println!("네, 저는 {occupation}{}.", get_copula(occupation)?);
    } else {
        println!("아니요, 저는 {occupation}{}.", get_copula(occupation)?);
    }

    Ok(())
}

fn guess<'a, T: AsRef<str> + PartialEq>(items: &'a [T], correct: &'a T) -> Result<&'a T> {
    let mut rng = thread_rng();
    let guess_correctly = rng.gen_bool(0.5);
    if guess_correctly {
        Ok(correct)
    } else {
        guess_other(items, correct)
    }
}

fn guess_other<'a, T: AsRef<str> + PartialEq>(items: &'a [T], except: &T) -> Result<&'a T> {
    let mut rng = thread_rng();
    let mut i = 0;
    loop {
        let Some(choice) = items.choose(&mut rng) else {
            return Err(anyhow!("items is empty"));
        };
        if choice != except {
            return Ok(choice);
        }
        i += 1;
        if i > 5000 {
            return Err(anyhow!("exceeded maximum attempts"));
        }
    }
}

fn ends_in_vowel<T: AsRef<str>>(value: T) -> Result<bool> {
    let Some(last_char) = value.as_ref().chars().last() else {
        return Err(anyhow!("string is empty"));
    };
    let Some((_initial, _vowel, final_consonant)) = decompose_hangul_syllable_to_jamos(last_char)
    else {
        return Err(anyhow!("final character is not a hangul syllable"));
    };
    Ok(final_consonant.is_none())
}

fn get_copula<T: AsRef<str>>(value: T) -> Result<&'static str> {
    if ends_in_vowel(value)? {
        Ok("예요")
    } else {
        Ok("이에요")
    }
}

#[cfg(test)]
mod tests {
    use crate::introductions::ends_in_vowel;

    #[test]
    fn test_ends_in_vowel() {
        assert_eq!(ends_in_vowel("한").unwrap(), false);
        assert_eq!(ends_in_vowel("네").unwrap(), true);
    }
}
