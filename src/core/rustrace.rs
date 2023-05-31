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
pub struct RusTrace<'a> {
    integrator: &'a mut dyn Integrator<'a>,
    camera: &'a dyn Camera,
    accelerator: &'a mut dyn Accelerator<'a>,
    film: &'a mut dyn Film,
}

//////////////////////////
// END INTERFACE        //
// BEGIN IMPLEMENTATION //
//////////////////////////

impl<'a> RusTrace<'a> {
    pub fn new(
        integrator: &'a mut dyn Integrator<'a>,
        camera: &'a dyn Camera,
        accelerator: &'a mut dyn Accelerator<'a>,
        film: &'a mut dyn Film,
    ) -> Self {
        Self {
            integrator,
            accelerator,
            camera,
            film,
        }
    }

    pub fn load_primitives(&mut self, primitives: &'a Vec<&'a dyn Primitive>) {
        self.accelerator.build(primitives)
    }

    pub fn render(&'a mut self) {
        self.integrator
            .render(self.camera, self.accelerator, self.film);
    }
}

////////////////////////
// END IMPLEMENTATION //
// BEGIN TESTS        //
////////////////////////

///////////////
// END TESTS //
///////////////
