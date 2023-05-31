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
    sampler: &'a mut dyn Sampler,
}

//////////////////////////
// END INTERFACE        //
// BEGIN IMPLEMENTATION //
//////////////////////////

impl<'a> SamplerIntegrator<'a> {
    pub fn new(sampler: &'a mut dyn Sampler) -> Self {
        Self { sampler }
    }

    pub fn calculate_ray_color(&self, accelerator: &dyn Accelerator, ray: &Ray) -> Color3f {
        let mut attenuation = Color3f::new(1.0, 1.0, 1.0);

        if let Some(_) = accelerator.test(ray) {
            attenuation *= Color3f::new(0.0, 0.0, 0.0);
        }

        attenuation
    }
}

impl<'a> Integrator<'a> for SamplerIntegrator<'a> {
    fn render(
        &mut self,
        camera: &dyn Camera,
        accelerator: &dyn Accelerator,
        film: &'a mut dyn Film,
    ) {
        while let Some(mut sample) = self.sampler.next_sample() {
            let camera_ray = camera.get_ray(&sample);
            sample.color = self.calculate_ray_color(accelerator, &camera_ray);
            film.add_sample(&sample);
        }
        film.develop();
    }
}

////////////////////////
// END IMPLEMENTATION //
// BEGIN TESTS        //
////////////////////////

///////////////
// END TESTS //
///////////////
