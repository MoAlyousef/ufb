//! # ufb
//!
//! Just quickly show or draw a framebuffer in a window, nothing else!
//!
//! ## Usage
//! ```toml
//! [dependencies]
//! ufb = "*"
//! ```
//!
//! ```no_run
//! use ufb::{ColorDepth, Window};
//!
//! const WIDTH: u32 = 768;
//! const HEIGHT: u32 = 768;
//!
//! fn main() {
//!     let mut fb: Vec<u8> = vec![0u8; (WIDTH * HEIGHT * 3) as usize];
//!     for (iter, pixel) in fb.chunks_exact_mut(3).enumerate() {
//!         let x = iter % WIDTH as usize;
//!         let y = iter / WIDTH as usize;
//!         let val = x ^ y;
//!         let hex = format!("{:06x}", val);
//!         let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
//!         let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
//!         let b = u8::from_str_radix(&hex[4..6], 16).unwrap();
//!         pixel.copy_from_slice(&[r, g, b]);
//!     }
//!
//!     let mut win = Window::new(WIDTH, HEIGHT, "Hello").unwrap();
//!     win.show(&fb, ColorDepth::Rgb8).unwrap();
//! }
//! ```

extern crate glfw;
use glu_sys::glu::*;

use glfw::Context;

/// ufb error types
#[derive(Debug)]
pub enum UfbError {
    /// glfw init error
    InitError(glfw::InitError),
    /// Internal error
    Internal(UfbErrorKind),
}

/// ufb error kinds
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum UfbErrorKind {
    /// Invalid ufb format
    InvalidFormat,
}

impl std::error::Error for UfbError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            _ => None,
        }
    }
}

impl std::fmt::Display for UfbError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            UfbError::InitError(ref err) => write!(f, "A glfw init error occured {:?}", err),
            UfbError::Internal(ref err) => write!(f, "An internal error occured {:?}", err),
        }
    }
}

impl From<glfw::InitError> for UfbError {
    fn from(err: glfw::InitError) -> UfbError {
        UfbError::InitError(err)
    }
}

/// Supported color depths
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum ColorDepth {
    L8 = 1,
    La8 = 2,
    Rgb8 = 3,
    Rgba8 = 4,
}

/// Wrapper around a glfw window
pub struct Window {
    glfw: glfw::Glfw,
    win: glfw::Window,
    w: u32,
    h: u32,
}

impl Window {
    /// Instantiate a window
    pub fn new(w: u32, h: u32, title: &str) -> Result<Self, UfbError> {
        let glfw = glfw::init(glfw::FAIL_ON_ERRORS)?;
        let (mut window, _) = glfw
            .create_window(w, h, title, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");
        window.set_resizable(false);
        window.make_current();
        Ok(Self {
            glfw,
            win: window,
            w,
            h,
        })
    }

    /// Show the window
    pub fn show(&mut self, fb: &[u8], depth: ColorDepth) -> Result<(), UfbError> {
        while !self.win.should_close() {
            let gl_enum = match depth {
                ColorDepth::L8 => GL_LUMINANCE,
                ColorDepth::La8 => GL_LUMINANCE_ALPHA,
                ColorDepth::Rgb8 => GL_RGB,
                ColorDepth::Rgba8 => GL_RGBA,
            };
            let depth = depth as u32;
            if fb.len() as u32 != self.w * self.h * depth {
                return Err(UfbError::Internal(UfbErrorKind::InvalidFormat));
            }
            unsafe {
                glRasterPos2i(-1, 1);
                glPixelZoom(1., -1.);
                glDrawPixels(
                    self.w as _,
                    self.h as _,
                    gl_enum,
                    GL_UNSIGNED_BYTE,
                    fb.as_ptr() as _,
                );
            }
            self.win.swap_buffers();
            self.glfw.poll_events();
            std::thread::sleep(std::time::Duration::from_millis(16));
        }
        Ok(())
    }
}
