// src/bin/convert.rs
use anyhow::Result;
use image::imageops::FilterType;
use std::fs;
use std::io::BufWriter; // Import BufWriter for speed
use std::path::Path;

// CHANGE 1: Use the new function name from your updated crate
use px2ansi_rs::write_ansi_art;

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
            .map(|ext| ext == "png" || ext == "jpg")
            .unwrap_or(false) // Safe unwrapping for extension check
        {
            let stem = path.file_stem().unwrap().to_string_lossy();
            println!("Processing {}...", stem);

            // 1. Open the image
            // Note: Since these are assets, we assume they fit in memory.
            let img = image::open(&path)?;

            // 2. Resize for consistency
            // Height 40 is a good "terminal tall" size. Width is auto-calculated.
            // FilterType::Nearest preserves the "pixel art" look best.
            let resized = img.resize(u32::MAX, 40, FilterType::Nearest);

            // 3. Save to text file
            let out_path = txt_dir.join(format!("{}.txt", stem));
            let file = fs::File::create(out_path)?;
            
            // CHANGE 2: Wrap file in BufWriter
            // Writing small ANSI codes directly to disk byte-by-byte is slow.
            let mut writer = BufWriter::new(file);

            // CHANGE 3: Use the new streaming API
            // Instead of allocating a huge String in memory, we write directly to the file.
            write_ansi_art(&resized, &mut writer)?;
        }
    }
    println!("All gory assets converted successfully.");
    Ok(())
}
