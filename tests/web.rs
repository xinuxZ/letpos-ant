use ant_leptos::*;
use leptos::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn version_component_renders() {
    let runtime = create_runtime();
    let root = document().create_element("div").unwrap();
    document().body().unwrap().append_child(&root).unwrap();

    leptos::mount_to_body(|| view! { <Version/> });

    let version_element = document().query_selector(".ant-version").unwrap().unwrap();
    assert!(version_element
        .text_content()
        .unwrap()
        .contains("ant-leptos"));

    runtime.dispose();
}

fn document() -> web_sys::Document {
    web_sys::window()
        .expect("should have a window")
        .document()
        .expect("should have a document")
}
