use num_traits::Float;

use crate::core::vector::Point3;

/////////////////////
// BEGIN INTERFACE //
/////////////////////

/// An axis-aligned box,
/// often used to represent the minimum enclosing space of a primitive.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Bounds3<T> {
    min: Point3<T>,
    max: Point3<T>,
}

pub type Bounds3f = Bounds3<f64>;
pub type Bounds3i = Bounds3<i32>;

//////////////////////////
// END INTERFACE        //
// BEGIN IMPLEMENTATION //
//////////////////////////

impl<T> Bounds3<T> {
    pub fn new(min: Point3<T>, max: Point3<T>) -> Self {
        Self { min, max }
    }
}

impl<T: Float> Bounds3<T> {
    pub fn union(&self, other: &Self) -> Self {
        let min = Point3::new(
            self.min.x.min(other.min.x),
            self.min.y.min(other.min.y),
            self.min.z.min(other.min.z),
        );

        let max = Point3::new(
            self.max.x.max(other.max.x),
            self.max.x.max(other.max.x),
            self.max.x.max(other.max.x),
        );

        Self::new(min, max)
    }
}

////////////////////////
// END IMPLEMENTATION //
// BEGIN TESTS        //
////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let min = Point3::new(1.0, 2.0, 3.0);
        let max = Point3::new(3.0, 4.0, 5.0);
        let bounds = Bounds3::new(min, max);
        assert_eq!(bounds, Bounds3 { min, max });
    }

    #[test]
    fn union() {
        let a_min = Point3::new(0.0, 0.0, 0.0);
        let a_max = Point3::new(1.0, 1.0, 1.0);
        let a = Bounds3::new(a_min, a_max);

        let b_min = Point3::new(2.0, 2.0, 2.0);
        let b_max = Point3::new(3.0, 3.0, 3.0);
        let b = Bounds3::new(b_min, b_max);

        let union = a.union(&b);

        let res_min = Point3::new(0.0, 0.0, 0.0);
        let res_max = Point3::new(3.0, 3.0, 3.0);
        assert_eq!(union, Bounds3::new(res_min, res_max));
    }
}

///////////////
// END TESTS //
///////////////
