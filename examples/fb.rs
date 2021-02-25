use ufb::{ColorDepth, Window};

const WIDTH: u32 = 768;
const HEIGHT: u32 = 768;

fn main() {
    let mut fb: Vec<u8> = vec![0u8; (WIDTH * HEIGHT * 3) as usize];
    for (iter, pixel) in fb.chunks_exact_mut(3).enumerate() {
        let x = iter % WIDTH as usize;
        let y = iter / WIDTH as usize;
        let val = x ^ y;
        let hex = format!("{:06x}", val);
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap();
        pixel.copy_from_slice(&[r, g, b]);
    }
    
    let mut win = Window::new(WIDTH, HEIGHT, "Hello").unwrap();
    win.show(&fb, ColorDepth::Rgb8).unwrap();
}