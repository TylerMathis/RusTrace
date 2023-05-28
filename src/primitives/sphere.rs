use crate::core::interaction::Interaction;
use crate::core::primitive::Primitive;
use crate::core::ray::Ray;
use crate::core::vector::{Point3f, Vec3f};
use crate::math::{solve_quadratic, QuadraticSolution};

/////////////////////
// BEGIN INTERFACE //
/////////////////////

#[derive(Debug, PartialEq)]
pub struct Sphere {
    center: Point3f,
    radius: f64,
}

//////////////////////////
// END INTERFACE        //
// BEGIN IMPLEMENTATION //
//////////////////////////

impl Sphere {
    pub fn new(center: Point3f, radius: f64) -> Self {
        Self { center, radius }
    }

    fn normal_at_point(&self, p: Point3f) -> Vec3f {
        (p - self.center).normalize()
    }
}

impl Primitive for Sphere {
    fn test(&self, ray: &Ray) -> Option<Interaction> {
        let a = ray.d.dot(&ray.d);
        let b = 2.0 * (ray.o - self.center).dot(&ray.d);
        let c = self.center.dot(&self.center) + ray.o.dot(&ray.o)
            - 2.0 * (self.center.dot(&ray.o))
            - self.radius * self.radius;

        match solve_quadratic(a, b, c)? {
            QuadraticSolution::None => None,
            QuadraticSolution::One { x } => {
                let p = ray.at(x)?;
                Some(Interaction::new_on_surface(
                    p,
                    x,
                    self.normal_at_point(p),
                    -ray.d,
                ))
            }
            QuadraticSolution::Two { x1, x2 } => {
                let closest = x1.min(x2);
                let maybe_location = ray.at(closest);
                let (p, t) = match maybe_location {
                    None => {
                        let furthest = x1.max(x2);
                        (ray.at(furthest)?, furthest)
                    }
                    Some(location) => (location, closest),
                };
                Some(Interaction::new_on_surface(
                    p,
                    t,
                    self.normal_at_point(p),
                    -ray.d,
                ))
            }
        }
    }
}

////////////////////////
// END IMPLEMENTATION //
// BEGIN TESTS        //
////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    fn unit_sphere() -> Sphere {
        let center = Point3f::new(0.0, 0.0, 0.0);
        let radius = 1.0;
        Sphere::new(center, radius)
    }

    #[test]
    fn new() {
        let center = Point3f::new(1.0, 2.0, 3.0);
        let radius = 5.0;
        let sphere = Sphere::new(center, radius);
        assert_eq!(sphere, Sphere { center, radius });
    }

    #[test]
    fn test_no_hit() {
        let sphere = unit_sphere();
        let ray = Ray::new(
            Point3f::new(2.0, 0.0, 0.0),
            Vec3f::new(0.0, 1.0, 0.0),
            0.0,
            100.0,
        );

        let interaction = sphere.test(&ray);
        assert_eq!(interaction, None);
    }

    #[test]
    fn test_one_hit() {
        let sphere = unit_sphere();
        let ray = Ray::new(
            Point3f::new(1.0, -1.0, 0.0),
            Vec3f::new(0.0, 1.0, 0.0),
            0.0,
            100.0,
        );

        let interaction = sphere.test(&ray).unwrap();
        assert!(interaction.is_eq(&Interaction::new_on_surface(
            Point3f::new(1.0, 0.0, 0.0),
            1.0,
            Vec3f::new(1.0, 0.0, 0.0),
            Vec3f::new(0.0, -1.0, 0.0)
        )));
    }

    #[test]
    fn test_two_hits() {
        let sphere = unit_sphere();
        let ray = Ray::new(
            Point3f::new(0.0, -2.0, 0.0),
            Vec3f::new(0.0, 1.0, 0.0),
            0.0,
            100.0,
        );

        let interaction = sphere.test(&ray).unwrap();
        assert!(interaction.is_eq(&Interaction::new_on_surface(
            Point3f::new(0.0, -1.0, 0.0),
            1.0,
            Vec3f::new(0.0, -1.0, 0.0),
            Vec3f::new(0.0, -1.0, 0.0)
        )));
    }

    #[test]
    fn test_inside_out_hit() {
        let sphere = unit_sphere();
        let ray = Ray::new(
            Point3f::new(0.0, 0.0, 0.0),
            Vec3f::new(0.0, 1.0, 0.0),
            0.0,
            100.0,
        );

        let interaction = sphere.test(&ray).unwrap();
        assert!(interaction.is_eq(&Interaction::new_on_surface(
            Point3f::new(0.0, 1.0, 0.0),
            1.0,
            Vec3f::new(0.0, 1.0, 0.0),
            Vec3f::new(0.0, -1.0, 0.0)
        )));
    }

    #[test]
    fn test_outside_on_line_behind_no_hit() {
        let sphere = unit_sphere();
        let ray = Ray::new(
            Point3f::new(0.0, 5.0, 0.0),
            Vec3f::new(0.0, 1.0, 0.0),
            0.0,
            1.0,
        );

        let interaction = sphere.test(&ray);
        assert_eq!(interaction, None);
    }

    #[test]
    fn test_outside_on_line_front_no_hit() {
        let sphere = unit_sphere();
        let ray = Ray::new(
            Point3f::new(0.0, -5.0, 0.0),
            Vec3f::new(0.0, 1.0, 0.0),
            0.0,
            1.0,
        );

        let interaction = sphere.test(&ray);
        assert_eq!(interaction, None);
    }
}

///////////////
// END TESTS //
///////////////
