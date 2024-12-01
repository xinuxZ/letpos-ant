use wasm_bindgen_test::*;
use ant_leptos::*;
use leptos::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn version_component_renders() {
    let version = Version();
    assert!(version.into_view().to_string().contains("ant-leptos"));
}
