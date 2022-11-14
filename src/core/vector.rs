use num_traits::float::Float;
use std::ops::{Add, Div, Mul, Sub};

/////////////////////
// BEGIN INTERFACE //
/////////////////////

/// A geometric three-dimensional vector
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

//////////////////////////
// END INTERFACE        //
// BEGIN IMPLEMENTATION //
//////////////////////////

impl<T> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub fn into<U>(self) -> Vec3<U>
    where T: Into<U>,
    {
        Vec3 {
            x: self.x.into(),
            y: self.y.into(),
            z: self.z.into(),
        }
    }
}

pub type Vec3f = Vec3<f32>;
pub type Vec3i = Vec3<i32>;

pub type Point3f = Vec3f;
pub type Point3i = Vec3i;

impl<T: Add<Output = T>> Add for Vec3<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Vec3<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

// Hadamard product
impl<T: Mul<Output = T>> Mul for Vec3<T> {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

// Inverse Hadamard product
impl<T: Div<Output = T>> Div for Vec3<T> {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl<T: Mul<Output = T> + Add<Output = T> + Copy> Vec3<T> {
    pub fn dot(&self, rhs: &Self) -> T {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn length_sq(&self) -> T {
        self.dot(self)
    }
}

impl<T: Float> Vec3<T> {
    pub fn length(&self) -> T {
        self.length_sq().sqrt()
    }
}

impl<T: Mul<Output = T> + Sub<Output = T> + Copy> Vec3<T> {
    pub fn cross(&self, rhs: &Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
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

    #[test]
    fn new() {
        let vec = Vec3::new(1, 2, 3);
        assert_eq!(vec, Vec3 { x: 1, y: 2, z: 3 });
    }

    #[test]
    fn add() {
        let vec1 = Vec3::new(1, 2, 3);
        let vec2 = Vec3::new(3, 2, 1);
        assert_eq!(vec1 + vec2, Vec3::new(4, 4, 4));
    }

    #[test]
    fn sub() {
        let vec1 = Vec3::new(1, 2, 3);
        let vec2 = Vec3::new(3, 2, 1);
        assert_eq!(vec1 - vec2, Vec3::new(-2, 0, 2));
    }

    #[test]
    fn hadamard() {
        let vec1 = Vec3::new(1, 2, 3);
        let vec2 = Vec3::new(3, 2, 1);
        assert_eq!(vec1 * vec2, Vec3::new(3, 4, 3));
    }

    #[test]
    fn inverse_hadamard() {
        let vec1 = Vec3::new(1, 2, 3);
        let vec2 = Vec3::new(3, 2, 1);
        assert_eq!(vec1 / vec2, Vec3::new(0, 1, 3));
    }

    #[test]
    fn dot() {
        let vec1 = Vec3::new(1, 2, 3);
        let vec2 = Vec3::new(3, 2, 1);
        assert_eq!(vec1.dot(&vec2), 10);
    }

    #[test]
    fn cross() {
        let vec1 = Vec3::new(1, 2, 3);
        let vec2 = Vec3::new(3, 2, 1);
        assert_eq!(vec1.cross(&vec2), Vec3::new(-4, 8, -4));
    }

    #[test]
    fn lengths() {
        let vec_i = Vec3::new(1, 2, 3);
        assert_eq!(vec_i.length_sq(), 14);

        let vec_f = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(vec_f.length(), vec_f.length_sq().sqrt());
    }
}

///////////////
// END TESTS //
///////////////
