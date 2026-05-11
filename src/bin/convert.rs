use anyhow::Result;
use px2ansi::{RenderOptions, RenderStylePreset, ResizeFilter};
use std::fs;
use std::io::BufWriter;
use std::path::Path;

fn main() -> Result<()> {
    let img_dir = Path::new("assets/images");
    let txt_dir = Path::new("assets/texts");

    if !img_dir.exists() {
        eprintln!("Error: assets/images directory not found.");
        return Ok(());
    }

    fs::create_dir_all(txt_dir)?;

    let opts = RenderOptions::builder()
        .preset(RenderStylePreset::Ansi)
        .width(80)
        .filter(ResizeFilter::Nearest)
        .build();

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

            let out_path = txt_dir.join(format!("{stem}.txt"));
            let file = fs::File::create(out_path)?;
            let mut writer = BufWriter::new(file);

            opts.render(&img, &mut writer)?;
        }
    }

    println!("All assets converted successfully.");
    Ok(())
}
