use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
/// Turn text into images of text.
pub struct Cli {
    /// Input text file, or string.
    #[clap(short, long)]
    pub input: Option<String>,

    /// Output file.
    #[clap(short, long)]
    pub output: String,

    /// Font to use (the full path to the font file).
    #[clap(long)]
    pub font: Option<String>,

    /// Height of glyphs.
    #[clap(long)]
    pub glyph_height: Option<f32>,

    /// Background color.
    #[clap(long)]
    pub bgcolor: Option<String>,

    /// Force the program to overwrite the output file.
    #[clap(long)]
    pub force: bool,
}
