use ufb::{ColorDepth, Window};

const WIDTH: u32 = 768;
const HEIGHT: u32 = 768;

fn main() { 
    let mut win = Window::new(WIDTH, HEIGHT, "Hello", ColorDepth::Rgb8).unwrap();
    for (iter, pixel) in win.get_frame().chunks_exact_mut(3).enumerate() {
        let x = iter % WIDTH as usize;
        let y = iter / WIDTH as usize;
        let val = x ^ y;
        let hex = format!("{:06x}", val);
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap();
        pixel.copy_from_slice(&[r, g, b]);
    }
    win.show();
}