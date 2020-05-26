use web_sys::{Document, Element};

fn get_document() -> Document {
    web_sys::window()
        .expect("no window found")
        .document()
        .expect("no document found")
}

fn get_style_el(doc: &Document, id: &str) -> Option<Element> {
    doc.get_element_by_id(id)
}

fn build_style_element(doc: &Document, id: &str, body: &str) -> Element {
    let el = doc
        .create_element("style")
        .expect("failed to create style element");
    el.set_id(id);
    el.set_inner_html(body);
    el
}

/// Add a style sheet to the head element.
pub fn add_style_sheet(id: &str, body: &str) -> bool {
    let doc = get_document();
    if get_style_el(&doc, id).is_some() {
        // already exists
        return false;
    }

    let el = build_style_element(&doc, id, body);
    let head = doc.head().expect("document has no head");
    head.append_child(&el)
        .expect("failed to add style to head element");
    true
}

/// Check if there is a style sheet with the given id.
pub fn has_style_sheet(id: &str) -> bool {
    get_style_el(&get_document(), id).is_some()
}

/// Remove a style sheet from the head element.
pub fn remove_style_sheet(id: &str) -> bool {
    let doc = get_document();
    if let Some(el) = get_style_el(&doc, id) {
        el.remove();
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
}
