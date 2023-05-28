use crate::core::vector::Color3f;

/////////////////////
// BEGIN INTERFACE //
/////////////////////

pub struct Sample {
    /// The x position of the sample [0.0, 1.0]
    pub x: f64,

    /// The y position of the sample [0.0, 1.0]
    pub y: f64,

    /// The color of the sample, each element [0.0, 1.0]
    pub color: Color3f,
}

//////////////////////////
// END INTERFACE        //
// BEGIN IMPLEMENTATION //
//////////////////////////

impl Sample {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            x,
            y,
            color: Color3f::default(),
        }
    }

    pub fn new_with_color(x: f64, y: f64, color: Color3f) -> Self {
        Self { x, y, color }
    }

    pub fn color_rgb_bytes(&self) -> [u8; 3] {
        [
            (self.color.x * 255.0) as u8,
            (self.color.y * 255.0) as u8,
            (self.color.z * 255.0) as u8,
        ]
    }
}

////////////////////////
// END IMPLEMENTATION //
// BEGIN TESTS        //
////////////////////////

///////////////
// END TESTS //
///////////////
