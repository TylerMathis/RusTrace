/////////////////////
// BEGIN INTERFACE //
/////////////////////

use crate::core::{film::Film, sample::Sample};
use std::{fs::File, io::BufWriter, path::Path};

struct PngFilm {
    data: Vec<u8>,
    width: u32,
    height: u32,
}

//////////////////////////
// END INTERFACE        //
// BEGIN IMPLEMENTATION //
//////////////////////////

impl PngFilm {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            data: Vec::new(),
            width,
            height,
        }
    }
}

impl Film for PngFilm {
    fn add_sample(&mut self, sample: &Sample) {
        // TODO: Properly handle
        if self.data.len() as u32 / 3 >= self.width * self.height {
            panic!("Attempted to add more samples than pixels in PNG");
        }

        for byte in sample.color_rgb_bytes() {
            self.data.push(byte);
        }
    }

    fn develop(&self) {
        let path = Path::new(r"/Users/tylerhm/Pictures/result.png");
        let file = File::create(path).unwrap();
        let ref mut w = BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, self.width, self.height);
        encoder.set_color(png::ColorType::Rgb);
        encoder.set_depth(png::BitDepth::Eight);
        let source_chromaticities = png::SourceChromaticities::new(
            (0.31270, 0.32900),
            (0.64000, 0.33000),
            (0.30000, 0.60000),
            (0.15000, 0.06000),
        );
        encoder.set_source_chromaticities(source_chromaticities);

        let mut writer = encoder.write_header().unwrap();
        writer.write_image_data(&self.data).unwrap();
    }
}

////////////////////////
// END IMPLEMENTATION //
// BEGIN TESTS        //
////////////////////////

///////////////
// END TESTS //
///////////////
