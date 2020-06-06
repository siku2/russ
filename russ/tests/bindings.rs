use russ::bindings::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_add() {
    assert!(add_style_sheet("add.1", "div{width: 100%;}"));
    assert!(!add_style_sheet("add.1", ""));

    assert!(add_style_sheet("add.2", "a{width: 1px;}"));
}

#[wasm_bindgen_test]
fn test_has() {
    assert!(add_style_sheet("has.1", ""));
    assert!(has_style_sheet("has.1"));

    assert!(!has_style_sheet("has.2"));
    assert!(add_style_sheet("has.2", ""));
    assert!(has_style_sheet("has.2"));
}

#[wasm_bindgen_test]
fn test_remove() {
    assert!(add_style_sheet("remove.1", ""));
    assert!(add_style_sheet("remove.2", ""));

    assert!(remove_style_sheet("remove.1"));
    assert!(remove_style_sheet("remove.2"));
    assert!(!remove_style_sheet("remove.1"));
}
