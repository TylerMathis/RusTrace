use crate::core::accelerator::Accelerator;
use crate::core::interaction::Interaction;
use crate::core::primitive::Primitive;
use crate::core::ray::Ray;

/////////////////////
// BEGIN INTERFACE //
/////////////////////

pub struct SimpleList<'a> {
    primitives: &'a Vec<&'a dyn Primitive>,
}

//////////////////////////
// END INTERFACE        //
// BEGIN IMPLEMENTATION //
//////////////////////////

impl<'a> SimpleList<'a> {
    pub fn new(primitives: &'a Vec<&'a dyn Primitive>) -> Self {
        Self { primitives }
    }
}

impl<'a> Accelerator<'a> for SimpleList<'a> {
    fn build(&mut self, primitives: &'a Vec<&'a dyn Primitive>) {
        self.primitives = primitives;
    }

    fn test(&self, ray: &Ray) -> Option<Interaction> {
        let mut closest_interaction: Option<Interaction> = None;

        for primitive in self.primitives {
            if let Some(interaction) = primitive.test(ray) {
                if closest_interaction.is_none() || interaction < closest_interaction.unwrap() {
                    closest_interaction = Some(interaction);
                }
            }
        }

        closest_interaction
    }
}

////////////////////////
// END IMPLEMENTATION //
// BEGIN TESTS        //
////////////////////////

///////////////
// END TESTS //
///////////////
