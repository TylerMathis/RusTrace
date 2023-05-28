use crate::core::accelerator::Accelerator;
use crate::core::camera::Camera;
use crate::core::film::Film;

/////////////////////
// BEGIN INTERFACE //
/////////////////////

/// Responsible for aggregating light information.
pub trait Integrator {
    fn render(&self, camera: &dyn Camera, accelerator: &dyn Accelerator, film: &mut dyn Film);
}

////////////////////
// END INTERFACE //
///////////////////
