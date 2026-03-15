use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();
    if target.contains("windows") {
        generate_icon_and_embed();
    }
}

/// Generates ICO from `assets/icons/icon.png` and embeds it into the Windows executable.
fn generate_icon_and_embed() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let png_path = format!("{manifest_dir}/assets/icons/icon.png");

    println!("cargo:rerun-if-changed=assets/icons/icon.png");

    // Load source PNG (1024x1024)
    let img = image::open(&png_path).unwrap_or_else(|e| panic!("Failed to open {png_path}: {e}"));

    // Generate ICO with standard Windows icon sizes
    let ico_path = format!("{out_dir}/icon.ico");
    let mut ico_file =
        std::fs::File::create(&ico_path).expect("Failed to create icon.ico in OUT_DIR");
    let mut icon_dir = ico::IconDir::new(ico::ResourceType::Icon);

    for size in [256, 48, 32, 16] {
        let resized =
            image::imageops::resize(&img, size, size, image::imageops::FilterType::Lanczos3);
        let rgba = resized.into_raw();
        let entry = ico::IconImage::from_rgba_data(size, size, rgba);
        icon_dir.add_entry(ico::IconDirEntry::encode(&entry).expect("Failed to encode ICO entry"));
    }

    icon_dir
        .write(&mut ico_file)
        .expect("Failed to write icon.ico");

    // Generate .rc file referencing the ICO via absolute path
    let rc_path = format!("{out_dir}/icon.rc");
    let rc_content = format!("app_icon ICON \"{}\"", ico_path.replace('\\', "\\\\"));
    std::fs::write(&rc_path, rc_content).expect("Failed to write icon.rc");

    let _ = embed_resource::compile(&rc_path, embed_resource::NONE);
}
