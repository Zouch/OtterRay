use std::fs::File;
use std::io::BufWriter;

use math::Color;

extern crate png;
use self::png::HasParameters;

pub struct Image {
    pub width: u32,
    pub height: u32,
    pub samples: u32,
    data: Vec<Color>,
}

impl Image {
    pub fn new(width: u32, height: u32, samples: u32) -> Image {
        let mut result = Image {
            data: Vec::new(),
            width: width,
            height: height,
            samples: samples,
        };

        result.data.resize((width * height) as usize, Color::BLACK);

        return result;
    }

    fn get_writable_data(&self) -> Vec<u8> {
        let size = self.data.len() * 4;
        let mut result: Vec<u8> = Vec::with_capacity(size);

        for c in self.data.iter() {
            result.push((c.red * 255.0) as u8);
            result.push((c.green * 255.0) as u8);
            result.push((c.blue * 255.0) as u8);
            result.push(255);
        }

        return result;
    }

    pub fn write_png(&self, filepath: String) -> () {
        let file = File::create(filepath).unwrap();
        let ref mut bufwriter = BufWriter::new(file);

        let mut encoder = png::Encoder::new(bufwriter, self.width, self.height);
        encoder.set(png::ColorType::RGBA).set(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();

        let data = self.get_writable_data();
        writer.write_image_data(&data).unwrap();
    }

    pub fn set_pixel_color(&mut self, i: u32, j: u32, color: Color) {
        assert!(i < self.width && j < self.height);
        let pixel = (i + j * self.width) as usize;
        self.data[pixel] = color;
    }
}