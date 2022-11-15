use crate::core::accelerator::Accelerator;
use crate::core::camera::Camera;
use crate::core::film::Film;
use crate::core::integrator::Integrator;
use crate::core::ray::Ray;
use crate::core::sampler::Sampler;
use crate::core::vector::Color3f;

/////////////////////
// BEGIN INTERFACE //
/////////////////////

/// An [Integrator] implementation that samples screen-space coordinates
/// with the help of a [Sampler].
pub struct SamplerIntegrator {
    sampler: Box<dyn Sampler>,
}

//////////////////////////
// END INTERFACE        //
// BEGIN IMPLEMENTATION //
//////////////////////////

impl SamplerIntegrator {
    pub fn calculate_ray_color(&self, mut ray: Ray) -> Color3f {
        let mut attenuation = Color3f::new(1.0, 1.0, 1.0);

        if let Some(_) = self.accelerator.test(ray) {
            attenuation *= Color3f::new(0.8, 0.8, 0.8);
        }

        attenuation
    }
}

impl Integrator for SamplerIntegrator {
    fn render(&self, camera: &dyn Camera, accelerator: &dyn Accelerator, film: &dyn Film) {
        while let Some(mut sample) = self.sampler.next_sample() {
            let camera_ray = camera.get_ray(sample);
            sample.color = self.calculate_ray_color(camera_ray);
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
