use trybuild::TestCases;

#[test]
fn css_value() {
    let t = TestCases::new();
    t.pass("css_value/color-pass.rs");
}