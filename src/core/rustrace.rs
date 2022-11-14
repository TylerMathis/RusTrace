/////////////////////
// BEGIN INTERFACE //
/////////////////////

use crate::core::accelerator::Accelerator;
use crate::core::camera::Camera;
use crate::core::integrator::Integrator;
use crate::core::primitive::Primitive;

/// The main runtime manager. This class should be used to manage everything about a render,
/// from construction of the instance to creation of materials
pub struct RusTrace {
    integrator: Box<dyn Integrator>,
    accelerator: Box<dyn Accelerator>,
    camera: Box<dyn Camera>,
}

//////////////////////////
// END INTERFACE        //
// BEGIN IMPLEMENTATION //
//////////////////////////

impl RusTrace {
    pub fn new(
        integrator: Box<dyn Integrator>,
        accelerator: Box<dyn Accelerator>,
        camera: Box<dyn Camera>,
    ) -> Self {
        Self {
            integrator,
            accelerator,
            camera,
        }
    }

    pub fn load_primitives(&self, primitives: Vec<&dyn Primitive>) {
        self.accelerator.build(primitives)
    }

    pub fn render(&self) {
        self.integrator.render(&*self.accelerator)
    }
}

////////////////////////
// END IMPLEMENTATION //
// BEGIN TESTS        //
////////////////////////

///////////////
// END TESTS //
///////////////
