# Slasher Horrorscripts 🔪

[![Nix Flake](https://img.shields.io/badge/Nix_Flake-Geared-dddd00?logo=nixos&logoColor=white)](https://nixos.org/manual/nix/stable/command-ref/new-cli/nix3-flake.html)

[![Nix](https://img.shields.io/badge/Nix-5277C3?style=flat&logo=nixos&logoColor=white)](https://nixos.org)

`.png` image credit https://pngegg.com

A Rust-based CLI tool that displays high-quality ANSI art of horror movie icons
in your terminal. Inspired by `pokemon-colorscripts`, but built for fans of the
macabre.

<p align="center">
  <img src="https://raw.githubusercontent.com/saylesss88/slasher-horrorscripts/main/assets/output-slasher.gif" width="600" alt="slasher-horrorscripts demo">
</p>

![screenshot1](https://raw.githubusercontent.com/saylesss88/slasher-horrorscripts/main/assets/saw.cleaned.png)

![screenshot1](https://raw.githubusercontent.com/saylesss88/slasher-horrorscripts/main/assets/demo2.png)

## ✨ Features

- **Multiple render styles**: ANSI half-blocks, Braille, ASCII, Unicode, Fade,
  FullBlock, and Sixel (pixel-accurate inline images).
- **Blazing Fast**: Written in pure Rust with embedded assets. Single binary, no
  runtime dependencies.
- **System fetch**: Display system info alongside your slasher with `--fetch`.
- **Sixel support**: Pixel-perfect inline images in compatible terminals
  (WezTerm, ghostty, foot).
- **Randomizer**: Get a different slasher every time you open your terminal.
- **Self-Contained**: Images are baked into the executable at build time.

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

2. Build & Install

```bash
cargo build --release
cp target/release/slasher-horrorscripts ~/.local/bin/
```

## Usage

Run the tool directly from your terminal, both `slasher-horrorscripts` &
`slasher` work:

```bash
# Show a random slasher (default ANSI style)
slasher

# Show a specific character
slasher --name jason

# List all available characters
slasher --list
Available Slashers:
  - chucky
  - freddy
  - it
  - jason
  - jigsaw
  - leatherface
  - mike-myers
  - pinhead
  - psycho-head
  - scarryrabbit
  - scream

# Choose a render style
slasher --style braille
slasher --style ascii
slasher --style fade
slasher --style unicode
slasher --style fullblock
slasher --style sixel        # requires a Sixel-compatible terminal

# Show system info alongside the image
slasher --fetch
slasher --fetch --style braille
slasher --fetch --style sixel

# Show system info with ASCII logo only (no image)
slasher --fetch-only

# Combine name + style
slasher --name jason --style sixel
```

## Add to Shell Startup

To see a random horror icon every time you launch your terminal, add this to
your shell config (`.bashrc`, `.zshrc`, or `config.fish`):

```bash
# Display a random slasher on startup
slasher-horrorscripts
# Random slasher w/ fetch
slasher-horrorscripts --fetch
slasher-horrorscripts --fetch --style sixel
```

---


## 🎨 Adding New Characters

Want to add Swamp Thing, or any other character?

1. Find a pixel art sprite (PNG/JPG).
   - Tip: 8-bit or 16-bit sprites with transparent backgrounds work best.
2. Save the image to `assets/embed/images/` (e.g., `swamp-thing.png`).
3. Rebuild the binary:
```bash
cargo build --release
```

That's it, the image is embedded at compile time and available immediately.

---

## 🔧 Technical Details

Rendering is powered by the [px2ansi](https://crates.io/crates/px2ansi) library.

- **Engine**: [px2ansi](https://crates.io/crates/px2ansi) handles RGB-to-ANSI
  escape sequence conversion, supporting multiple styles including half-blocks,
  Braille, ASCII, and Sixel.
- **Resizing**: Images are resized at runtime to fit your terminal, with
  per-style dimension logic (e.g. Braille uses 2×4 dot patterns, Sixel renders
  pixel-accurate).
- **Embedding**: The [rust-embed](https://crates.io/crates/rust-embed) crate
  compiles the PNG sprites in `assets/embed/images/` directly into the binary
  at build time, producing a single portable executable with no runtime
  dependencies.

  ---

## 📜 License

[MIT](https://github.com/saylesss88/slasher-horrorscripts/blob/main/LICENSE)
License - Hack away!
