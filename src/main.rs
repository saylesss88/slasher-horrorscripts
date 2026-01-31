#![allow(clippy::multiple_crate_versions)]
use clap::Parser;
use rand::prelude::IndexedRandom;
use rust_embed::RustEmbed;
pub mod fetch;

#[derive(RustEmbed)]
#[folder = "assets/texts/"]
struct Assets;

#[derive(Parser)]
#[command(name = "slasher")]
#[command(about = "Horror-themed script manager", long_about = None)]
#[allow(clippy::struct_excessive_bools)]
struct Cli {
    /// Character name (e.g. "jason", "freddy")
    #[arg(short, long)]
    name: Option<String>,

    /// List all slashers
    #[arg(short, long)]
    list: bool,

    /// Random (default behavior)
    #[arg(short, long)]
    random: bool,

    #[arg(long)]
    fetch: bool,

    #[arg(long)]
    fetch_only: bool,
}

fn main() {
    let cli = Cli::parse();
    let files: Vec<String> = Assets::iter().map(|f| f.to_string()).collect();

    if files.is_empty() {
        eprintln!("No assets found. Did you run 'cargo run --bin convert'?");
        return;
    }

    if cli.list {
        println!("Available Slashers:");
        for file in &files {
            println!("  - {}", file.trim_end_matches(".txt"));
        }
        return;
    }
    if cli.fetch_only {
        fetch::print_fetch(13);
        return;
    }

    let target = cli.name.map_or_else(
        || {
            // Pick random
            let mut rng = rand::rng();
            files.choose(&mut rng).unwrap().clone()
        },
        |n| format!("{}.txt", n.to_lowercase()),
    );

    if let Some(file) = Assets::get(&target) {
        let art = std::str::from_utf8(file.data.as_ref()).unwrap();

        if cli.fetch {
            fetch::print_with_left_block(art, 13);
        } else {
            println!("{art}");
        }
    } else {
        eprintln!("Slasher '{}' not found.", target.trim_end_matches(".txt"));
    }
}
