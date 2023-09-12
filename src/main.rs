mod cli;
mod file;
mod misc;
mod log;
mod colors;

use std::env;
use std::io::{self, BufRead};
use clap::Parser;
use image::{DynamicImage, GenericImageView, Rgba};
use imageproc::rect::Rect;
use imageproc::drawing::{draw_filled_rect_mut, draw_text_mut};
use rusttype::{Font, Scale};
use regex::Regex;
use misc::*;
use log::*;

use crate::colors::color_code_to_array;

#[derive(PartialEq)]
enum ExitCode {
    Success,
    Fail,
}

fn real_main() -> ExitCode {
    let args = cli::Cli::parse();

    let font_path: String;

    match args.font {
        Some(s) => font_path = s,
        None => {
            match env::var("ANSIMAGE_FONT") {
                Ok(o) => font_path = o,
                Err(_) => {
                    error!("Please either specify a font path with '--font FONT_PATH', or set the environment variable: ANSIMAGE_FONT=FONT_PATH");

                    return ExitCode::Fail;
                },
            };
        },
    };

    info!("Font: {}", font_path);

    if path_exists(args.output.as_str()) {
        if args.force == false {
            error!("Cannot overwrite file! If you wish to bypass this, use: --force");

            return ExitCode::Fail;
        }

        else {
            warning!("Overwriting output file...");
        }
    }

    // Load the font
    let bytes = match include_bytes_runtime(&font_path) {
        Ok(o) => o,
        Err(_) => {
            error!("Failed to load font bytes from font file!");

            return ExitCode::Fail;
        },
    };

    let font = match Font::try_from_bytes(&bytes) {
        Some(s) => s,
        None => {
            error!("Retrieving font from bytes got an Option::None value!");

            return ExitCode::Fail;
        },
    };

    let text: String;

    match args.input {
        Some(input) => {
            text = match path_exists(input.as_str()) {
                true => {
                    info!("Input from file contents...");
                    
                    match file::read(input.as_str()) {
                        Ok(o) => o,
                        Err(_e) => return ExitCode::Fail,
                    }
                },
                false => {
                    info!("Input from string...");
                    
                    input
                },
            };
        },
        None => {
            info!("Input from stdin/pipe...");

            let mut l_string = String::new();
            for lr in io::stdin().lock().lines() {
                let l = match lr {
                    Ok(o) => o,
                    Err(_) => {
                        error!("Failed to get string from raw stdin line!");

                        return ExitCode::Fail;
                    },
                };

                l_string.push_str(format!("{}\n", l).as_str());
            }

            text = l_string.trim().to_string();
        },
    };

    let result = remove_ansi_colors(text.as_str());

    let lines: Vec<&str> = result.split('\n').collect();
    let max_len = lines.iter()
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(0);

    let glyph_height = args.glyph_height.unwrap_or(90.0);

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

    // TODO: add error handling
    let background_color = match args.bg_color {
        Some(color) => color_code_to_array(&color),
        None => [40, 40, 40, 255]
    };

    draw_filled_rect_mut(
        &mut img,
        Rect::at(0, 0).of_size(width, height),
        Rgba(background_color),
    );

    let mut color: Rgba<u8> = Rgba([255, 255, 255, 255]);

    let mut x = 0;
    let mut y = 0;

    let mut current = 0;
    let max = text.chars().count();

    // horrible part, but it works
    while current < max {
        let c = match text.chars().nth(current) {
            Some(s) => s,
            None => {
                error!("The developer failed you! Please open an issue and type: \"You have failed me! (text.chars)\"");

                return ExitCode::Fail;
            },
        };

        if c == '\u{001b}' {
            let chars_left = max - current;
            let lower = std::cmp::min(chars_left, 23);
            let sub: String = text.chars().take(current + lower).skip(current).collect();

            let pattern = match Regex::new(r"\[[0-9;]*m") {
                Ok(o) => o,
                Err(_) => {
                    error!("Failed to create new regex pattern!");

                    return ExitCode::Fail;
                },
            };

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
    match png_output.save(args.output.as_str()) {
        Ok(_) => {},
        Err(_) => {
            error!("Failed to save image!");

            return ExitCode::Fail;
        },
    };

    info!("Successfully saved to {}!", args.output);

    return ExitCode::Success;
}

fn main() {
    if real_main() == ExitCode::Fail {
        std::process::exit(1);
    }
}
