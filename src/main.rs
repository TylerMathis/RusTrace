mod accelerators;
mod cameras;
mod core;
mod films;
mod integrators;
mod math;
mod primitives;
mod samplers;

use crate::core::primitive::Primitive;
use crate::core::vector::{Point3f, Vec3f};

fn build_camera() -> cameras::perspective_camera::PerspectiveCamera {
    let origin = Point3f::new(0.0, 0.0, 0.0);
    let look_at = Point3f::new(1.0, 0.0, 0.0);
    let up = Vec3f::new(0.0, 1.0, 0.0);
    let vertical_fov = 70.0;

    let aspect_ratio = 1.0;
    let aperture = 0.0;
    let focus_distance = (look_at - origin).length();

    cameras::perspective_camera::PerspectiveCamera::new(
        origin,
        look_at,
        up,
        vertical_fov,
        aspect_ratio,
        aperture,
        focus_distance,
    )
}

fn main() {
    let mut sampler = samplers::perfect_square_sampler::PerfectSquareSampler::new(100, 100, 3);
    let mut integrator = integrators::sampler_integrator::SamplerIntegrator::new(&mut sampler);
    let camera = build_camera();

    let sphere = primitives::sphere::Sphere::new(Vec3f::new(10.0, 0.0, 0.0), 3.0);
    let mut environment = Vec::new();
    environment.push(&sphere as &dyn Primitive);
    let mut accelerator = accelerators::simple_list::SimpleList::new(&environment);

    let mut film = films::png_film::PngFilm::new(100, 100);

    let mut rustrace =
        core::rustrace::RusTrace::new(&mut integrator, &camera, &mut accelerator, &mut film);

    rustrace.render();
}
