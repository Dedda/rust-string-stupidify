extern crate rand;

use rand::thread_rng;
use rand::seq::SliceRandom;

mod decorator;
pub mod decorators;

pub use decorator::{Decoratable, DecorationError, StringDecorator, ForcedStringDecorator};

pub trait Stupid<T: ToString> {

    fn alternate_case(&self) -> Option<T>;
    fn invert_case(&self) -> Option<T>;
    fn vapor_wave(&self) -> Option<T>;
    fn shuffle(&self) -> Option<T>;
    fn alphabetical(&self) -> Option<T>;

}

#[derive(Debug, PartialEq)]
enum Case {
    Upper,
    Lower,
}

impl Case {

    fn case_of(c: char) -> Option<Case> {
        if c.is_alphabetic() {
            Some( if c.is_lowercase() { Case::Lower } else { Case::Upper } )
        } else {
            None
        }
    }

    fn opposite(&self) -> Case {
        match self {
            Case::Upper => Case::Lower,
            Case::Lower => Case::Upper,
        }
    }

    fn apply(&self, c: char) -> String {
        if c.is_alphabetic() {
            match self {
                Case::Upper => c.to_uppercase().to_string(),
                Case::Lower => c.to_lowercase().to_string(),
            }
        } else {
            c.to_string()
        }
    }
}

#[inline]
fn alternate_str(data: &str) -> Option<String> {
    let mut buffer = String::with_capacity(data.len());
    let chars = data.chars();
    let mut case = Case::Upper;
    for c in chars {
        if c.is_alphabetic() {
            buffer.push_str(case.apply(c).as_str());
            case = case.opposite()
        } else {
            buffer.push(c)
        }
    }
    Some(buffer)
}

#[inline]
fn invert_case_on_str(data: &str) -> Option<String> {
    let mut buffer = String::with_capacity(data.len());
    let chars = data.chars();
    for c in chars {
        match Case::case_of(c) {
            None => buffer.push(c),
            Some(case) => buffer.push_str(case.opposite().apply(c).as_str()),
        }
    }
    Some(buffer)
}

#[inline]
fn vapor_wave_str(data: &str) -> Option<String> {
    if data.is_empty() {
        return Some(String::new());
    }
    let mut buffer = String::with_capacity(data.len() * 2 - 1);
    let chars = data.chars();
    for c in chars {
        buffer.push(c);
        buffer.push(' ');
    }
    buffer.pop();
    Some(buffer.to_uppercase())
}

#[inline]
fn shuffle_str(data: &str) -> Option<String> {
    let mut vec: Vec<char> = data.chars().collect();
    let slice: &mut [char] = &mut vec;

    let mut rng = thread_rng();
    slice.shuffle(&mut rng);
    Some(slice.iter().cloned().collect::<String>())
}

#[inline]
fn is_str_made_of_identical_chars(data: &str) -> bool {
    if data.len() < 2 {
        return true;
    }
    let c = data.chars().next().unwrap();
    return data.replace(c, "").is_empty()
}

#[inline]
fn sort_str_alphabetical(data: &str) -> String {
    let mut vec: Vec<char> = data.chars().collect();
    vec.sort();
    vec.iter().cloned().collect::<String>()
}

impl Stupid<String> for String {

    /// # Examples:
    ///
    /// ```
    /// use string_stupidify::Stupid;
    ///
    /// let alternating = String::from("abcde").alternate_case().unwrap();
    /// assert_eq!("AbCdE", alternating.as_str());
    /// ```
    ///
    /// This function will ignore non-alphabetic characters:
    ///
    /// ```
    /// use string_stupidify::Stupid;
    ///
    /// let alternating = String::from("abc.de f34ghßi").alternate_case().unwrap();
    /// assert_eq!("AbC.dE f34GhSSi", alternating.as_str());
    /// ```
    fn alternate_case(&self) -> Option<String> {
        let chars = self.as_str();
        alternate_str(chars)
    }

    /// # Example:
    ///
    /// ```
    /// use string_stupidify::Stupid;
    ///
    /// let inverted = String::from("aBc.de F").invert_case().unwrap();
    /// assert_eq!("AbC.DE f", inverted);
    /// ```
    fn invert_case(&self) -> Option<String> {
        let chars = self.as_str();
        invert_case_on_str(chars)
    }

    /// # Example:
    ///
    /// ```
    /// use string_stupidify::Stupid;
    ///
    /// let vaporized = String::from("abCD eF").vapor_wave().unwrap();
    /// assert_eq!("A B C D   E F", vaporized.as_str());
    /// ```
    fn vapor_wave(&self) -> Option<String> {
        let chars = self.as_str();
        vapor_wave_str(chars)
    }

    /// # Example:
    ///
    /// ```
    /// use string_stupidify::Stupid;
    ///
    /// let shuffled = String::from("abcdeba").shuffle().unwrap();
    /// assert_ne!("abcdeba", shuffled);
    /// assert_eq!(7, shuffled.len());
    /// assert_eq!(2, shuffled.matches("a").count());
    /// assert_eq!(2, shuffled.matches("b").count());
    /// assert_eq!(1, shuffled.matches("c").count());
    /// assert_eq!(1, shuffled.matches("d").count());
    /// assert_eq!(1, shuffled.matches("e").count());
    /// ```
    ///
    /// Single char or empty Strings return a copy of the String:
    ///
    /// ```
    /// use string_stupidify::Stupid;
    ///
    /// assert_eq!("", "".to_string().shuffle().unwrap());
    /// assert_eq!("a", "a".to_string().shuffle().unwrap());
    /// ```
    ///
    /// Strings with only one repeated char are also returning a copy:
    ///
    /// ```
    /// use string_stupidify::Stupid;
    ///
    /// assert_eq!("aaaaa", "aaaaa".to_string().shuffle().unwrap());
    /// ```
    fn shuffle(&self) -> Option<String> {
        if is_str_made_of_identical_chars(self.as_str()) {
           return Some(self.clone())
        }
        let chars = self.as_str();
        if let Some(mut shuffled) = shuffle_str(chars) {
            while &shuffled == self {
                if let Some(s) = shuffle_str(chars) {
                    shuffled =s;
                }
            }
            Some(shuffled)
        } else {
            None
        }
    }

    /// # Example:
    ///
    /// ```
    /// use string_stupidify::Stupid;
    ///
    /// let sorted = String::from("basbc").alphabetical().unwrap();
    /// assert_eq!("abbcs", sorted);
    /// ```
    fn alphabetical(&self) -> Option<String> {
        let chars = self.as_str();
        Some(sort_str_alphabetical(chars))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_of_char() {
        assert_eq!(Some(Case::Lower), Case::case_of('a'));
        assert_eq!(Some(Case::Upper), Case::case_of('A'));
        assert_eq!(None, Case::case_of('.'));
    }

    #[test]
    fn opposite_case() {
        assert_eq!(Case::Lower, Case::Upper.opposite());
        assert_eq!(Case::Upper, Case::Lower.opposite());
    }

    #[test]
    fn apply_case() {
        assert_eq!("A", Case::Upper.apply('a'));
        assert_eq!("a", Case::Lower.apply('A'));
    }

    #[test]
    fn str_made_of_identical_chars() {
        assert!(is_str_made_of_identical_chars("aaaaa"));
        assert!(!is_str_made_of_identical_chars("aaada"));
    }
}
