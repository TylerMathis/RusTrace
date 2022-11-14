use crate::core::accelerator::Accelerator;

/////////////////////
// BEGIN INTERFACE //
/////////////////////

/// Responsible for aggregating light information over an [Accelerator].
/// Generally used to generate a two dimensional image, but is intentionally left general
/// to support other use cases.
pub trait Integrator {
    fn render(&self, accelerator: &dyn Accelerator);
}

////////////////////
// END INTERFACE //
///////////////////
