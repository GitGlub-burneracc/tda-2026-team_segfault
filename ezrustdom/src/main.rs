// Command-line utility for creating a starter page inside a Cargo project.
use std::fs;
use std::path::Path;
use std::process::Command;

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
ezrustdom = { path = "../../ezrustdom" }
wasm-bindgen = "0.2"

[dependencies.web-sys]
version = "0.3"
features = [
    "Window",
    "Document",
    "Element",
    "Response",
]

[workspace]
"#,
    ).expect("Filesystem didnt write");

}

pub fn compile_pages() {
    let erd_dir = Path::new("erd");

    if !erd_dir.is_dir() {
        return;
    }

    for entry in fs::read_dir(erd_dir).expect("failed to read erd directory") {
        let page_dir = entry.expect("failed to read page entry").path();

        if !page_dir.is_dir() {
            continue;
        }

        let manifest = page_dir.join("Cargo.toml");

        if !manifest.is_file() {
            continue;
        }

        // Rust -> WASM
        let status = Command::new("cargo")
            .args([
                "build",
                "--manifest-path",
                manifest.to_str().expect("invalid manifest path"),
                "--target",
                "wasm32-unknown-unknown",
                "--release",
            ])
            .status()
            .expect("failed to run cargo");

        assert!(status.success(), "failed to compile page WASM");

        // Get package name from Cargo.toml
        let cargo_toml =
            fs::read_to_string(&manifest).expect("failed to read Cargo.toml");

        let package_name = cargo_toml
            .lines()
            .find_map(|line| {
                let line = line.trim();

                line.strip_prefix("name = ")
                    .map(|name| name.trim_matches('"').to_string())
            })
            .expect("missing package name");

        let wasm_path = page_dir
            .join("target")
            .join("wasm32-unknown-unknown")
            .join("release")
            .join(format!("{package_name}.wasm"));

        // Temporary wasm-bindgen output
        let generated = page_dir.join(".generated");

        if generated.exists() {
            fs::remove_dir_all(&generated)
                .expect("failed to remove old generated files");
        }

        fs::create_dir_all(&generated)
            .expect("failed to create generated directory");

        // WASM -> browser WASM + JS
        let status = Command::new("wasm-bindgen")
            .args([
                wasm_path.to_str().expect("invalid WASM path"),
                "--target",
                "web",
                "--out-dir",
                generated.to_str().expect("invalid output path"),
            ])
            .status()
            .expect("failed to run wasm-bindgen");

        assert!(status.success(), "wasm-bindgen failed");

        // Move the normal wasm-bindgen output into the page directory.
        fs::rename(
            generated.join(format!("{package_name}.js")),
            page_dir.join("wasm.js"),
        )
        .expect("failed to create wasm.js");

        fs::rename(
            generated.join(format!("{package_name}_bg.wasm")),
            page_dir.join("wasm_bg.wasm"),
        )
        .expect("failed to create wasm_bg.wasm");

        fs::remove_dir_all(&generated)
            .expect("failed to remove generated directory");
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 2 && args[1] == "add" {
        add_page(&args[2])
    } else if args.len() == 2 && args[1] == "compile" {
        compile_pages();
    }
}
