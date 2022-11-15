use crate::core::accelerator::Accelerator;
use crate::core::interaction::Interaction;
use crate::core::primitive::Primitive;
use crate::core::ray::Ray;

/////////////////////
// BEGIN INTERFACE //
/////////////////////

struct SimpleList {
    primitives: Vec<Box<dyn Primitive>>,
}

//////////////////////////
// END INTERFACE        //
// BEGIN IMPLEMENTATION //
//////////////////////////

impl Accelerator for SimpleList {
    fn build(&mut self, primitives: Vec<&dyn Primitive>) {
        self.primitives = primitives.into()
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
