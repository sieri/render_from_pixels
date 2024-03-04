use pixels::wgpu::Color;
use pixels::{Pixels, SurfaceTexture};

use crate::traits::PixelNumber;

pub trait Canvas<T>
where
    T: PixelNumber,
    usize: From<T>,
{
    fn force_update(&self);
    fn update(&self);
    fn index_at(&self, x: T, y: T) -> usize {
        (y * self.width() + x).into()
    }
    fn width(&self) -> T;
    fn height(&self) -> T;

    fn change_pixel(&mut self, x: T, y: T, colour: Color);
}

pub struct WindowCanvas {
    pub(super) pixels: Pixels,
    width: usize,
    height: usize,
}

impl WindowCanvas {
    pub fn new<
        T: pixels::raw_window_handle::HasRawWindowHandle
            + pixels::raw_window_handle::HasRawDisplayHandle,
    >(
        width: usize,
        height: usize,
        surface_texture: SurfaceTexture<T>,
    ) -> Self {
        Self {
            pixels: Pixels::new(width.clone() as u32, height.clone() as u32, surface_texture)
                .unwrap(),
            width,
            height,
        }
    }
}

impl Canvas<usize> for WindowCanvas {
    fn force_update(&self) {}

    fn update(&self) {}

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn change_pixel(&mut self, x: usize, y: usize, colour: Color) {
        let pixel_index = self.index_at(x, y) * 4;

        let pixel = &mut self.pixels.frame_mut()[pixel_index..pixel_index + 4];
        set_pixel(pixel, colour);
    }
}

fn set_pixel(pixel: &mut [u8], colour: Color) {
    pixel[0] = (colour.r * 255f64) as u8;
    pixel[1] = (colour.g * 255f64) as u8;
    pixel[2] = (colour.b * 255f64) as u8;
    pixel[3] = (colour.a * 255f64) as u8;
}
