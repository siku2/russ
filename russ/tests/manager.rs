use russ::{
    bindings,
    css::{props::*, values::*},
    RuleSet, StyleManager, Styles,
};
use std::rc::Rc;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn tracking() {
    let mut manager = StyleManager::default();
    let styles = Styles::build(vec![RuleSet::build(vec![Background(
        vec![],
        BackgroundLayerFinal {
            color: BackgroundColor(Color::hex(0xffffff)),
            layer: Default::default(),
        },
    )])]);
    let sheet_ref = manager.track_styles(&styles);
    // make sure we get the same one again
    assert!(Rc::ptr_eq(&sheet_ref, &manager.track_styles(&styles)));

    let id = styles.generate_key().unique_id();
    assert!(bindings::has_style_sheet(&id));
    drop(sheet_ref);
    assert!(!bindings::has_style_sheet(&id));

    // make sure we can add it again
    let sheet_ref = manager.track_styles(&styles);
    assert!(bindings::has_style_sheet(&id));
    drop(sheet_ref);
}
