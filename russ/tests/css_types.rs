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
fn position() {
    assert_eq!(render(Position::center()), "center");
    assert_eq!(render(Position::x(PositionHorizontalAnchor::Left)), "left");
    assert_eq!(
        render(Position::xy(
            PositionHorizontal::Center,
            PositionVerticalAnchor::Top
        )),
        "center top"
    );

    assert_eq!(
        render(Position::xy(
            PositionHorizontalAnchor::Right,
            Percentage(8.5.into()),
        )),
        "right 8.5%"
    );
    assert_eq!(
        render(Position::xy(
            (PositionHorizontalAnchor::Right, Length::Px((-6).into())),
            (PositionVerticalAnchor::Bottom, Length::VMin(12.into())),
        )),
        "right -6px bottom 12vmin"
    );

    assert_eq!(
        render(Position::xy(Percentage(10.into()), Percentage(20.into()),)),
        "10% 20%"
    );
    assert_eq!(
        render(Position::xy(Length::Rem(8.into()), Length::Px(14.into()),)),
        "8rem 14px"
    );
}

#[test]
fn ratio() {
    assert_eq!(render(Ratio(16.into(), 9.into())), "16/9");
}

#[test]
fn string() {
    assert_eq!(render(CSSString::from("hello world")), "\"hello world\"");
    assert_eq!(render(CSSString::from(r#" "'" "#)), r#"" \"'\" ""#);
}

#[test]
fn length() {
    assert_eq!(render(Length::Px(10.into())), "10px");
    assert_eq!(render(Length::Zero), "0");
}
