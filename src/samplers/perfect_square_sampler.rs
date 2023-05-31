/////////////////////
// BEGIN INTERFACE //
/////////////////////

use crate::core::{sample::Sample, sampler::Sampler};

/// Implemntation of [Sampler] that samples evenly within a pixel
pub struct PerfectSquareSampler {
    samples_per_row: usize,
    samples_per_col: usize,
    samples: usize,
    current_sample: usize,
}

//////////////////////////
// END INTERFACE        //
// BEGIN IMPLEMENTATION //
//////////////////////////

impl PerfectSquareSampler {
    pub fn new(width: usize, height: usize, sqrt_samples_per_pixel: usize) -> Self {
        let samples_per_row = width * sqrt_samples_per_pixel;
        let samples_per_col = height * sqrt_samples_per_pixel;

        Self {
            samples_per_row,
            samples_per_col,
            samples: samples_per_row * samples_per_col,
            current_sample: 0,
        }
    }
}

impl Sampler for PerfectSquareSampler {
    fn next_sample(&mut self) -> Option<Sample> {
        if self.current_sample < self.samples {
            let y =
                (self.current_sample / self.samples_per_row) as f64 / self.samples_per_col as f64;
            let x =
                (self.current_sample % self.samples_per_row) as f64 / self.samples_per_row as f64;

            self.current_sample += 1;
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
