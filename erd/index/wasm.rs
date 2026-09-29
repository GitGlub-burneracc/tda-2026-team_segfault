use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use ezrustdom as erd;
use serde;
use serde_json;

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
    });
}
