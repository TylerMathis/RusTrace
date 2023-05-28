mod accelerators;
mod core;
mod films;
mod integrators;
mod math;
mod primitives;
mod samplers;

fn main() {
    let sampler = samplers::perfect_square_sampler::PerfectSquareSampler::new(100, 100, 9);
    let _integrator = integrators::sampler_integrator::SamplerIntegrator::new(&sampler);
}
