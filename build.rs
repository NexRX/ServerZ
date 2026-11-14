use std::{fs, path::Path, process::Command};

const BUILD_ASSETS: &str = "target/build-assets";
const PACKAGE_JSON: &str = r#"{
    "private": true,
    "dependencies": {
        "daisyui": "5.7.50",
        "@iconify/tailwind4": "1.2.3",
        "@iconify-json/solar": "1.2.13",
        "@tailwindcss/cli": "4.3.3"
    }
}"#;

fn main() {
    ensure_tailwind_deps();

    topcoat::tailwind::BuildConfig::new()
        .executable(tailwind())
        .input("styles.css")
        .minify(!cfg!(debug_assertions))
        .optimize(!cfg!(debug_assertions))
        .render()
        .unwrap();

    // topcoat::icon::iconify::BuildConfig::new()
    //     .cache_dir(BUILD_ASSETS)
    //     .icon_set("solar")
    //     .icon_set("lucide")
    //     .stage()
    //     .expect("failed to stage the local Solar icon set");
}

fn ensure_tailwind_deps() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let directory = manifest_dir.join(BUILD_ASSETS).join("tailwind_deps");
    let package_json = directory.join("package.json");
    let installed_daisyui = directory.join("node_modules/daisyui/package.json");
    let installed_iconify = directory.join("node_modules/@iconify/tailwind4/package.json");
    let installed_cli = directory.join("node_modules/@tailwindcss/cli/package.json");
    let npmrc = directory.join(".npmrc");
    let workspace_config = directory.join("pnpm-workspace.yaml");

    println!("cargo:rerun-if-changed={}", package_json.display());

    fs::create_dir_all(&directory).expect("could not create DaisyUI build directory");
    fs::write(&npmrc, "node-linker=hoisted\n").expect("could not write pnpm configuration");
    fs::write(
        &workspace_config,
        "allowBuilds:\n  '@parcel/watcher': true\n",
    )
    .expect("could not write pnpm build policy");
    fs::write(&package_json, PACKAGE_JSON).expect("could not write Tailwind package manifest");

    if installed_daisyui.is_file() && installed_iconify.is_file() && installed_cli.is_file() {
        return;
    }

    let (pkg_mgr, pkg_mgr_args) = js_pkg_manager();

    let status = Command::new(pkg_mgr)
        .current_dir(&directory)
        .args(pkg_mgr_args)
        .status()
        .expect("Failed to install DaisyUI dependacies");
    if !status.success() {
        panic!("npm failed while installing DaisyUI");
    }
}

fn tailwind() -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let dependency_dir = manifest_dir.join(BUILD_ASSETS).join("tailwind_deps");
    let wrapper = manifest_dir.join(BUILD_ASSETS).join("tailwindcss");
    let dependency_dir = dependency_dir.display();

    if !wrapper.is_file() {
        fs::write(
            &wrapper,
            format!(
                "#!/bin/sh\nexec pnpm --dir '{}' exec tailwindcss \"$@\"\n",
                dependency_dir
            ),
        )
        .expect("could not create pnpm Tailwind wrapper");
    }

    Command::new("chmod")
        .args(["+x", wrapper.to_str().expect("invalid wrapper path")])
        .status()
        .expect("failed to make pnpm Tailwind wrapper executable");

    wrapper.to_string_lossy().into_owned()
}

// ----- utils -----

fn js_pkg_manager() -> (&'static str, &'static [&'static str]) {
    const PNPM: (&str, &[&str]) = (
        "pnpm",
        &[
            "install",
            "--no-frozen-lockfile",
            "--lockfile=false",
            "--node-linker=hoisted",
        ],
    );
    const NPM: (&str, &[&str]) = (
        "npm",
        &["install", "--ignore-scripts", "--no-audit", "--no-fund"],
    );

    Command::new("pnpm")
        .arg("--version")
        .status()
        .is_ok_and(|status| status.success())
        .then_some(PNPM)
        .or_else(|| {
            Command::new("npm")
                .arg("--version")
                .status()
                .is_ok_and(|status| status.success())
                .then_some(NPM)
        })
        .expect("Could not find pnpm or npm; please install pnpm or npm")
}
