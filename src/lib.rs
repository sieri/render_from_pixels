pub mod canvas;
pub mod point;
pub mod traits;
pub mod window;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub use ::pixels::wgpu::Color;
