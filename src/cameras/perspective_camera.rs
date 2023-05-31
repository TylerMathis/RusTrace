/////////////////////
// BEGIN INTERFACE //
/////////////////////

use std::f64::consts::PI;

use crate::core::{
    camera::Camera,
    ray::Ray,
    sample::Sample,
    vector::{self, Point3f, Vec3f},
};

pub struct PerspectiveCamera {
    origin: Point3f,
    top_left: Point3f,
    horizontal: Vec3f,
    vertical: Vec3f,
    u: Vec3f,
    v: Vec3f,
    lens_radius: f64,
}

//////////////////////////
// END INTERFACE        //
// BEGIN IMPLEMENTATION //
//////////////////////////

/// Standard perspective camera, constructed with FOV and aspect ratio
impl PerspectiveCamera {
    pub fn new(
        origin: Point3f,
        look_at: Point3f,
        up: Vec3f,
        vertical_fov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_distance: f64,
    ) -> Self {
        let theta = vertical_fov / 180.0 * PI;
        let h = (theta / 2.0).tan();

        let height = h * 2.0;
        let width = aspect_ratio * height;

        let w = (origin - look_at).normalize();
        let u = (up.cross(&w)).normalize();
        let v = w.cross(&u);

        let horizontal = u * width * focus_distance;
        let vertical = v * -height * focus_distance;
        let top_left = origin - horizontal / 2.0 - vertical / 2.0 - w * focus_distance;

        let lens_radius = aperture / 2.0;

        Self {
            origin,
            top_left,
            horizontal,
            vertical,
            u,
            v,
            lens_radius,
        }
    }
}

impl Camera for PerspectiveCamera {
    fn get_ray(&self, sample: &Sample) -> Ray {
        let random = vector::random_in_unit_disk() * self.lens_radius;
        let focus_blur_offset = self.u * random.x + self.v * random.y;

        Ray::new(
            self.origin + focus_blur_offset,
            self.top_left + self.horizontal * sample.x + self.vertical * sample.y
                - self.origin
                - focus_blur_offset,
            0.0,
            1000.0,
        )
    }
}

////////////////////////
// END IMPLEMENTATION //
// BEGIN TESTS        //
////////////////////////

///////////////
// END TESTS //
///////////////
