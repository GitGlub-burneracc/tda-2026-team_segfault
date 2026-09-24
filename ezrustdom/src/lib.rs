
use js_sys::Promise;
use wasm_bindgen_futures::JsFuture;
use web_sys::window;
use wasm_bindgen::JsCast;

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