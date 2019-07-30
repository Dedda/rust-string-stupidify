use crate::{StringDecorator, DecorationError, Stupid};

pub struct Alternate;

impl StringDecorator for Alternate {
    fn decorate(&self, text: &String) -> Result<String, DecorationError> {
        match text.alternate_case() {
            Some(s) => Ok(s),
            None => Err(DecorationError::Unknown(None)),
        }
    }
}

pub struct InvertCase;

impl StringDecorator for InvertCase {
    fn decorate(&self, text: &String) -> Result<String, DecorationError> {
        match text.invert_case() {
            Some(s) => Ok(s),
            None => Err(DecorationError::Unknown(None)),
        }
    }
}

pub struct VaporWave;

impl StringDecorator for VaporWave {
    fn decorate(&self, text: &String) -> Result<String, DecorationError> {
        match text.vapor_wave() {
            Some(s) => Ok(s),
            None => Err(DecorationError::Unknown(None)),
        }
    }
}

pub struct Shuffle;

impl StringDecorator for Shuffle {
    fn decorate(&self, text: &String) -> Result<String, DecorationError> {
        match text.shuffle() {
            Some(s) => Ok(s),
            None => Err(DecorationError::Unknown(None)),
        }
    }
}

pub struct AlphaSort;

impl StringDecorator for AlphaSort {
    fn decorate(&self, text: &String) -> Result<String, DecorationError> {
        match text.alphabetical() {
            Some(s) => Ok(s),
            None => Err(DecorationError::Unknown(None)),
        }
    }
}

#[cfg(test)]
mod decorators_tests {
    use crate::{Stupid, StringDecorator};
    use crate::decorators::Alternate;

    #[test]
    fn test_alternate() {
        let text = "abcde fghi".to_string();
        assert_eq!(text.alternate_case().unwrap(), Alternate.decorate(&text).unwrap());
    }
}