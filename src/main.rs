use clap::Parser;
use rand::prelude::IndexedRandom;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "assets/texts/"]
struct Assets;

#[derive(Parser)]
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

    let target = if let Some(n) = cli.name {
        format!("{}.txt", n.to_lowercase())
    } else {
        // Pick random
        let mut rng = rand::rng();
        files.choose(&mut rng).unwrap().to_string()
    };

    if let Some(file) = Assets::get(&target) {
        let art = std::str::from_utf8(file.data.as_ref()).unwrap();
        println!("{}", art);
    } else {
        eprintln!("Slasher '{}' not found.", target.trim_end_matches(".txt"));
    }
}
