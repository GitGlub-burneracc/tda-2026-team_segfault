// Command-line utility for creating a starter page inside a Cargo project.
use std::fs;
use std::path::Path;

// Create an HTML, CSS, Rust/WASM, and Cargo manifest scaffold under erd/<name>.
fn add_page(name: &String) {
    // Make sure we're in a Cargo project.
    if !Path::new("Cargo.toml").is_file() {
        eprintln!("not in a Cargo project");
        std::process::exit(1);
    }

    fs::create_dir_all("erd").expect("Filesystem didnt write");

    let page_dir = Path::new("erd").join(name);

    // Refuse to overwrite an existing page directory.
    if page_dir.exists() {
        eprintln!("page already exists: {}", page_dir.display());
        std::process::exit(1);
    }

    fs::create_dir(&page_dir).expect("Filesystem didnt write");

    // Write a minimal HTML document that loads the sibling stylesheet.
    fs::write(
        page_dir.join("html.html"),
        "<!doctype html>\n<html>\n<head>\n    <link rel=\"stylesheet\" href=\"erd.css\">\n</head>\n<body>\n</body>\n</html>\n",
    ).expect("Filesystem didnt write");

    // Start the stylesheet empty so the page can be styled by its author.
    fs::write(
        page_dir.join("erd.css"),
        "",
    ).expect("Filesystem didnt write");

    // Provide the WASM entry point that can later add browser-side behavior.
    fs::write(
        page_dir.join("wasm.rs"),
        "use wasm_bindgen::prelude::*;\n\n#[wasm_bindgen(start)]\npub fn start() {\n}\n",
    ).expect("Filesystem didnt write");

    // Configure this page as a WebAssembly library with browser bindings.
    fs::write(
        page_dir.join("Cargo.toml"),
        r#"[package]
name = "erd-page"
version = "0.1.0"
edition = "2024"

[lib]
path = "wasm.rs"
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = "0.2"
web-sys = "0.3"
"#,
    ).expect("Filesystem didnt write");

}

fn main() {
    // Accept commands in the form `ezrustdom add <page-name>`.
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 2 {
        if args[1] == "add" {
            add_page(&args[2])
        }
    }
}
