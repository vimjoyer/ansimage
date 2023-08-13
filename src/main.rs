mod args;

use args::ProgramArgs;
use clap::*;
use image::{imageops, DynamicImage, GenericImageView, Rgba};
use imageproc::{
    drawing::{draw_filled_rect, draw_filled_rect_mut, draw_text_mut},
    rect::Rect,
};
use regex::Regex;
use rusttype::{Font, Scale};
use std::{env, fs::File, io::Read};

fn remove_ansi_colors(input: &str) -> String {
    // Create a regular expression pattern to match RGB ANSI color codes
    let ansi_pattern = Regex::new("\x1B\\[[0-9;]*m").unwrap();
    ansi_pattern.replace_all(input, "").into()
}

fn include_bytes_runtime(path: &str) -> Option<Vec<u8>> {
    let mut f = match File::open(path) {
        Ok(file) => file,
        Err(_) => return None, // Failed to open file or file not found
    };

    let mut buffer = Vec::new();
    match f.read_to_end(&mut buffer) {
        Ok(_) => Some(buffer),
        Err(_) => None, // Failed to read from file
    }
}

fn main() {
    let args = ProgramArgs::parse();

    let font_path: String;

    match env::var("ANSIMAGE_FONT") {
        Err(_) => {
            println!("ANSIMAGE_FONT not set");
            std::process::exit(1)
        }
        Ok(value) => font_path = value,
    };

    // Load the font
    let bytes = include_bytes_runtime(&font_path).unwrap();

    let font = Font::try_from_bytes(&bytes).unwrap();

    let text = args.input.as_str();

    let result = remove_ansi_colors(text);

    let lines: Vec<&str> = result.split('\n').collect();
    let max_len = lines
        .iter()
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(0);

    // should be added as a flag
    let glyph_height = 90.0;

    // umm... uhhhh... yes
    let character = 'A';
    let scale = Scale::uniform(glyph_height);
    let glyph = font.glyph(character).scaled(scale);

    let glyph_width = glyph.h_metrics().advance_width.ceil() as u32;
    let space_scale = Scale {
        x: glyph_width as f32,
        y: glyph_height as f32,
    };

    let mut img = DynamicImage::new_rgba16(
        (max_len as u32) * (glyph_width as u32),
        (lines.len() as u32) * (glyph_height as u32),
    );

    let (width, height) = img.dimensions();

    // hard coded to gruvbox color, should be added as a flag
    draw_filled_rect_mut(
        &mut img,
        Rect::at(0, 0).of_size(width, height),
        Rgba([40, 40, 40, 255]),
    );

    let mut color: Rgba<u8> = Rgba([0, 0, 0, 255]);

    let mut x = 0;
    let mut y = 0;

    let mut current = 0;
    let max = text.chars().count();

    // horrible part, but it works
    while current < max {
        let c = text.chars().nth(current).unwrap();

        if c == '\u{001b}' {
            let chars_left = max - current;
            let lower = std::cmp::min(chars_left, 23);
            let sub: String = text.chars().take(current + lower).skip(current).collect();

            let pattern = Regex::new(r"\[[0-9;]*m").unwrap();

            if let Some(regex) = pattern.find(&sub) {
                let regex_text = regex.as_str();
                let regex_length = regex_text.chars().count();

                current += regex_length + 1;
                let new_string = &regex_text[1..regex_text.chars().count() - 1];
                let split_string: Vec<&str> = new_string.split(';').collect();
                match split_string.len() {
                    1 => {
                        if split_string[0] == "0" {
                            color = Rgba([255, 255, 255, 255])
                        }
                    }
                    5 => {
                        let r = split_string[2].parse::<u8>().unwrap();
                        let g = split_string[3].parse::<u8>().unwrap();
                        let b = split_string[4].parse::<u8>().unwrap();
                        color = Rgba([r, g, b, 255])
                    }
                    6 => {
                        let r = split_string[3].parse::<u8>().unwrap();
                        let g = split_string[4].parse::<u8>().unwrap();
                        let b = split_string[5].parse::<u8>().unwrap();
                        color = Rgba([r, g, b, 255])
                    }
                    _ => {}
                }
                continue;
            }
        }

        if c == '\n' {
            y += space_scale.y as i32;
            x = 0;
            current += 1;
            continue;
        }

        draw_text_mut(&mut img, color, x, y, scale, &font, &c.to_string());
        x += space_scale.x as i32;
        current += 1;
    }

    // Save the image as a PNG
    let png_output = img.to_rgb16();
    png_output.save(args.output_file).unwrap();
}
