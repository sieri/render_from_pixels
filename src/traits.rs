use num_traits::{Float, NumCast, NumOps, Unsigned};
use std::fmt::Display;

///generic number trait for the render_from_pixels library

///m
pub trait Number: NumOps + Display + Copy + NumCast {}

impl<T: NumOps + Display + Copy + NumCast> Number for T {}

///Numbers compatible with pixels, unsigned ints
pub trait PixelNumber: Unsigned + Number {}

impl<T: Unsigned + Number> PixelNumber for T {}

///Floating point numbers
pub trait FloatNumber: Float + Number {}

impl<T: Float + Number> FloatNumber for T {}
