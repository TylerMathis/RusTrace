use crate::core::interaction::Interaction;

/////////////////////
// BEGIN INTERFACE //
/////////////////////

/// Defines the physical properties of some [Primitive],
/// determining how light interacts with the object.
pub trait Material {
    fn interact(&self, interaction: &Interaction);
}

///////////////////
// END INTERFACE //
///////////////////

