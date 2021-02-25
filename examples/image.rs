use ufb::{ColorDepth, Window};
use image::GenericImageView;

fn main() {
    let img = image::open("screenshots/image.jpg").unwrap();
    let (w, h) = img.dimensions();

    let mut win = Window::new(w, h, "Hello").unwrap();
    win.show(&img.to_rgba8(), ColorDepth::Rgba8).unwrap();
}