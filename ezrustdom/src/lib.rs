use std::fs;
use std::path::Path;
use std::process::Command;
use js_sys::Promise;
use wasm_bindgen_futures::JsFuture;
use web_sys::window;

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