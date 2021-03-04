// modified from https://github.com/image-rs/image/blob/master/examples/fractal.rs

extern crate num_complex;

use ufb::{ColorDepth, Window};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

fn main() {
    let mut win = Window::new(WIDTH, HEIGHT, ColorDepth::Rgb8, "fractal").unwrap();
    let scalex = 3.0 / WIDTH as f32;
    let scaley = 3.0 / HEIGHT as f32;

    for (iter, pixel) in win.get_frame().chunks_exact_mut(3).enumerate() {
        let x = iter as u32 / WIDTH;
        let y = iter as u32 % WIDTH;
        let r = (0.3 * x as f32) as u8;
        let b = (0.3 * y as f32) as u8;
        pixel.copy_from_slice(&[r, 0, b]);
    }

    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let cx = y as f32 * scalex - 1.5;
            let cy = x as f32 * scaley - 1.5;

            let c = num_complex::Complex::new(-0.4, 0.6);
            let mut z = num_complex::Complex::new(cx, cy);

            let mut i = 0;
            while i < 255 && z.norm() <= 2.0 {
                z = z * z + c;
                i += 1;
            }

            let idx = (y as usize * WIDTH as usize + x as usize) * 3;
            let frame = win.get_frame();
            let r = frame[idx];
            let g = i as u8;
            let b = frame[idx + 2];
            frame[idx..idx + 3].copy_from_slice(&[r, g, b]);
        }
    }

    win.show();
}
