use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use ezrustdom as erd;
use serde;

#[derive(serde::Serialize, serde::Deserialize)]
struct Stop {
    id: i32,
    name: String,
    image_url: Option<String>,
    wheelchair_accessible: bool,
    has_shelter: bool,
    has_ticket_machine: bool,
}
#[wasm_bindgen(start)]
pub fn on_start() {
    spawn_local(async {
        let document = erd::get_document().await;
        let stops = erd::fetch_json::<Vec<Stop>>("/api/v1/stops").await;
        let stop = erd::fetch_json::<Stop>("/api/v1/stops/2").await;
    });
}
