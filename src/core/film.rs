use crate::core::sample::Sample;

/////////////////////
// BEGIN INTERFACE //
/////////////////////

/// The final step of light in it's simulation.
/// Responsible for converting light information into an image.
pub trait Film {
    /// Adds a [Sample] to the composition.
    /// Implementor should discretize and aggregate the [Sample]
    fn add_sample(&mut self, sample: &Sample);

    fn develop(&self);
}

///////////////////
// END INTERFACE //
///////////////////
