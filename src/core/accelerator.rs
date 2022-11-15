use crate::core::interaction::Interaction;
use crate::core::primitive::Primitive;
use crate::core::ray::Ray;

/////////////////////
// BEGIN INTERFACE //
/////////////////////

/// Responsible for testing rays against a collection of [Primitive]s.
/// The most basic implementation would test every [Primitive] in the collection
/// and return the closest.
pub trait Accelerator {
    fn build(&self, primitives: Vec<&dyn Primitive>);
    fn test(&self, ray: &Ray) -> Option<Interaction>;
}

///////////////////
// END INTERFACE //
///////////////////
