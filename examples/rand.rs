use ufb::{ColorDepth, Window};
use rand::distributions::{Distribution, Uniform};

const WIDTH: u32 = 768;
const HEIGHT: u32 = 768;

fn main() { 
    let range = Uniform::new(0u8, 255);
    let mut rng = rand::thread_rng();
    let mut win = Window::new(WIDTH, HEIGHT, ColorDepth::La8, "My Framebuffer").unwrap();
    for (_, pixel) in win.get_frame().chunks_exact_mut(2).enumerate() {
        let p = range.sample(&mut rng);
        let a = range.sample(&mut rng);
        pixel.copy_from_slice(&[p, a]);
    }
    win.show();
}