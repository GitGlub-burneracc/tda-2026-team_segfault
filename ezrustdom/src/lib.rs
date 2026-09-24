
use js_sys::Promise;
use wasm_bindgen_futures::JsFuture;
use web_sys::window;
use web_sys;
use wasm_bindgen::JsCast;
use serde::de::DeserializeOwned;

#[derive(serde::Deserialize)]
#[derive(Debug)]
pub struct Team {
    name: String,
    contestants: i32,
}

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

pub async fn fetch(url: &str) -> web_sys::Response {
    let window = web_sys::window().unwrap();

    wasm_bindgen_futures::JsFuture::from(
        window.fetch_with_str(url)
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
    let response = fetch(url).await;
    print("got res");

    let json = JsFuture::from(
        response.json().unwrap()
    )
    .await
    .unwrap();
    print("got json");

    serde_wasm_bindgen::from_value(json).unwrap()
}

pub async fn get_document()  -> web_sys::Document {
    web_sys::window()
    .unwrap()
    .document()
    .unwrap()
}