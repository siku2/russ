use russ::css::{types::*, CSSWriter, WriteValue};

fn render(value: impl WriteValue) -> String {
    let mut v = Vec::new();
    value
        .write_value(&mut CSSWriter::new(&mut v))
        .expect("failed to write value");
    String::from_utf8(v).expect("invalid utf8 returned")
}

#[test]
fn integer() {
    assert_eq!(render(Integer::from(5)), "5");
}

#[test]
fn number() {
    assert_eq!(render(Number::from(5)), "5");
    assert_eq!(render(Number::from(5.5)), "5.5");
}

#[test]
fn string() {
    assert_eq!(render(CSSString::from("hello world")), "\"hello world\"");
    assert_eq!(render(CSSString::from(r#" "' "#)), r#" \"' "#);
}

#[test]
fn length() {
    assert_eq!(render(Length::Px(10.into())), "10px");
    assert_eq!(render(Length::Zero), "0");
}
