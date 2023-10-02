pub fn color_code_to_array(color_code: &str) -> [u8; 4] {
    let color_code = color_code.trim_start_matches('#');
    let red = u8::from_str_radix(&color_code[0..2], 16).unwrap();
    let green = u8::from_str_radix(&color_code[2..4], 16).unwrap();
    let blue = u8::from_str_radix(&color_code[4..6], 16).unwrap();
    
    let alpha: u8;
    
    if color_code.len() > 6 {
        alpha = u8::from_str_radix(&color_code[6..8], 16).unwrap();
    } else {
        alpha = 255;
    }
    
    [red, green, blue, alpha]
}
