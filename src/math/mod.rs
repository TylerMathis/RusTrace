use std::cmp::Ordering;

pub enum QuadraticSolution {
    None,
    One { x: f64 },
    Two { x1: f64, x2: f64 },
}

pub fn solve_quadratic(a: f64, b: f64, c: f64) -> Option<QuadraticSolution> {
    let discriminant = b * b - 4.0 * a * c;

    Some(match discriminant.partial_cmp(&0.0)? {
        Ordering::Less => QuadraticSolution::None,
        Ordering::Equal => QuadraticSolution::One {
            x: (-b - discriminant.sqrt()) / (2.0 * a),
        },
        Ordering::Greater => QuadraticSolution::Two {
            x1: (-b - discriminant.sqrt()) / (2.0 * a),
            x2: (-b + discriminant.sqrt()) / (2.0 * a),
        },
    })
}
