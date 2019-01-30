use string_stupidify::Stupid;

#[test]
fn alternate_case() {
    let alternating = String::from("abcde").alternate_case().unwrap();
    assert_eq!("AbCdE", alternating.as_str());
}

#[test]
fn alternate_case_with_special_chars() {
    let alternating = String::from("abc.de f34ghßi").alternate_case().unwrap();
    assert_eq!("AbC.dE f34GhSSi", alternating.as_str());
}