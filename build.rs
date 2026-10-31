use reqwest::StatusCode;
use std::{
    fs, io::{ErrorKind, Write}, path::Path, process::Command,
};

const BUILD_ASSETS: &str = ".cargo/build-assets";
const DAISYUI_VERSION: &str = "5.7.50";

fn main() {
    ensure_daisyui();

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

fn ensure_daisyui() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let directory = manifest_dir.join(BUILD_ASSETS).join("daisyui");
    let package_json = directory.join("package.json");
    let installed_package = directory.join("node_modules/daisyui/package.json");

    println!("cargo:rerun-if-changed={}", package_json.display());

    fs::create_dir_all(&directory).expect("could not create DaisyUI build directory");
    if !package_json.is_file() {
        fs::write(
            &package_json,
            format!(
                "{{\n  \"private\": true,\n  \"dependencies\": {{\n    \"daisyui\": \"{DAISYUI_VERSION}\"\n  }}\n}}\n"
            ),
        )
        .expect("could not write DaisyUI package manifest");
    }

    if installed_package.is_file() {
        return;
    }

    let (pkg_mgr, args) = Command::new("pnpm").spawn().map_or_else(|err| err.kind() != ErrorKind::NotFound, |_| true)
        .then_some(("pnpm", ["install", "--ignore-scripts", "--no-frozen-lockfile",  "--lockfile=false"]))
        .or_else(|| Command::new("npm").spawn().map_or_else(|err| err.kind() != ErrorKind::NotFound, |_| true)
            .then_some(("npm", ["install", "--ignore-scripts", "--no-audit", "--no-fund"])))
        .expect("Could not find a suitable JavaScript package manager, please install npm or pnpm");


    let status = Command::new(pkg_mgr)
        .current_dir(&directory)
        .args(args)
        .status()
        .expect("Failed to install DaisyUI dependacies");
    if !status.success() {
        panic!("npm failed while installing DaisyUI");
    }
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
        "aarch64" => "-arm64",
        arch => panic!("System architecture not supported: {arch}")
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
    let download_path = format!("{}/{to}", env!("CARGO_MANIFEST_DIR"));

    // Build scripts can run on every source edit. Keep the downloaded CLI
    // out of that edit loop once it has been fetched.
    if Path::new(&download_path).is_file() {
        return download_path;
    }

    let resp = reqwest::blocking::get(from).expect("tailwindcss couldnt be downloaded");
    assert_eq!(resp.status(), StatusCode::OK, "tailwindcss download failed");
    let bytes = resp
        .bytes()
        .expect("tailwindcss download failed to convert to bytes");

    let mut file = std::fs::File::create(&download_path).expect("couldnt create tailwindcss file");
    file.write_all(bytes.as_ref())
        .expect("failed to write tailwindcss cli");

    download_path
}
