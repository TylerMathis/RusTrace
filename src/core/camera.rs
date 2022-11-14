use crate::core::ray::Ray;

/////////////////////
// BEGIN INTERFACE //
/////////////////////

/// Responsible for generating outgoing rays to test.
pub trait Camera {
    /// Transforms a screen space coordinate to an outgoing ray.
    ///
    /// # Arguments
    ///
    /// - `x` - Ratio from [0.0, 1.0] of pixel position to lens width
    /// - `y` - Ratio from [0.0, 1.0] of pixel position to lens height
    fn get_ray(&self, x: f64, y: f64) -> Ray;
}

///////////////////
// END INTERFACE //
///////////////////