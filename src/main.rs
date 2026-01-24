// src/main.rs
use clap::Parser;
use rand::seq::SliceRandom;
use rust_embed::RustEmbed;
use std::borrow::Cow;

#[derive(RustEmbed)]
#[folder = "assets/texts/"]
struct Assets;

#[derive(Parser)]
#[command(name = "slasher-horrorscripts")]
#[command(about = "Display ANSI art of horror icons")]
struct Cli {
    /// Show a specific character by name
    #[arg(short, long)]
    name: Option<String>,

    /// Show a random character
    #[arg(short, long)]
    random: bool,

    /// List available characters
    #[arg(short, long)]
    list: bool,

    /// Do not print the character name
    #[arg(long)]
    no_title: bool,
}

fn main() {
    let cli = Cli::parse();
    let files: Vec<String> = Assets::iter().map(|f| f.to_string()).collect();

    if files.is_empty() {
        eprintln!("No horror scripts found! Did you run the converter?");
        return;
    }

    if cli.list {
        for file in &files {
            // file is "jason.txt", print "jason"
            println!("{}", file.trim_end_matches(".txt"));
        }
        return;
    }

    let selected_file = if let Some(name) = cli.name {
        let filename = format!("{}.txt", name.to_lowercase());
        if files.contains(&filename) {
            Some(filename)
        } else {
            eprintln!("Character '{}' not found.", name);
            None
        }
    } else if cli.random || !cli.list {
        // Default to random if no args provided (like pokemon-colorscripts often does)
        let mut rng = rand::thread_rng();
        files.choose(&mut rng).cloned()
    } else {
        None
    };

    if let Some(filename) = selected_file {
        if let Some(file) = Assets::get(&filename) {
            let content = std::str::from_utf8(file.data.as_ref()).unwrap();
            print!("{}", content);

            if !cli.no_title {
                let name = filename
                    .trim_end_matches(".txt")
                    .replace('_', " ")
                    .to_uppercase();
                println!("\n    {}", name);
            }
        }
    }
}
