use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen::JsCast;
use web_sys::{window, Response};
use ezrustdom as erd;

#[wasm_bindgen(start)]
pub fn on_start() {
    spawn_local(async {
        let window = window().unwrap();

        let response = wasm_bindgen_futures::JsFuture::from(
            window.fetch_with_str("/api/v1/health")
        )
        .await
        .unwrap();

        let response: Response = response.dyn_into().unwrap();

        let json = wasm_bindgen_futures::JsFuture::from(
            response.json().unwrap()
        )
        .await
        .unwrap();

        let status = js_sys::Reflect::get(
            &json,
            &JsValue::from_str("status"),
        )
        .unwrap()
        .as_string()
        .unwrap_or_else(|| "unknown".to_string());

        let document = window.document().unwrap();
        let element = document
            .get_element_by_id("health")
            .unwrap();

        element.set_text_content(Some(&format!(
            "Status: {}",
            status.to_uppercase()
        )));
    });
}