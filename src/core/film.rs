use crate::core::sample::Sample;

/////////////////////
// BEGIN INTERFACE //
/////////////////////

/// The final step of light in it's simulation.
/// Responsible for converting light information into an image.
pub trait Film {
    /// Adds a sample to the composition.
    /// Implementor should discretize and aggregate the sample
    fn add_sample(&self, sample: &Sample);

    fn develop(&self);
}

///////////////////
// END INTERFACE //
///////////////////
