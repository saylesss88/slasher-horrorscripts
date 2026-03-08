use anyhow::Result;
use image::imageops::FilterType;
use std::fs;
use std::io::BufWriter;
use std::path::Path;

// Import OutputMode along with the function
use px2ansi_rs::{OutputMode, write_ansi_art};

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

        if path
            .extension()
            .is_some_and(|ext| ext == "png" || ext == "jpg" || ext == "jpeg")
        {
            let stem = path.file_stem().unwrap().to_string_lossy();
            println!("Processing {stem}...");

            let img = image::open(&path)?;

            // Logic for resizing
            let target_height = 40;
            let resized = if img.height() > target_height {
                // For ANSI mode, height is pixels, but terminal characters are 2-px tall.
                // So target_height 40 here actually results in 20 lines of text.
                img.resize(u32::MAX, target_height, FilterType::Nearest)
            } else {
                img
            };

            let out_path = txt_dir.join(format!("{stem}.txt"));
            let file = fs::File::create(out_path)?;
            let mut writer = BufWriter::new(file);

            // Use Ansi (half-blocks) for better resolution.
            write_ansi_art(&resized, &mut writer, OutputMode::Ansi)?;
        }
    }
    println!("All assets converted successfully.");
    Ok(())
}
