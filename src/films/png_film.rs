/////////////////////
// BEGIN INTERFACE //
/////////////////////

use crate::core::{film::Film, sample::Sample, vector::Color3i};
use std::{fs::File, io::BufWriter, path::Path};

pub struct PngFilm {
    colors: Vec<Vec<Vec<Color3i>>>,
    width: u32,
    height: u32,
}

//////////////////////////
// END INTERFACE        //
// BEGIN IMPLEMENTATION //
//////////////////////////

impl PngFilm {
    pub fn new(width: u32, height: u32) -> Self {
        let mut colors = Vec::with_capacity(height as usize);

        for r in 0..height {
            colors.push(Vec::with_capacity(width as usize));
            for _ in 0..width {
                colors[r as usize].push(Vec::new());
            }
        }

        Self {
            colors,
            width,
            height,
        }
    }
}

impl Film for PngFilm {
    fn add_sample(&mut self, sample: &Sample) {
        let row = (sample.y * self.height as f64) as usize;
        let col = (sample.x * self.width as f64) as usize;

        self.colors[row][col].push(sample.color_rgb_bytes());
    }

    fn develop(&self) {
        let mut data = Vec::new();
        for row in self.colors.iter() {
            for colors in row.iter() {
                let mut aggregated = Color3i::new(0, 0, 0);
                for color in colors.iter() {
                    aggregated += color
                }
                aggregated /= colors.len() as i32;

                data.push(aggregated.x as u8);
                data.push(aggregated.y as u8);
                data.push(aggregated.z as u8);
            }
        }

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
        writer.write_image_data(&data).unwrap();
    }
}

////////////////////////
// END IMPLEMENTATION //
// BEGIN TESTS        //
////////////////////////

///////////////
// END TESTS //
///////////////
