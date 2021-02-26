# ufb

Just quickly show or draw a framebuffer in a window, nothing else!

Why use ufb:
- Uses hardware-acceleration via OpenGL.
- To quickly debug image or framebuffer output, instead of writing to files.
- Supports L8, La8, Rgb8 and Rgba8 `&[u8]` buffers.
- Fast to build.
- Doesn't need vulkan drivers.
- Minimal interface.

## Usage
```toml
[dependencies]
ufb = "*"
```

```rust
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
```

Using the image crate:
```rust
use ufb::{ColorDepth, Window};
use image::GenericImageView;

fn main() {
    let img = image::open("screenshots/image.jpg").unwrap();
    let (w, h) = img.dimensions();

    let mut win = Window::new(w, h, "Hello").unwrap();
    win.show(&img.to_rgba8(), ColorDepth::Rgba8).unwrap();
}
```