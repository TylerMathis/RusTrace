use crate::core::vector::{Point3f, Vec3f};

/////////////////////
// BEGIN INTERFACE //
/////////////////////

/// An instance of the simple geometric [Ray].
pub struct Ray {
    /// The origin
    o: Point3f,

    /// The direction, often normalized but not guaranteed
    d: Vec3f,
}

//////////////////////////
// END INTERFACE        //
// BEGIN IMPLEMENTATION //
//////////////////////////

////////////////////////
// END IMPLEMENTATION //
// BEGIN TESTS        //
////////////////////////

///////////////
// END TESTS //
///////////////
