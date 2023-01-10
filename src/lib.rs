use std::f64::consts::PI;

// Get a color using trigonometry
pub fn get_rgb_color(i: u32) -> (u8, u8, u8) {
    let factor = 0.1;
    let i = i as f64;

    // These angles are chosen because they are 120 degrees apart on
    // the trigonometric circle, which produces complementary colors.
    (((factor * i) * 127.0 + 128.0).sin() as u8,
    ((factor * i + 2.0 * PI / 3.0).sin() * 127.0 + 128.0) as u8,
    ((factor * i + 4.0 * PI / 3.0).sin() * 127.0 + 128.0) as u8)
}
