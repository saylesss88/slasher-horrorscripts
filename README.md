# Slasher Horrorscripts 🔪

[![Nix Flake](https://img.shields.io/badge/Nix_Flake-Geared-dddd00?logo=nixos&logoColor=white)](https://nixos.org/manual/nix/stable/command-ref/new-cli/nix3-flake.html)

[![Nix](https://img.shields.io/badge/Nix-5277C3?style=flat&logo=nixos&logoColor=white)](https://nixos.org)

`.png` image credit https://pngegg.com

A Rust-based CLI tool that displays high-quality ANSI art of horror movie icons
in your terminal. Inspired by `pokemon-colorscripts`, but built for fans of the
macabre.

![screenshot1](https://raw.githubusercontent.com/saylesss88/slasher-horrorscripts/main/assets/demo1.png)

![screenshot1](https://raw.githubusercontent.com/saylesss88/slasher-horrorscripts/main/assets/demo2.png)

## ✨ Features

- **Blazing Fast**: Written in pure Rust with embedded assets (single binary, no
  runtime dependencies).

- **High Fidelity**: Uses a custom image-to-ANSI engine (px2ansi-rs) for crisp
  pixel-perfect rendering.

- **Self-Contained**: No external image files needed at runtime; everything is
  baked into the executable.

- **Randomizer**: Get a different slasher every time you open your terminal.

## 📦 Installation

Option 1: Install via Cargo (Recommended)

Get the default slashers (Jason, Freddy, etc.) immediately.

```bash
cargo install slasher-horrorscripts
```

Option 2: Build From Source (For Customization)

Choose this if you want to add your own characters or modify the art.

1. Clone the repository:

```bash
git clone https://github.com/saylesss88/slasher-horrorscripts.git
cd slasher-horrorscripts
cargo install --path .
```

Option 3: Nix

```bash
nix run github:saylesss88/slasher-horrorscripts
# If that doesn't work try:
nix run --no-write-lock-file github:saylesss88/slasher-horrorscripts
```

Flake input:

```nix
slasher-horrorscripts.url = "github:saylesss88/slasher-horrorscripts";
```

`environment.systemPackages`:

```nix
{ inputs, pkgs, ... }: {
inputs.slasher-horrorscripts.packages.${pkgs.stdenv.hostPlatform.system}.default
}
```

2. Generate Assets:

This step converts the raw PNG sprites in `assets/images` into optimized ANSI
text files.

```bash
cargo run --bin convert
# Or once installed simply
convert
```

3. Build & Install

```bash
cargo build --release
cp target/release/slasher-horrorscripts ~/.local/bin/
```

## Usage

Run the tool directly from your terminal:

```bash
# Show a random slasher
slasher-horrorscripts

# Show a specific character
slasher-horrorscripts --name jason

# List all available characters
slasher-horrorscripts --list

# List a random slasher with a fetch of system info
slasher-horrorscripts --fetch

# Fetch only with a simple box next to it (WIP)
slasher-horrorscripts --fetch-only
```

## Add to Shell Startup

To see a random horror icon every time you launch your terminal, add this to
your shell config (.bashrc, .zshrc, or config.fish):

```bash
# Display a random slasher on startup
slasher-horrorscripts
# Random slasher w/ fetch
slasher-horrorscripts --fetch
```

---

## 🎨 Adding New Characters

Want to add Pinhead, or any other character?

1. Find a pixel art sprite (PNG/JPG).

- Tip: 8-bit or 16-bit sprites with transparent backgrounds work best.

2. Save the image to `assets/images/` (e.g., `swampthing.png`).

3. Run the converter:

```bash
cargo run --bin convert
```

4. Rebuild the binary:

```bash
cargo build --release
```

---

## 🔧 Technical Details

This project uses a custom rendering engine, px2ansi-rs, to handle image
processing.

- **Engine**: [px2ansi-rs](https://crates.io/crates/px2ansi-rs) handles the
  RGB-to-ANSI escape sequence conversion.

- **Resizing**: Images are automatically resized to a terminal-friendly height
  (40 rows) during the conversion step to ensure consistent presentation.

- **Embedding**: The rust-embed crate compiles the generated ANSI text files
  directly into the final binary, making it portable and easy to distribute.

---

## 📜 License

[MIT](https://github.com/saylesss88/slasher-horrorscripts/blob/main/LICENSE)
License - Hack away!
