// src/bin/convert.rs
use anyhow::Result;
use image::imageops::FilterType;
// use image::GenericImageView;
use std::fs;
use std::io::Write;
use std::path::Path;

fn main() -> Result<()> {
    let img_dir = Path::new("assets/images");
    let txt_dir = Path::new("assets/texts");

    if !img_dir.exists() {
        eprintln!("Error: assets/images directory not found.");
        return Ok(());
    }
    fs::create_dir_all(txt_dir)?;

    for entry in fs::read_dir(img_dir)? {
        let entry = entry?;
        let path = entry.path();

        // Only process PNG/JPG
        if path
            .extension()
            .is_some_and(|ext| ext == "png" || ext == "jpg")
        {
            let stem = path.file_stem().unwrap().to_string_lossy();
            println!("Processing {}...", stem);

            // 1. Open the image
            let img = image::open(&path)?;

            // 2. Resize for consistency
            // Height 40 is a good "terminal tall" size. Width is auto-calculated.
            // FilterType::Nearest preserves the "pixel art" look best.
            let resized = img.resize(u32::MAX, 40, FilterType::Nearest);

            // 3. Convert using your crate
            let ansi_art = px2ansi_rs::image_to_ansi(&resized);

            // 4. Save to text file
            let out_path = txt_dir.join(format!("{}.txt", stem));
            let mut file = fs::File::create(out_path)?;
            file.write_all(ansi_art.as_bytes())?;
        }
    }
    println!("All gory assets converted successfully.");
    Ok(())
}
