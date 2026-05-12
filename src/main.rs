#![allow(clippy::multiple_crate_versions)]
use anyhow::Result;
use clap::Parser;
use px2ansi::{RenderOptions, RenderStylePreset};
use rand::prelude::IndexedRandom;
use rust_embed::RustEmbed;

/// System information and fetch logic.
pub mod fetch;

/// Static assets embedded into the binary at compile time.
#[derive(RustEmbed)]
#[folder = "assets/embed/images/"]
struct Assets;

/// Command-line arguments for the Slasher CLI.
#[derive(Parser)]
#[command(name = "slasher")]
#[command(about = "Horror-themed script manager", long_about = None)]
struct Cli {
    /// Character name (e.g. "jason", "freddy"). If omitted, a random slasher is chosen.
    #[arg(short, long)]
    name: Option<String>,
    /// List all available slashers
    #[arg(short, long)]
    list: bool,
    /// Show system info only, no image
    #[arg(long)]
    fetch_only: bool,
    /// Show system info alongside the image
    #[arg(long)]
    fetch: bool,
    /// Render style: ansi, braille, ascii, unicode, fade, fullblock, sixel
    #[arg(long, default_value = "ansi")]
    style: String,
}

/// Maps a string input to a corresponding `RenderStylePreset`.
/// Defaults to `Ansi` if the input is unrecognized.
fn parse_preset(s: &str) -> RenderStylePreset {
    match s.to_lowercase().as_str() {
        "braille" => RenderStylePreset::Braille,
        "ascii" => RenderStylePreset::Ascii,
        "unicode" => RenderStylePreset::Unicode,
        "fade" => RenderStylePreset::Fade,
        "fullblock" => RenderStylePreset::FullBlock,
        "sixel" => RenderStylePreset::Sixel,
        _ => RenderStylePreset::Ansi,
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Retrieve list of embedded image filenames
    let files: Vec<String> = Assets::iter().map(|f| f.to_string()).collect();

    if files.is_empty() {
        eprintln!("No assets found. Add images to assets/images/.");
        return Ok(());
    }

    // Handle the --list flag
    if cli.list {
        println!("Available Slashers:");
        for file in &files {
            let name = std::path::Path::new(file)
                .file_stem()
                .map_or_else(|| file.clone(), |s| s.to_string_lossy().to_string());

            println!("  - {name}");
        }
        return Ok(());
    }

    // Handle the --fetch-only flag (no image processing needed)
    if cli.fetch_only {
        fetch::print_fetch();
        return Ok(());
    }

    // Determine which image to render: specific name or random selection
    let target = cli.name.map_or_else(
        || {
            let mut rng = rand::rng();
            files.choose(&mut rng).unwrap().clone()
        },
        |n| {
            // Match against actual filenames with any extension
            let lower = n.to_lowercase();
            files
                .iter()
                .find(|f| {
                    std::path::Path::new(f)
                        .file_stem()
                        .is_some_and(|s| s.to_string_lossy().to_lowercase() == lower)
                })
                .cloned()
                .unwrap_or_else(|| format!("{lower}.png"))
        },
    );

    // Load the image from the embedded virtual filesystem
    let Some(file) = Assets::get(&target) else {
        let name = target.trim_end_matches(|c: char| c == '.' || c.is_alphabetic());
        eprintln!("Slasher '{name}' not found.");
        return Ok(());
    };

    let fmt = image::ImageFormat::from_path(&target).unwrap_or(image::ImageFormat::Png);
    let img = image::load(std::io::Cursor::new(file.data.as_ref()), fmt)?;

    // Configure and execute the terminal render
    let preset = parse_preset(&cli.style);
    let opts = RenderOptions::builder().preset(preset).build();
    let mut stdout = std::io::stdout();

    // Handle the --fetch flag
    if cli.fetch {
        fetch::print_fetch_with_image(&img, &opts, &mut stdout)?;
    } else {
        opts.render_centered(&img, &mut stdout)?;
    }

    Ok(())
}
