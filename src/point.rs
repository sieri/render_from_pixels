use std::fmt::{Display, Formatter};

use crate::traits::Number;

///Basic point and coordinates traits


pub trait Coordinate{

}


pub trait Coordinate2D: Coordinate + Display {
    ///X coordinate
    fn x(&self) -> impl Number;

    ///Y coordinate
    fn y(&self) -> impl Number;
}




struct Point2D<T> where T : Number {
    x: T,
    y: T,
}


impl<T: Number> Coordinate for Point2D<T> {}

impl<T: Number> Display for Point2D<T> where T : Number{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})",self.x,self.y)
    }
}

impl<T: Number> Coordinate2D for Point2D<T>{
    fn x(&self) -> T {
        self.x
    }

    fn y(&self) -> T {
        self.y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    ///Simple test of coordinate instantiation
    fn coordinate_instantiation() {
        let point = Point2D{ x: 1u64, y: 4u64 };

        assert_eq!(point.x(), 1u64, "Check unsigned integers X");
        assert_eq!(point.y(), 4u64, "Check unsigned integers Y");
        assert_eq!(format!("{}", point), ("(1,4)"));

        
        let point = Point2D{ x: 1i64, y: 4i64 };

        assert_eq!(point.x(), 1i64, "Check signed integers X");
        assert_eq!(point.y(), 4i64, "Check signed integers Y");
        assert_eq!(format!("{}", point), ("(1,4)"));
        let point = Point2D{ x: 1f64, y: 4f64 };

        assert_eq!(point.x(), 1f64, "Check float X");
        assert_eq!(point.y(), 4f64, "Check float Y");
        assert_eq!(format!("{}", point), ("(1,4)"));
    }
}