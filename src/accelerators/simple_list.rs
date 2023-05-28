use crate::core::accelerator::Accelerator;
use crate::core::interaction::Interaction;
use crate::core::primitive::Primitive;
use crate::core::ray::Ray;

/////////////////////
// BEGIN INTERFACE //
/////////////////////

struct SimpleList<'a> {
    primitives: &'a Vec<&'a dyn Primitive>,
}

//////////////////////////
// END INTERFACE        //
// BEGIN IMPLEMENTATION //
//////////////////////////

impl<'a> Accelerator<'a> for SimpleList<'a> {
    fn build(&mut self, primitives: &'a Vec<&'a dyn Primitive>) {
        self.primitives = primitives;
    }

    fn test(&self, ray: &Ray) -> Option<Interaction> {
        let mut closest_interaction_or_none: Option<Interaction> = None;

        for primitive in self.primitives {
            if let Some(interaction) = primitive.test(ray) {
                closest_interaction_or_none = match closest_interaction_or_none {
                    Some(closest_interaction) => {
                        if interaction < closest_interaction {
                            Some(interaction)
                        } else {
                            None
                        }
                    }
                    None => Some(interaction),
                }
            }
        }

        closest_interaction_or_none
    }
}

////////////////////////
// END IMPLEMENTATION //
// BEGIN TESTS        //
////////////////////////

///////////////
// END TESTS //
///////////////
