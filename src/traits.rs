use num_traits::{NumOps, Unsigned};
use std::fmt::Display;

///generic number trait for the render_from_pixels library
pub trait Number: NumOps + Display + Copy {}

impl<T: NumOps + Display + Copy> Number for T {}

pub trait PixelNumber: Unsigned + Number {}

impl<T: Unsigned + Number> PixelNumber for T {}
