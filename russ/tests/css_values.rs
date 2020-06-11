use russ::css::{values::*, CSSWriter, WriteValue};

fn render(value: impl WriteValue) -> String {
    let mut v = Vec::new();
    value
        .write_value(&mut CSSWriter::new(&mut v))
        .expect("failed to write value");
    String::from_utf8(v).expect("invalid utf8 returned")
}

#[test]
fn calc() {
    assert_eq!(
        render(Calc::bin_sub(Percentage::from(100), Length::px(80))),
        "calc(100% - 80px)"
    );
    assert_eq!(
        render(Calc::bin_div(Calc::bin_div(Length::px(100), 2), 2)),
        "calc((100px / 2) / 2)"
    );
    assert_eq!(
        render(Color::rgba(Calc::bin_sub(255, 5), 0, 153, 1)),
        "rgb(calc(255 - 5),0,153,1)"
    );
}

#[test]
fn color() {
    assert_eq!(render(Color::hex(0xff0099)), "#FF0099");

    assert_eq!(render(Color::rgb(255, 0, 153)), "rgb(255,0,153)");
    assert_eq!(
        render(Color::rgb(
            Percentage::from(100),
            Percentage::from(0),
            Percentage::from(60)
        )),
        "rgb(100%,0%,60%)"
    );
    assert_eq!(render(Color::rgba(255, 0, 153, 1)), "rgb(255,0,153,1)");

    assert_eq!(
        render(Color::hsl(
            Angle::turn(0.75),
            Percentage::from(60),
            Percentage::from(70),
        )),
        "hsl(0.75turn,60%,70%)"
    );
    assert_eq!(
        render(Color::hsla(
            Angle::deg(270),
            Percentage::from(60),
            Percentage::from(50),
            Percentage::from(15),
        )),
        "hsl(270deg,60%,50%,15%)"
    );
}

#[test]
fn gradient() {
    // linear

    assert_eq!(
        render(Gradient::linear(
            Some(Angle::deg(45)),
            vec![(
                (Color::hex(0xff0000), Length::Zero, Percentage::from(50)),
                None
            )],
            (
                Color::hex(0x0000ff),
                Percentage::from(50),
                Percentage::from(100)
            )
        )),
        "linear-gradient(45deg,#FF0000 0 50%,#0000FF 50% 100%)"
    );
    assert_eq!(
        render(Gradient::linear(
            Some(Angle::turn(0.25)),
            vec![(Color::hex(0xff0000), Percentage::from(10))],
            Color::hex(0x0000ff),
        )),
        "linear-gradient(0.25turn,#FF0000,10%,#0000FF)"
    );
    assert_eq!(
        render(Gradient::linear(
            None,
            vec![
                ((Color::hex(0xFF0000), Percentage::from(0)), None),
                ((Color::hex(0xFFA500), Percentage::from(10)), None),
                ((Color::hex(0xFFA500), Percentage::from(30)), None),
                ((Color::hex(0xFFFF00), Percentage::from(50)), None),
                ((Color::hex(0xFFFF00), Percentage::from(70)), None),
                ((Color::hex(0x00FF00), Percentage::from(90)), None),
            ],
            (Color::hex(0x00FF00), Percentage::from(100)),
        )),
        "linear-gradient(#FF0000 0%,#FFA500 10%,#FFA500 30%,#FFFF00 50%,#FFFF00 70%,#00FF00 90%,#00FF00 100%)"
    );

    // radial

    assert_eq!(
        render(Gradient::radial_at(
            None,
            vec![(Color::hex(0xE66465), None)],
            Color::hex(0x9198E5),
        )),
        "radial-gradient(#E66465,#9198E5)"
    );

    assert_eq!(
        render(Gradient::radial_size(
            GradientShapeSize::ClosestSide,
            vec![(Color::hex(0x3F87A6), None), (Color::hex(0xEBF8E1), None)],
            Color::hex(0xD69D3C)
        )),
        "radial-gradient(closest-side,#3F87A6,#EBF8E1,#D69D3C)"
    );
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
            (PositionHorizontalAnchor::Right, Length::px(-6)),
            (PositionVerticalAnchor::Bottom, Length::v_min(12)),
        )),
        "right -6px bottom 12vmin"
    );

    assert_eq!(
        render(Position::xy(Percentage::from(10), Percentage::from(20))),
        "10% 20%"
    );
    assert_eq!(
        render(Position::xy(Length::rem(8), Length::px(14),)),
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
