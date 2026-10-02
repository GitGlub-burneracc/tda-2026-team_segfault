use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;

#[derive(Deserialize)]
pub struct StopResponse {
    pub id: u32,
    pub name: String,
    pub image_url: Option<String>,
    pub wheelchair_accessible: bool,
    pub has_shelter: bool,
    pub has_ticket_machine: bool,
}

#[derive(Serialize)]
pub struct StopInput {
    pub name: String,
    pub image_url: Option<String>,
    pub wheelchair_accessible: bool,
    pub has_shelter: bool,
    pub has_ticket_machine: bool,
}

#[wasm_bindgen(start)]
pub fn on_start() {
    spawn_local(async {
        let response = Request::get("/api/v1/stops")
            .send()
            .await
            .unwrap();

        let stops: Vec<StopResponse> = response
            .json()
            .await
            .unwrap();

        let document = window()
            .unwrap()
            .document()
            .unwrap();

        let card_grid = document
            .get_element_by_id("card-grid")
            .unwrap();

        if stops.is_empty() {
            let message = document.create_element("p").unwrap();

            message.set_text_content(Some("No stops found :("));
            message.set_class_name("no-stops");

            card_grid.append_child(&message).unwrap();
        } else {
            for stop in stops {
                let card = document.create_element("a").unwrap();

                card.set_attribute(
                    "href",
                    &format!("/stops/{}", stop.id),
                ).unwrap();

                card.set_class_name("stop-card");

                let image_url = stop
                    .image_url
                    .as_deref()
                    .unwrap_or("/assets/stopsImages/noStopImage.png");

                card.set_inner_html(&format!(
                    r#"
                    <img src="{}" alt="{}">
                    <h2>{}</h2>
                    "#,
                    image_url,
                    stop.name,
                    stop.name,
                ));

                card_grid.append_child(&card).unwrap();
            }
        }

    });
}