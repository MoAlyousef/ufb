use ufb::{ColorDepth, Window};
use image::GenericImageView;

fn main() {
    let img = image::open("screenshots/image.jpg").unwrap();
    let (w, h) = img.dimensions();
    let mut win = Window::new(w, h, ColorDepth::Rgba8, "image.jpg").unwrap();
    win.get_frame().copy_from_slice(&img.to_rgba8());
    win.show();
}