use crate::core::ray::Ray;
use crate::core::sample::Sample;

/////////////////////
// BEGIN INTERFACE //
/////////////////////

/// Responsible for generating outgoing rays to test.
pub trait Camera {
    /// Transforms a [Sample] into an outgoing [Ray].
    fn get_ray(&self, sample: &Sample) -> Ray;
}

///////////////////
// END INTERFACE //
///////////////////
