#[derive(Debug)]
pub enum DecorationError {
    Unknown(Option<String>),
    StringEmpty,
}

pub trait StringDecorator {
    fn decorate(&self, text: &String) -> Result<String, DecorationError>;
}

pub trait ForcedStringDecorator {
    fn force_decorate(&self, text: &String) -> String;
}

impl<T> ForcedStringDecorator for T where T: StringDecorator {
    fn force_decorate(&self, text: &String) -> String {
        match self.decorate(text) {
            Ok(s) => s,
            Err(e) => panic!("Error decorating text: {:?}", e),
        }
    }
}

pub trait Decoratable {
    fn decorate(&self, decorators: &Vec<Box<dyn StringDecorator>>) -> Result<String, DecorationError>;
}

impl<T> Decoratable for T where T: ToString {
    fn decorate(&self, decorators: &Vec<Box<dyn StringDecorator>>) -> Result<String, DecorationError> {
        let mut s = self.to_string();
        for decorator in decorators {
            match decorator.decorate(&s) {
                Ok(decorated) => s = decorated,
                error => return error,
            }
        }
        Ok(s)
    }
}

#[cfg(test)]
mod decoratable_tests {
    use crate::{Decoratable, StringDecorator};
    use crate::decorators::{Alternate, VaporWave};

    #[test]
    fn test_decorate() {
        let text = "Hello, World";
        let decorators: Vec<Box<dyn StringDecorator>> = vec![
            Box::new(VaporWave),
            Box::new(Alternate),
        ];
        assert_eq!("H e L l O ,   w O r L d".to_string(), text.decorate(&decorators).unwrap());
    }
}