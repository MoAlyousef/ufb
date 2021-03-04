use ufb::{ColorDepth, Window};

const WIDTH: u32 = 768;
const HEIGHT: u32 = 768;

fn main() {
    let mut win = Window::new(WIDTH, HEIGHT, ColorDepth::Rgb8, "Gradient").unwrap();
    for (i, pixel) in win.get_frame().chunks_exact_mut(3).enumerate() {
        let r = (i / 3) as u8;
        let g = r;
        let b = 255 - r;
        pixel.copy_from_slice(&[r, g, b]);
    }
    win.show();
}
