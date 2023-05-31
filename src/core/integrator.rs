use crate::core::accelerator::Accelerator;
use crate::core::camera::Camera;
use crate::core::film::Film;

/////////////////////
// BEGIN INTERFACE //
/////////////////////

/// Responsible for aggregating light information.
pub trait Integrator<'a> {
    fn render(
        &mut self,
        camera: &dyn Camera,
        accelerator: &dyn Accelerator,
        film: &'a mut dyn Film,
    );
}

////////////////////
// END INTERFACE //
///////////////////
