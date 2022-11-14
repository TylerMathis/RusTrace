use crate::core::interaction::Interaction;
use crate::core::ray::Ray;

/////////////////////
// BEGIN INTERFACE //
/////////////////////

/// The core trait for geometry.
pub trait Primitive {
    /// Tests a ray against this primitive.
    /// Assumes that `ray` is normalized.
    fn test(&self, ray: &Ray) -> Option<Interaction>;
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
