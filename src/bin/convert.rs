use anyhow::Result;
use px2ansi::{RenderOptions, RenderStylePreset, ResizeFilter};
use std::fs;
use std::io::BufWriter;
use std::path::Path;

const PRESETS: &[(&str, RenderStylePreset)] = &[
    ("ansi", RenderStylePreset::Ansi),
    ("braille", RenderStylePreset::Braille),
    ("ascii", RenderStylePreset::Ascii),
    ("unicode", RenderStylePreset::Unicode),
    ("fade", RenderStylePreset::Fade),
];

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

            for (name, preset) in PRESETS {
                let opts = RenderOptions::builder()
                    .preset(*preset)
                    .width(40)
                    .filter(ResizeFilter::Nearest)
                    .build();

                let out_path = txt_dir.join(format!("{stem}.{name}.txt"));
                let file = fs::File::create(&out_path)?;
                let mut writer = BufWriter::new(file);
                opts.render(&img, &mut writer)?;
                println!(" -> {}", out_path.display());
            }
        }
    }

    println!("All assets converted successfully.");
    Ok(())
}
