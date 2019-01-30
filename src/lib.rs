pub trait Stupid<T: ToString> {

    fn alternate_case(&self) -> Option<T>;

    fn vapor_wave(&self) -> Option<T>;

}

enum Case {
    Upper,
    Lower,
}

impl Case {
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

fn vapor_wave_str(data: &str) -> Option<String> {
    let mut buffer = String::with_capacity(data.len() * 2 - 1);
    let chars = data.chars();
    for c in chars {
        buffer.push(c);
        buffer.push(' ');
    }
    if buffer.len() > 0 {
        buffer.pop();
    }
    Some(buffer.to_uppercase())
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
    /// let vaporized = String::from("abCD eF").vapor_wave().unwrap();
    /// assert_eq!("A B C D   E F", vaporized.as_str());
    /// ```
    fn vapor_wave(&self) -> Option<String> {
        let chars = self.as_str();
        vapor_wave_str(chars)
    }

}
