use crate::core::accelerator::Accelerator;
use crate::core::camera::Camera;
use crate::core::film::Film;
use crate::core::integrator::Integrator;
use crate::core::primitive::Primitive;

/////////////////////
// BEGIN INTERFACE //
/////////////////////

/// The main runtime manager. This class should be used to manage everything about a render,
/// from construction of the instance to creation of materials
pub struct RusTrace {
    integrator: Box<dyn Integrator>,
    camera: Box<dyn Camera>,
    accelerator: Box<dyn Accelerator>,
    film: Box<dyn Film>,
}

//////////////////////////
// END INTERFACE        //
// BEGIN IMPLEMENTATION //
//////////////////////////

impl RusTrace {
    pub fn new(
        integrator: Box<dyn Integrator>,
        camera: Box<dyn Camera>,
        accelerator: Box<dyn Accelerator>,
        film: Box<dyn Film>,
    ) -> Self {
        Self {
            integrator,
            accelerator,
            camera,
            film,
        }
    }

    pub fn load_primitives(&self, primitives: Vec<&dyn Primitive>) {
        self.accelerator.build(primitives)
    }

    pub fn render(&self) {
        self.integrator
            .render(&*self.camera, &*self.accelerator, &*self.film)
    }
}

////////////////////////
// END IMPLEMENTATION //
// BEGIN TESTS        //
////////////////////////

///////////////
// END TESTS //
///////////////
