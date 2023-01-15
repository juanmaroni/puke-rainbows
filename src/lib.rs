use std::f64::consts::PI;

// Print text with RGB colors
pub fn print_colored(text: String) {
    println!("{}", convert_to_colored(text));
}

// Iterate chars from a String and add color
fn convert_to_colored(text: String) -> String {
    text.chars()
        .enumerate()
        .map(|(i, c)| {
            let (r, g, b) = get_rgb_color(i);
            format!("\x1b[38;2;{r};{g};{b}m{c}\x1b[0m") // ANSI
        })
        .collect::<String>()
}

// Get a color using trigonometry
fn get_rgb_color(i: usize) -> (u8, u8, u8) {
    let i = i as f64;

    // These angles are chosen because they are 120 degrees apart on
    // the trigonometric circle, which produces complementary colors.
    (calc_color_channel(i, 0.0) as u8,
    calc_color_channel(i, 2.0) as u8,
    calc_color_channel(i, 4.0) as u8)
}

fn calc_color_channel(i: f64, angle: f64) -> f64 {
    // Change this and see what happens
    let factor = 0.1;

    (factor * i + angle * PI / 3.0).sin() * 127.0 + 128.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc_color_channel_test() {
        assert_eq!(calc_color_channel(0.0, 0.0), 128.0);
    }

    #[test]
    fn get_rgb_color_test() {
        assert_eq!(get_rgb_color(0), (128, 237, 18));
    }

    #[test]
    fn convert_to_colored_test() {
        assert_eq!(convert_to_colored(String::from("a")), format!("\x1b[38;2;128;237;18ma\x1b[0m"));
    }
}
