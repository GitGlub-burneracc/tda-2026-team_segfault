use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use ezrustdom as erd;

#[wasm_bindgen(start)]
pub fn on_start() {
    spawn_local(async {
        let document = erd::get_document().await;
        let teams = match erd::fetch_json::<Vec<erd::Team>>("/api/teamdb").await {
            Ok(teams) => teams,
            Err(error) => {
                web_sys::console::error_1(&error);
                if let Some(output) = document.get_element_by_id("db_output") {
                    output.set_text_content(Some("Failed to load teams. See the console for details."));
                }
                return;
            }
        };
        erd::print("im in");
        if let Some(output) = document.get_element_by_id("db_output") {
            output.set_text_content(Some(format!("{teams:?}").as_str()));
        }
    });
}
