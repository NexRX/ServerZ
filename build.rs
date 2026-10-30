use reqwest::StatusCode;
use std::io::Write;

const BUILD_ASSETS: &str = ".cargo/build-assets";

fn main() {
    topcoat::tailwind::BuildConfig::new()
        .executable(tailwind())
        .input("styles.css")
        .minify(!cfg!(debug_assertions))
        .optimize(!cfg!(debug_assertions))
        .render()
        .unwrap();

    topcoat::icon::iconify::BuildConfig::new()
        .cache_dir(BUILD_ASSETS)
        .icon_set("solar")
        .icon_set("lucide")
        .stage()
        .expect("failed to stage the local Solar icon set");
}

fn tailwind() -> String {
    let filename_prefix = match std::env::consts::OS {
        "linux" => "tailwindcss-linux",
        "macos" => "tailwindcss-macos",
        "windows" => "tailwindcss-windows",
        _ => panic!("System architecture not supported"),
    };
    let arch_suffix: &str = match std::env::consts::ARCH {
        "x86_64" => "-x64",
        "arm" => "-arm64",
        _ => panic!("System architecture not supported"),
    };
    let extension = match std::env::consts::OS {
        "windows" => ".exe",
        _ => "",
    };
    let filename = format!("{filename_prefix}{arch_suffix}{extension}");
    let version = option_env!("TAILWIND_CLI_VERSION").unwrap_or("v4.3.3");
    let url = format!(
        "https://github.com/tailwindlabs/tailwindcss/releases/download/{version}/{filename}"
    );

    download(&url, ".cargo/build-assets/tailwindcss")
}

fn download(from: &str, to: &str) -> String {
    let resp = reqwest::blocking::get(from).expect("tailwindcss couldnt be downloaded");
    assert_eq!(resp.status(), StatusCode::OK, "tailwindcss download failed");
    let bytes = resp
        .bytes()
        .expect("tailwindcss download failed to convert to bytes");

    let download_path = format!("{}/{to}", env!("CARGO_MANIFEST_DIR"));
    let mut file = std::fs::File::create(&download_path).expect("couldnt create tailwindcss file");
    file.write_all(bytes.as_ref())
        .expect("failed to write tailwindcss cli");

    download_path
}
