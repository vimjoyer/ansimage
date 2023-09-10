# ANSIMAGE

A simple Rust program that generates ANSI 256-colored images from text.

# Usage

## Cargo

```bash
git clone https://github.com/vimjoyer/ansimage.git
cd ansimage
cargo install --path .

ansimg --help
```

## Nix (no need to clone)

```
nix run github:vimjoyer/ansimage -- --help
```

# Flags

```
-i, --input <INPUT>                Input text file, or string
    -o, --output <OUTPUT>          Output file
    --font <FONT>                  Font to use (the full path to the font file)
    --glyph-height <GLYPH_HEIGHT>  Height of glyphs
    --force                        Force the program to overwrite the output file
    -h, --help                     Print help
    -V, --version                  Print version
```
