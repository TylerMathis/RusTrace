use crate::core::scene::Scene;

/////////////////////
// BEGIN INTERFACE //
/////////////////////

/// An [Integrator] is responsible for aggregating light information over a [Scene].
/// Generally used to generate a two dimensional image, but is intentionally left general
/// to support other use cases.
pub trait Integrator {
    fn render(scene: &Scene);
}

//////////////////////////
// END INTERFACE        //
// BEGIN IMPLEMENTATION //
//////////////////////////

////////////////////////
// END IMPLEMENTATION //
// BEGIN TESTS        //
////////////////////////

///////////////
// END TESTS //
///////////////
