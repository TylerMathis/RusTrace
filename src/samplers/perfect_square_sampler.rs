/////////////////////
// BEGIN INTERFACE //
/////////////////////

use crate::core::{sample::Sample, sampler::Sampler};

/// Implemntation of [Sampler] that samples evenly within a pixel
pub struct PerfectSquareSampler {
    width: usize,
    height: usize,

    /// Must be a perfect square
    samples_per_pixel: usize,
    sqrt_samples_per_pixel: usize,

    samples_per_row: usize,
    samples: usize,
    current_sample: usize,
}

//////////////////////////
// END INTERFACE        //
// BEGIN IMPLEMENTATION //
//////////////////////////

impl PerfectSquareSampler {
    pub fn new(width: usize, height: usize, samples_per_pixel: usize) -> Self {
        Self {
            width,
            height,
            samples_per_pixel,
            sqrt_samples_per_pixel: (samples_per_pixel as f64).sqrt() as usize,
            samples_per_row: width * samples_per_pixel,
            samples: width * height * samples_per_pixel,
            current_sample: 0,
        }
    }
}

impl Sampler for PerfectSquareSampler {
    fn next_sample(&self) -> Option<Sample> {
        if self.current_sample < self.samples {
            let pixel_row =
                (self.current_sample / self.samples_per_row) / self.sqrt_samples_per_pixel;
            let pixel_column =
                (self.current_sample % self.samples_per_row) / self.sqrt_samples_per_pixel;

            let y = (pixel_row as f64) / (self.height as f64);
            let x = (pixel_column as f64) / (self.width as f64);

            Some(Sample::new(x, y))
        } else {
            None
        }
    }
}

////////////////////////
// END IMPLEMENTATION //
// BEGIN TESTS        //
////////////////////////

///////////////
// END TESTS //
///////////////
