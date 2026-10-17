fn main() {
    topcoat::tailwind::BuildConfig::new()
        .executable(".cargo/build-assets/tailwindcss")
        .input("styles.css")
        .minify(!cfg!(debug_assertions))
        .optimize(!cfg!(debug_assertions))
        .render()
        .unwrap();

    topcoat::icon::iconify::BuildConfig::new()
        .cache_dir(".cargo/build-assets")
        .icon_set("solar")
        .icon_set("lucide")
        .stage()
        .expect("failed to stage the local Solar icon set");
}
