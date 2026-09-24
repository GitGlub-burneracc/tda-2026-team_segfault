use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use ezrustdom as erd;

#[wasm_bindgen(start)]
pub fn on_start() {
    spawn_local(async {
        let document = erd::get_document().await;
        let json = erd::fetch_json("/api/teamdb").await;
        if let Some(output) = document.get_element_by_id("db_output") {
            output.set_text_content(format!());
        }
    });
}
