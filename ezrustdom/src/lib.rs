
use js_sys::Promise;
use wasm_bindgen_futures::JsFuture;
use web_sys::window;
use web_sys;
use web_sys::{Request, RequestInit, Response};
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use serde::de::DeserializeOwned;

pub fn print(str: &str) {
    web_sys::console::log_1(
            &str.into()
    );
}

pub async fn sleep(ms: u32) {
    let promise = Promise::new(&mut |resolve, _reject| {
        window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                &resolve,
                ms as i32,
            )
            .unwrap();
    });

    JsFuture::from(promise).await.unwrap();
}

pub async fn fetch(request: web_sys::Request) -> web_sys::Response {
    let window = web_sys::window().unwrap();

    wasm_bindgen_futures::JsFuture::from(
        window.fetch_with_request(&request)
    )
    .await
    .unwrap()
    .dyn_into::<web_sys::Response>()
    .unwrap()
}

pub async fn fetch_json<T>(url: &str) -> T
where
    T: DeserializeOwned,
{
    let response = fetch(web_sys::Request::new_with_str(url).unwrap()).await;

    let json = JsFuture::from(
        response.json().unwrap()
    )
    .await.unwrap();

    serde_wasm_bindgen::from_value(json).unwrap()
}

pub async fn post(url: &str, body: String, req_type: &str) -> web_sys::Response {
    let options = web_sys::RequestInit::new();
    options.set_method(req_type);
    options.set_body(&body.into());

    let request =
        web_sys::Request::new_with_str_and_init(url, &options).unwrap();

    request
        .headers()
        .set("Content-Type", "application/json")
        .unwrap();

    fetch(request).await
}

pub async fn get_document()  -> web_sys::Document {
    web_sys::window()
    .unwrap()
    .document()
    .unwrap()
}
