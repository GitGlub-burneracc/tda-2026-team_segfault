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
        let document = erd::get_document().await;
        let newly_built_stop = Stop{id:0, name:String::from("GGGGGG"), image_url: None, wheelchair_accessible: true, has_shelter:false, has_ticket_machine:false};
        let rebuilt_stop = Stop{id:0, name:String::from("New hopper hall"), image_url: None, wheelchair_accessible: true, has_shelter:true, has_ticket_machine:true};
        erd::fetch_json::<Vec<Stop>>("/api/v1/stops").await;
        erd::post("/api/v1/stops", serde_json::to_string(&newly_built_stop).unwrap(), "POST", Some(&[("Authorization", "Bearer Kyqc49jIM+5+D0Sed8ZQ671gxkd7W/bBTWjDtZ0Zrgk="),])).await;
        erd::post("/api/v1/stops/6", serde_json::to_string(&rebuilt_stop).unwrap(), "PUT", None).await;
        erd::post("/api/v1/stops/13", String::from(""), "DELETE",Some(&[("Authorization", "Bearer Kyqc49jIM+5+D0Sed8ZQ671gxkd7W/bBTWjDtZ0Zrgk="),])).await;
        let stops = erd::fetch_json::<Vec<Stop>>("/api/v1/stops").await;
        let stop = erd::fetch_json::<Stop>("/api/v1/stops/2").await;
    });
}
