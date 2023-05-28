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
pub struct SamplerIntegrator<'a> {
    sampler: &'a dyn Sampler,
}

//////////////////////////
// END INTERFACE        //
// BEGIN IMPLEMENTATION //
//////////////////////////

impl<'a> SamplerIntegrator<'a> {
    pub fn new(sampler: &'a dyn Sampler) -> Self {
        Self { sampler }
    }

    pub fn calculate_ray_color(&self, accelerator: &dyn Accelerator, ray: &mut Ray) -> Color3f {
        let mut attenuation = Color3f::new(1.0, 1.0, 1.0);

        if let Some(_) = accelerator.test(ray) {
            attenuation *= Color3f::new(0.8, 0.8, 0.8);
        }

        attenuation
    }
}

impl<'a> Integrator for SamplerIntegrator<'a> {
    fn render(&self, camera: &dyn Camera, accelerator: &dyn Accelerator, film: &mut dyn Film) {
        while let Some(mut sample) = self.sampler.next_sample() {
            let mut camera_ray = camera.get_ray(&sample);
            sample.color = self.calculate_ray_color(accelerator, &mut camera_ray);
            film.add_sample(&sample);
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
