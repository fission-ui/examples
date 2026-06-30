use fission::op::Color;

pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
    Color { r, g, b, a }
}

pub fn black_alpha(alpha: u8) -> Color {
    rgba(0, 0, 0, alpha)
}

pub fn white_alpha(alpha: u8) -> Color {
    rgba(255, 255, 255, alpha)
}

pub fn color_from_hex(value: &str, fallback: Color) -> Color {
    let hex = value.trim_start_matches('#');
    if hex.len() != 6 {
        return fallback;
    }

    let parsed = u32::from_str_radix(hex, 16);
    match parsed {
        Ok(rgb) => rgba(
            ((rgb >> 16) & 0xff) as u8,
            ((rgb >> 8) & 0xff) as u8,
            (rgb & 0xff) as u8,
            255,
        ),
        Err(_) => fallback,
    }
}
