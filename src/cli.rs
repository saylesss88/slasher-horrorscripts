use clap::Parser;
use px2ansi::RenderStylePreset;
use std::path::PathBuf;
/// Command-line arguments for the Slasher CLI.
#[derive(Parser)]
#[command(name = "slasher", version, about = "Horror-themed script manager")]
pub struct Cli {
    /// Character name (e.g. "jason", "freddy"). If omitted, a random slasher is chosen.
    #[arg(short, long)]
    pub name: Option<String>,
    /// List all available slashers
    #[arg(short, long)]
    pub list: bool,
    /// Show system info only, no image
    #[arg(long)]
    pub fetch_only: bool,
    /// Show system info alongside the image
    #[arg(long)]
    pub fetch: bool,
    /// Render style: ansi, braille, ascii, unicode, fade, fullblock, sixel
    #[arg(long, default_value = "ansi")]
    pub style: String,
    /// Build a JSON index of images in a directory
    #[arg(long)]
    pub index: Option<PathBuf>,
    /// Output path for the index (defaults to <dir>/index.json)
    #[arg(long)]
    pub index_out: Option<PathBuf>,
}

/// Maps a string input to a corresponding `RenderStylePreset`.
/// Defaults to `Ansi` if the input is unrecognized.
#[must_use]
pub fn parse_preset(s: &str) -> RenderStylePreset {
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
