use crate::core::vector::{Point3f, Vec3f};

/////////////////////
// BEGIN INTERFACE //
/////////////////////

/// An instance of the simple geometric [Ray].
#[derive(Copy)]
pub struct Ray {
    /// The origin
    pub o: Point3f,

    /// The direction, often normalized but not guaranteed
    pub d: Vec3f,

    /// The minimum t that is valid on the ray
    min_t: f64,

    /// The maximum t that is valid on the ray
    max_t: f64,
}

//////////////////////////
// END INTERFACE        //
// BEGIN IMPLEMENTATION //
//////////////////////////

impl Ray {
    pub fn new(o: Point3f, d: Vec3f, min_t: f64, max_t: f64) -> Self {
        Ray { o, d, min_t, max_t }
    }

    pub fn at(&self, t: f64) -> Option<Point3f> {
        if t < self.min_t || t > self.max_t {
            return None;
        }
        Some(self.o + self.d * t)
    }
}

////////////////////////
// END IMPLEMENTATION //
// BEGIN TESTS        //
////////////////////////

///////////////
// END TESTS //
///////////////
