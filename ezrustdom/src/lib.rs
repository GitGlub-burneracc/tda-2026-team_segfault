
use js_sys::Promise;
use wasm_bindgen_futures::JsFuture;
use web_sys::window;
use web_sys;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use serde::de::DeserializeOwned;

#[derive(serde::Deserialize)]
#[derive(Debug)]
pub struct Team {
    pub name: String,
    pub contestants: String,
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

pub async fn fetch(url: &str) -> Result<web_sys::Response, JsValue> {
    let window = web_sys::window()
        .ok_or_else(|| JsValue::from_str("Browser window is unavailable"))?;

    let response = wasm_bindgen_futures::JsFuture::from(
        window.fetch_with_str(url)
    )
    .await?
    .dyn_into::<web_sys::Response>()?;

    if !response.ok() {
        return Err(JsValue::from_str(&format!(
            "GET {url} failed: HTTP {} {}",
            response.status(), response.status_text()
        )));
    }

    Ok(response)
}

pub async fn fetch_json<T>(url: &str) -> T
where
    T: DeserializeOwned,
{
    let response = fetch(url).await.unwrap();

    let json = JsFuture::from(
        response.json().unwrap()
    )
    .await.unwrap();

    serde_wasm_bindgen::from_value(json).unwrap()
}

pub async fn get_document()  -> web_sys::Document {
    web_sys::window()
    .unwrap()
    .document()
    .unwrap()
}
