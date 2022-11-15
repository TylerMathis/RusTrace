use crate::core::vector::Color3f;

/////////////////////
// BEGIN INTERFACE //
/////////////////////

pub struct Sample {
    /// The x position of the sample [0.0, 1.0]
    x: f64,

    /// The y position of the sample [0.0, 1.0]
    y: f64,

    /// The color of the sample, each element [0.0, 1.0]
    color: Color3f,
}

//////////////////////////
// END INTERFACE        //
// BEGIN IMPLEMENTATION //
//////////////////////////

impl Sample {
    pub fn new(x: f64, y: f64, color: Color3f) -> Self {
        Self { x, y, color }
    }
}

////////////////////////
// END IMPLEMENTATION //
// BEGIN TESTS        //
////////////////////////

///////////////
// END TESTS //
///////////////
