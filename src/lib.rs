use std::f64::consts::PI;

// Print text with RGB colors
pub fn print_colored(text: String) {
    text.chars()
        .enumerate()
        .for_each(|(i, c)| {
            let (r, g, b) = get_rgb_color(i);
            print!("\x1b[38;2;{r};{g};{b}m{c}\x1b[0m"); // ANSI
        });
    print!("\n");
}

// Get a color using trigonometry
fn get_rgb_color(i: usize) -> (u8, u8, u8) {
    let factor = 0.1;
    let i = i as f64;

    // These angles are chosen because they are 120 degrees apart on
    // the trigonometric circle, which produces complementary colors.
    (((factor * i).sin() * 127.0 + 128.0) as u8,
    ((factor * i + 2.0 * PI / 3.0).sin() * 127.0 + 128.0) as u8,
    ((factor * i + 4.0 * PI / 3.0).sin() * 127.0 + 128.0) as u8)
}
