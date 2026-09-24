use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use ezrustdom as erd;

#[wasm_bindgen(start)]
pub fn on_start() {
    spawn_local(async {
        let document = erd::get_document().await;
        let teams = erd::fetch_json::<Vec<erd::Team>>("/api/teamdb").await;
        erd::print("im in");
        if let Some(output) = document.get_element_by_id("db_output") {
            output.set_text_content(Some(format!("The team of {}, Members: {}", teams[0].name, teams[0].contestants).as_str()));
        }
    });
}
