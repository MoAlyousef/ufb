extern crate ufb;
extern crate rand;

use ufb::{ColorDepth, Window};
use rand::distributions::{Distribution, Uniform};

const WIDTH: u32 = 300;
const HEIGHT: u32 = 300;

fn main() { 
    let range = Uniform::new(0u8, 255);
    let mut rng = rand::thread_rng();

    let mut win = Window::new(WIDTH, HEIGHT, ColorDepth::Rgb8, "Noise").unwrap();

	while win.shown() {
        for (_, pixel) in win.get_frame().chunks_exact_mut(3).enumerate() {
            let r = range.sample(&mut rng);
            let g = range.sample(&mut rng);
            let b = range.sample(&mut rng);
            pixel.copy_from_slice(&[r, g, b]);
        }
        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}
